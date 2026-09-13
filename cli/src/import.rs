//! `ptc import` -- LIFECYCLE.md stage 2. Copies raw records (a file or a
//! whole directory tree) into `data/<collection>/`, registers the
//! collection in `capsule.yaml` if it isn't already declared, and appends
//! an `import` event to `data/provenance/events.yaml` (METADATA.md,
//! ROADMAP.md item 10).
//!
//! ROADMAP.md item 22.

use crate::model::{self, LifecycleStage};
use anyhow::{bail, Context, Result};
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct ImportSummary {
    pub files_copied: usize,
    pub bytes_copied: u64,
    pub collection: String,
    pub collection_path: String,
    pub collection_registered: bool,
}

pub fn import(
    root: &Path,
    source: &Path,
    collection: &str,
    source_description: Option<&str>,
    agent: Option<&str>,
) -> Result<ImportSummary> {
    if collection.is_empty() || collection.contains(['/', '\\']) {
        bail!("--collection must be a plain name, not a path (got {collection:?})");
    }
    if !source.exists() {
        bail!("import source {} does not exist", source.display());
    }

    let capsule_yaml_path = root.join("capsule.yaml");
    let capsule = model::read_capsule_yaml(&capsule_yaml_path).with_context(|| {
        format!(
            "{} must exist and parse before importing",
            capsule_yaml_path.display()
        )
    })?;

    guard_against_post_seal_import(capsule.status.lifecycle_stage)?;

    let existing = capsule
        .collections
        .as_ref()
        .and_then(|cs| cs.iter().find(|c| c.name == collection));
    let collection_registered = existing.is_none();
    let collection_rel_path = match existing {
        Some(c) => c.path.clone(),
        None => format!("data/{collection}"),
    };
    let collection_dir = root.join(&collection_rel_path);

    guard_against_self_copy(source, &collection_dir)?;

    let (files_copied, bytes_copied) = copy_into_collection(source, &collection_dir)?;

    if collection_registered {
        model::patch_add_collection(&capsule_yaml_path, collection, &collection_rel_path)?;
    }

    if capsule.status.lifecycle_stage == LifecycleStage::Origination {
        model::patch_lifecycle_stage(&capsule_yaml_path, "import")?;
    }

    let agent = agent
        .map(str::to_string)
        .unwrap_or_else(|| format!("ptc CLI v{}", env!("CARGO_PKG_VERSION")));
    let source_description = source_description
        .map(str::to_string)
        .unwrap_or_else(|| format!("Copied from {}", source.display()));
    append_import_event(root, collection, &source_description, &agent)?;

    Ok(ImportSummary {
        files_copied,
        bytes_copied,
        collection: collection.to_string(),
        collection_path: collection_rel_path,
        collection_registered,
    })
}

/// The amend loop (LIFECYCLE.md stages 1-4) is only valid before sealing --
/// once a manifest exists, importing more files without re-sealing would
/// silently invalidate it rather than being caught as tampering.
fn guard_against_post_seal_import(stage: LifecycleStage) -> Result<()> {
    use LifecycleStage::*;
    match stage {
        Origination | Import | Annotation | Screening => Ok(()),
        _ => bail!(
            "capsule.yaml's status.lifecycle_stage is {stage:?}, past the pre-seal amend loop -- \
             importing now would silently invalidate an existing manifest instead of being caught \
             as tampering. Re-run `ptc seal` after importing if you need to add material this late."
        ),
    }
}

fn guard_against_self_copy(source: &Path, collection_dir: &Path) -> Result<()> {
    let source_abs =
        fs::canonicalize(source).with_context(|| format!("resolving {}", source.display()))?;
    if let Ok(dest_abs) = fs::canonicalize(collection_dir) {
        if dest_abs == source_abs || dest_abs.starts_with(&source_abs) {
            bail!(
                "import destination {} is inside (or is) the import source {} -- refusing to copy a directory into itself",
                collection_dir.display(),
                source.display()
            );
        }
    }
    Ok(())
}

/// Copy `source` (a file or a directory tree) into `collection_dir` -- see
/// `plan_copy` for how a directory's own name is preserved as a subfolder.
/// Refuses (without copying anything) if any destination file already
/// exists, rather than silently overwriting or leaving a half-finished
/// copy behind.
fn copy_into_collection(source: &Path, collection_dir: &Path) -> Result<(usize, u64)> {
    let pairs = plan_copy(source, collection_dir)?;

    let conflicts: Vec<&PathBuf> = pairs
        .iter()
        .map(|(_, dest)| dest)
        .filter(|dest| dest.exists())
        .collect();
    if !conflicts.is_empty() {
        let list = conflicts
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join("\n  ");
        bail!("refusing to overwrite existing file(s):\n  {list}");
    }

    let mut files_copied = 0usize;
    let mut bytes_copied = 0u64;
    for (src, dest) in &pairs {
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
        }
        bytes_copied += fs::copy(src, dest)
            .with_context(|| format!("copying {} to {}", src.display(), dest.display()))?;
        files_copied += 1;
    }

    Ok((files_copied, bytes_copied))
}

