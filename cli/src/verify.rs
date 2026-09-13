//! `ptc verify` -- read-only integrity check of a sealed capsule.
//! Recomputes every file's SHA-256 and compares it against
//! `manifest-sha256.txt` (payload) and `tagmanifest-sha256.txt` (tag
//! files), the way `sha256sum -c` would -- exactly what `STANDARDS.md`
//! describes BagIt's manifests as being for, and the "structural, not
//! full JSON Schema, validation" gap `cli/README.md` has flagged since
//! `ptc init`/`ptc seal` first shipped names this as the eventual
//! authoritative check.
//!
//! **Encrypted files are not a failure.** `ENCRYPTION.md` is explicit that
//! once `ptc encrypt` replaces a payload file's plaintext with ciphertext
//! (renamed with `.enc`), its bytes no longer match the manifest --
//! "expected, not a bug." So for each manifest entry, `ptc verify` checks
//! for the plaintext file first; if that's gone but `<path>.enc` exists
//! instead, it's reported as encrypted-and-not-independently-verifiable,
//! not as a mismatch or a missing file. Only a file that's neither present
//! nor encrypted counts as missing.
//!
//! Also flags any file under `data/` the manifest never recorded at all --
//! material added post-seal without a legitimate reseal, or possible
//! tampering -- since a manifest that only checks what it already knows
//! about would miss exactly that.
//!
//! ROADMAP.md item 27.

use crate::checksum::{parse_manifest_line, sha256_file};
use anyhow::{bail, Context, Result};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub struct VerifyReport {
    pub payload_checked: usize,
    pub payload_ok: usize,
    pub payload_encrypted: Vec<String>,
    pub payload_missing: Vec<String>,
    pub payload_mismatched: Vec<String>,
    pub tag_checked: usize,
    pub tag_ok: usize,
    pub tag_missing: Vec<String>,
    pub tag_mismatched: Vec<String>,
    pub unexpected_files: Vec<String>,
}

impl VerifyReport {
    /// `false` if there's anything a creator/custodian/auditor should
    /// actually worry about. Encrypted files don't count against this --
    /// see the module doc comment.
    pub fn passed(&self) -> bool {
        self.payload_missing.is_empty()
            && self.payload_mismatched.is_empty()
            && self.tag_missing.is_empty()
            && self.tag_mismatched.is_empty()
            && self.unexpected_files.is_empty()
    }
}

pub fn verify(root: &Path) -> Result<VerifyReport> {
    let manifest_path = root.join("manifest-sha256.txt");
    let tagmanifest_path = root.join("tagmanifest-sha256.txt");
    if !manifest_path.is_file() || !tagmanifest_path.is_file() {
        bail!(
            "{} and/or {} don't exist -- this capsule hasn't been sealed yet (`ptc seal` first)",
            manifest_path.display(),
            tagmanifest_path.display()
        );
    }

    let manifest_entries = read_manifest(&manifest_path)?;
    let tag_entries = read_manifest(&tagmanifest_path)?;

    let mut payload_ok = 0;
    let mut payload_encrypted = Vec::new();
    let mut payload_missing = Vec::new();
    let mut payload_mismatched = Vec::new();
    let mut recorded_paths: BTreeSet<String> = BTreeSet::new();

    for (expected_hash, rel_path) in &manifest_entries {
        recorded_paths.insert(rel_path.clone());
        let abs = root.join(rel_path);

        if abs.is_file() {
            let actual_hash = sha256_file(&abs)?;
            if &actual_hash == expected_hash {
                payload_ok += 1;
            } else {
                payload_mismatched.push(rel_path.clone());
            }
            continue;
        }

        let enc_path = root.join(format!("{rel_path}.enc"));
        if enc_path.is_file() {
            payload_encrypted.push(rel_path.clone());
        } else {
            payload_missing.push(rel_path.clone());
        }
    }

    let mut tag_ok = 0;
    let mut tag_missing = Vec::new();
    let mut tag_mismatched = Vec::new();
    for (expected_hash, rel_path) in &tag_entries {
        let abs = root.join(rel_path);
        if !abs.is_file() {
            tag_missing.push(rel_path.clone());
            continue;
        }
        let actual_hash = sha256_file(&abs)?;
        if &actual_hash == expected_hash {
            tag_ok += 1;
        } else {
            tag_mismatched.push(rel_path.clone());
        }
    }

    let unexpected_files = find_unexpected_files(root, &recorded_paths)?;

    Ok(VerifyReport {
        payload_checked: manifest_entries.len(),
        payload_ok,
        payload_encrypted,
        payload_missing,
        payload_mismatched,
        tag_checked: tag_entries.len(),
        tag_ok,
        tag_missing,
        tag_mismatched,
        unexpected_files,
    })
}

fn read_manifest(path: &Path) -> Result<Vec<(String, String)>> {
    let text = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut entries = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let entry = parse_manifest_line(line)
            .with_context(|| format!("{}:{}: malformed line: {line:?}", path.display(), i + 1))?;
        entries.push(entry);
    }
    Ok(entries)
}

