//! End-to-end: `ptc init` -> replace the placeholder release policy ->
//! `ptc seal`, then independently recompute checksums rather than trusting
//! the code under test to grade its own homework. Runs the actual compiled
//! `ptc` binary (not just internal function calls), since the binary's
//! behavior -- not the library's -- is this project's real contract; see
//! cli/README.md's "Stability" section.

use ptc::model::{self, ReleasePolicy, ReleasePolicyType};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::Command;

fn run_ptc(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ptc"))
        .args(args)
        .output()
        .expect("failed to run the ptc binary")
}

fn sha256_hex(path: &Path) -> String {
    let mut file = fs::File::open(path).expect("open payload file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("read payload file");
    format!("{:x}", Sha256::digest(&buf))
}

/// Recompute every line of a manifest file against the real bytes on disk
/// right now -- the same thing `sha256sum -c` does, and exactly what would
/// have caught a real bug found while building this: `ptc seal` originally
/// hashed capsule.yaml for tagmanifest-sha256.txt *before* patching its
/// `lifecycle_stage` field, so the recorded checksum described content the
/// file no longer had by the time `seal` finished. A test that only checks
/// a manifest *mentions* a filename can't catch that; only recomputing the
/// hash and comparing can.
fn assert_manifest_matches_disk(capsule_root: &Path, manifest_file: &str) {
    let manifest_path = capsule_root.join(manifest_file);
    let manifest = fs::read_to_string(&manifest_path)
        .unwrap_or_else(|e| panic!("reading {}: {e}", manifest_path.display()));
    assert!(
        !manifest.trim().is_empty(),
        "{manifest_file} is empty -- expected at least one entry"
    );
    for line in manifest.lines() {
        let (recorded_hash, rel_path) = line
            .split_once("  ")
            .unwrap_or_else(|| panic!("malformed {manifest_file} line: {line:?}"));
        let actual_hash = sha256_hex(&capsule_root.join(rel_path));
        assert_eq!(
            actual_hash, recorded_hash,
            "{manifest_file} entry for {rel_path} does not match its actual on-disk content"
        );
    }
}

#[test]
fn init_then_seal_produces_a_verifiable_manifest() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let capsule_path = tmp.path().join("capsule");
    let capsule_path_str = capsule_path.to_str().expect("utf8 tempdir path");

    let init_output = run_ptc(&[
        "init",
        capsule_path_str,
        "--creator-name",
        "Test Creator",
        "--purpose",
        "An integration-test capsule",
    ]);
    assert!(
        init_output.status.success(),
        "ptc init failed: {}",
        String::from_utf8_lossy(&init_output.stderr)
    );

    // ptc init should have scaffolded FORMAT.md's data/person/ layer, not
    // just the empty directory.
    let data_person = capsule_path.join("data").join("person");
    for file in ["biography.md", "timeline.md", "people.yaml", "places.yaml"] {
        assert!(
            data_person.join(file).is_file(),
            "ptc init should have written data/person/{file}"
        );
    }
    assert!(
        fs::read_to_string(data_person.join("biography.md"))
            .expect("read biography.md")
            .contains("Test Creator"),
        "biography.md's heading should include the --creator-name passed to ptc init"
    );

    let photos_dir = capsule_path.join("data").join("photos");
    fs::create_dir_all(&photos_dir).expect("mkdir data/photos");
    let payload_path = photos_dir.join("note.txt");
    fs::write(&payload_path, "hello from the integration test\n").expect("write payload file");

    // ptc init's placeholder release policy must be refused, not silently sealed.
    let premature_seal = run_ptc(&["seal", capsule_path_str]);
    assert!(
        !premature_seal.status.success(),
        "ptc seal should refuse the placeholder release policy ptc init wrote"
    );

    // Replace it with a real policy via the same model the CLI itself
    // uses, exercising the read/write round trip rather than string surgery.
    let capsule_yaml_path = capsule_path.join("capsule.yaml");
    let mut capsule = model::read_capsule_yaml(&capsule_yaml_path).expect("read capsule.yaml");
    capsule.access.default_release_policy = ReleasePolicy {
        policy_type: ReleasePolicyType::Immediate,
        date: None,
        years: None,
        description: None,
    };
    model::write_capsule_yaml(&capsule_yaml_path, &capsule, "").expect("rewrite capsule.yaml");

    let seal_output = run_ptc(&["seal", capsule_path_str]);
    assert!(
        seal_output.status.success(),
        "ptc seal failed: {}",
        String::from_utf8_lossy(&seal_output.stderr)
    );

    // Independently recompute every manifest-sha256.txt and
    // tagmanifest-sha256.txt entry against the real, final on-disk bytes --
    // not just that seal produced *some* line mentioning each file.
    // Critically, this must run AFTER `ptc seal` has fully finished, since
    // it checks the files' FINAL state (this is what actually caught the
    // capsule.yaml-hashed-before-being-patched ordering bug).
    assert_manifest_matches_disk(&capsule_path, "manifest-sha256.txt");
    assert_manifest_matches_disk(&capsule_path, "tagmanifest-sha256.txt");

    // Sanity-check the specific payload file is actually the one recorded.
    let manifest = fs::read_to_string(capsule_path.join("manifest-sha256.txt"))
        .expect("read manifest-sha256.txt");
    let expected_line = format!("{}  data/photos/note.txt", sha256_hex(&payload_path));
    assert!(
        manifest.lines().any(|line| line == expected_line),
        "manifest-sha256.txt missing expected line {expected_line:?}; got:\n{manifest}"
    );

    // No privacy-overrides.yaml or legal/ exist in this fixture, so the
    // tagmanifest must cover exactly the three tag files that do -- not more.
    let tagmanifest = fs::read_to_string(capsule_path.join("tagmanifest-sha256.txt"))
        .expect("read tagmanifest-sha256.txt");
    for expected_file in ["bagit.txt", "bag-info.txt", "capsule.yaml"] {
        assert!(
            tagmanifest.contains(expected_file),
            "tagmanifest-sha256.txt missing {expected_file}"
        );
    }
    assert!(!tagmanifest.contains("privacy-overrides.yaml"));
    assert!(!tagmanifest.contains("legal/"));

    let final_capsule_yaml = fs::read_to_string(&capsule_yaml_path).expect("re-read capsule.yaml");
    assert!(final_capsule_yaml.contains("lifecycle_stage: sealing"));

    // And, since FORMAT.md specifically warns about this exact footgun: the
    // date fields must still be quoted strings after ptc seal has touched
    // the file, not just right after ptc init.
    assert!(final_capsule_yaml.contains("created: \""));
}