/// A directory source is nested under its own name inside the collection
/// (`data/photos/1945/img.jpg`, not a flattened `data/photos/img.jpg`) --
/// otherwise re-importing several same-named-file source directories into
/// one collection (e.g. a "1945" and a "1946" photo folder both going into
/// "photos") would silently collide or lose which folder a file came from.
/// A single-file source has no such structure to preserve and lands
/// directly under the collection.
fn plan_copy(source: &Path, collection_dir: &Path) -> Result<Vec<(PathBuf, PathBuf)>> {
    if source.is_file() {
        let file_name = source
            .file_name()
            .with_context(|| format!("{} has no file name", source.display()))?;
        return Ok(vec![(source.to_path_buf(), collection_dir.join(file_name))]);
    }

    let source_name = source
        .file_name()
        .with_context(|| format!("{} has no directory name", source.display()))?;
    let dest_root = collection_dir.join(source_name);

    let mut pairs = Vec::new();
    for entry in WalkDir::new(source) {
        let entry = entry.with_context(|| format!("walking {}", source.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }
        let rel = entry
            .path()
            .strip_prefix(source)
            .with_context(|| format!("computing a path relative to {}", source.display()))?;
        pairs.push((entry.path().to_path_buf(), dest_root.join(rel)));
    }
    Ok(pairs)
}

fn append_import_event(root: &Path, collection: &str, source: &str, agent: &str) -> Result<()> {
    let events_path = root.join("data").join("provenance").join("events.yaml");
    let date = Utc::now().format("%Y-%m-%d");
    let source_escaped = escape_yaml_double_quoted(source);
    let agent_escaped = escape_yaml_double_quoted(agent);

    let entry = format!(
        "- date: \"{date}\"\n  type: import\n  agent: \"{agent_escaped}\"\n  collection: {collection}\n  source: \"{source_escaped}\"\n"
    );

    if events_path.exists() {
        let mut existing = fs::read_to_string(&events_path)
            .with_context(|| format!("reading {}", events_path.display()))?;
        if !existing.ends_with('\n') {
            existing.push('\n');
        }
        existing.push('\n');
        existing.push_str(&entry);
        fs::write(&events_path, existing)
            .with_context(|| format!("writing {}", events_path.display()))
    } else {
        if let Some(parent) = events_path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
        }
        let header = "# data/provenance/events.yaml -- see METADATA.md for the field reference:\n\
                      # https://github.com/neodem/personal-time-capsule/blob/main/METADATA.md\n\n";
        let contents = format!("{header}{entry}");
        fs::write(&events_path, contents)
            .with_context(|| format!("writing {}", events_path.display()))
    }
}