/// Files under `data/` that `manifest-sha256.txt` never recorded, checked
/// against the *base* name (an `.enc` suffix stripped) so a properly
/// encrypted file -- already accounted for above -- isn't flagged twice.
fn find_unexpected_files(root: &Path, recorded_paths: &BTreeSet<String>) -> Result<Vec<String>> {
    let data_dir = root.join("data");
    if !data_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut unexpected = Vec::new();
    for entry in WalkDir::new(&data_dir) {
        let entry = entry.with_context(|| format!("walking {}", data_dir.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }
        let rel = entry
            .path()
            .strip_prefix(root)
            .with_context(|| format!("computing a path relative to {}", root.display()))?
            .to_string_lossy()
            .replace('\\', "/");
        let base = rel.strip_suffix(".enc").unwrap_or(&rel);
        if !recorded_paths.contains(base) {
            unexpected.push(rel);
        }
    }
    unexpected.sort();
    Ok(unexpected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        Access, Capsule, CapsuleIdentity, Integrity, LifecycleStage, ManifestAlgorithm,
        PrivacyZone, ReleasePolicy, ReleasePolicyType, Status, SPEC_VERSION,
    };
    use crate::seal;

    /// Build and seal a small, real capsule -- exercising actual
    /// `ptc seal` rather than hand-typing manifest lines, so these tests
    /// verify against genuinely produced fixtures.
    fn sealed_capsule(root: &Path) {
        let capsule = Capsule {
            capsule: CapsuleIdentity {
                spec_version: SPEC_VERSION.to_string(),
                id: "urn:uuid:00000000-0000-0000-0000-000000000000".to_string(),
                created: "2026-08-30".to_string(),
                creator: None,
            },
            purpose: None,
            series: None,
            collections: None,
            access: Access {
                default_privacy_zone: PrivacyZone::Restricted,
                default_release_policy: ReleasePolicy {
                    policy_type: ReleasePolicyType::Immediate,
                    date: None,
                    years: None,
                    description: None,
                },
            },
            custody: None,
            encryption: None,
            integrity: Integrity {
                manifest_algorithm: ManifestAlgorithm::Sha256,
            },
            status: Status {
                lifecycle_stage: LifecycleStage::Screening,
            },
        };
        crate::model::write_capsule_yaml(&root.join("capsule.yaml"), &capsule, "").unwrap();
        fs::create_dir_all(root.join("data").join("writing")).unwrap();
        fs::write(
            root.join("data").join("writing").join("note.txt"),
            b"hello capsule",
        )
        .unwrap();
        seal::seal(root).unwrap();
    }

    #[test]
    fn passes_on_a_freshly_sealed_capsule() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        sealed_capsule(root);

        let report = verify(root).unwrap();
        assert!(report.passed());
        assert_eq!(report.payload_checked, 1);
        assert_eq!(report.payload_ok, 1);
        assert!(report.payload_mismatched.is_empty());
        assert!(report.payload_missing.is_empty());
        assert!(report.payload_encrypted.is_empty());
        assert!(report.tag_ok >= 3); // bagit.txt, bag-info.txt, capsule.yaml
        assert!(report.unexpected_files.is_empty());
    }

    #[test]
    fn detects_a_tampered_payload_file() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        sealed_capsule(root);

        fs::write(
            root.join("data").join("writing").join("note.txt"),
            b"tampered content",
        )
        .unwrap();

        let report = verify(root).unwrap();
        assert!(!report.passed());
        assert_eq!(report.payload_mismatched, vec!["data/writing/note.txt"]);
    }

    #[test]
    fn detects_a_missing_payload_file() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        sealed_capsule(root);

        fs::remove_file(root.join("data").join("writing").join("note.txt")).unwrap();

        let report = verify(root).unwrap();
        assert!(!report.passed());
        assert_eq!(report.payload_missing, vec!["data/writing/note.txt"]);
    }

    #[test]
    fn treats_an_encrypted_file_as_neither_missing_nor_mismatched() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        sealed_capsule(root);

        let plain = root.join("data").join("writing").join("note.txt");
        fs::write(&plain, b"totally different ciphertext-shaped bytes").unwrap();
        fs::rename(&plain, plain.with_extension("txt.enc")).unwrap();

        let report = verify(root).unwrap();
        assert!(report.passed());
        assert_eq!(report.payload_encrypted, vec!["data/writing/note.txt"]);
        assert!(report.payload_mismatched.is_empty());
        assert!(report.payload_missing.is_empty());
    }

    #[test]
    fn detects_an_unexpected_file_not_in_the_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        sealed_capsule(root);

        fs::create_dir_all(root.join("data").join("writing")).unwrap();
        fs::write(
            root.join("data").join("writing").join("snuck-in.txt"),
            b"added after sealing without ptc import or a reseal",
        )
        .unwrap();

        let report = verify(root).unwrap();
        assert!(!report.passed());
        assert_eq!(report.unexpected_files, vec!["data/writing/snuck-in.txt"]);
    }

    #[test]
    fn detects_a_tampered_tag_file() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        sealed_capsule(root);

        let mut text = fs::read_to_string(root.join("capsule.yaml")).unwrap();
        text.push_str("# a byte that was never sealed in\n");
        fs::write(root.join("capsule.yaml"), text).unwrap();

        let report = verify(root).unwrap();
        assert!(!report.passed());
        assert_eq!(report.tag_mismatched, vec!["capsule.yaml"]);
    }

    #[test]
    fn bails_when_not_sealed() {
        let dir = tempfile::tempdir().unwrap();
        assert!(verify(dir.path()).is_err());
    }
}