/// Escape `"` and `\` for embedding in a double-quoted YAML scalar. The
/// free-text `agent`/`source` fields are attacker-controllable-ish (a
/// creator's own free-text input) so this can't assume they're quote-free.
fn escape_yaml_double_quoted(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        Access, Capsule, CapsuleIdentity, Integrity, ManifestAlgorithm, PrivacyZone, ReleasePolicy,
        ReleasePolicyType, Status, SPEC_VERSION,
    };

    fn write_test_capsule(root: &Path, stage: LifecycleStage) {
        let capsule = Capsule {
            capsule: CapsuleIdentity {
                spec_version: SPEC_VERSION.to_string(),
                id: "urn:uuid:00000000-0000-0000-0000-000000000000".to_string(),
                created: "2026-08-30".to_string(),
                creator: None,
            },
            purpose: None,
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
                lifecycle_stage: stage,
            },
        };
        model::write_capsule_yaml(&root.join("capsule.yaml"), &capsule, "").unwrap();
    }

    #[test]
    fn imports_a_single_file_and_registers_collection() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, LifecycleStage::Origination);

        let source_dir = dir.path().join("source");
        fs::create_dir_all(&source_dir).unwrap();
        let source_file = source_dir.join("photo.jpg");
        fs::write(&source_file, b"fake image bytes").unwrap();

        let summary = import(root, &source_file, "photos", None, None).unwrap();
        assert_eq!(summary.files_copied, 1);
        assert_eq!(summary.bytes_copied, 16);
        assert!(summary.collection_registered);

        let copied = root.join("data").join("photos").join("photo.jpg");
        assert!(copied.exists());

        let capsule_text = fs::read_to_string(root.join("capsule.yaml")).unwrap();
        assert!(capsule_text.contains("- name: photos"));
        assert!(capsule_text.contains("path: data/photos"));
        assert!(capsule_text.contains("lifecycle_stage: import"));

        let events_text =
            fs::read_to_string(root.join("data").join("provenance").join("events.yaml")).unwrap();
        assert!(events_text.contains("type: import"));
        assert!(events_text.contains("collection: photos"));
    }

    #[test]
    fn imports_a_directory_tree_preserving_structure() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, LifecycleStage::Origination);

        let source_dir = dir.path().join("export");
        fs::create_dir_all(source_dir.join("2020")).unwrap();
        fs::write(source_dir.join("2020").join("a.txt"), b"a").unwrap();
        fs::write(source_dir.join("b.txt"), b"bb").unwrap();

        let summary = import(root, &source_dir, "writing", None, None).unwrap();
        assert_eq!(summary.files_copied, 2);
        assert_eq!(summary.bytes_copied, 3);

        // The source directory's own name ("export") is preserved as a
        // subfolder, so re-importing a same-named-file "2021" folder later
        // wouldn't collide with this one.
        assert!(root
            .join("data")
            .join("writing")
            .join("export")
            .join("2020")
            .join("a.txt")
            .exists());
        assert!(root
            .join("data")
            .join("writing")
            .join("export")
            .join("b.txt")
            .exists());
    }

    #[test]
    fn imports_multiple_source_directories_into_one_collection_without_colliding() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, LifecycleStage::Origination);

        let dir_1945 = dir.path().join("1945");
        fs::create_dir_all(&dir_1945).unwrap();
        fs::write(dir_1945.join("img0001.jpg"), b"a").unwrap();

        let dir_1946 = dir.path().join("1946");
        fs::create_dir_all(&dir_1946).unwrap();
        fs::write(dir_1946.join("img0001.jpg"), b"bb").unwrap();

        import(root, &dir_1945, "photos", None, None).unwrap();
        import(root, &dir_1946, "photos", None, None).unwrap();

        assert!(root
            .join("data")
            .join("photos")
            .join("1945")
            .join("img0001.jpg")
            .exists());
        assert!(root
            .join("data")
            .join("photos")
            .join("1946")
            .join("img0001.jpg")
            .exists());
    }

    #[test]
    fn refuses_to_overwrite_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, LifecycleStage::Origination);

        let source_dir = dir.path().join("source");
        fs::create_dir_all(&source_dir).unwrap();
        fs::write(source_dir.join("photo.jpg"), b"new content").unwrap();

        // Pre-existing file at the path this import will actually land on
        // -- nested under the source directory's own name.
        let conflicting = root
            .join("data")
            .join("photos")
            .join("source")
            .join("photo.jpg");
        fs::create_dir_all(conflicting.parent().unwrap()).unwrap();
        fs::write(&conflicting, b"already here").unwrap();

        let result = import(root, &source_dir, "photos", None, None);
        assert!(result.is_err());
        // Original content must be untouched.
        assert_eq!(fs::read(&conflicting).unwrap(), b"already here");
    }

    #[test]
    fn refuses_to_import_after_sealing() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, LifecycleStage::Sealing);

        let source_file = dir.path().join("f.txt");
        fs::write(&source_file, b"x").unwrap();

        let result = import(root, &source_file, "writing", None, None);
        assert!(result.is_err());
        assert!(!root.join("data").join("writing").exists());
    }

    #[test]
    fn does_not_reregister_an_already_declared_collection() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let capsule_path = root.join("capsule.yaml");
        write_test_capsule(root, LifecycleStage::Import);
        // Hand-append a pre-existing "photos" collection with a
        // non-default path, and a comment, to prove both the "already
        // registered" check and comment-preservation.
        let mut text = fs::read_to_string(&capsule_path).unwrap();
        text = text.replace(
            "access:",
            "collections:\n  # a hand-written comment\n  - name: photos\n    path: data/photos\naccess:",
        );
        fs::write(&capsule_path, &text).unwrap();

        let source_file = dir.path().join("photo.jpg");
        fs::write(&source_file, b"x").unwrap();

        let summary = import(root, &source_file, "photos", None, None).unwrap();
        assert!(!summary.collection_registered);

        let capsule_text = fs::read_to_string(&capsule_path).unwrap();
        assert_eq!(capsule_text.matches("- name: photos").count(), 1);
        assert!(capsule_text.contains("# a hand-written comment"));
    }
}
