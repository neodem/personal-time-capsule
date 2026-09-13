//! `ptc unlock` -- LIFECYCLE.md stage 10, the inverse of `ptc encrypt`.
//! Given one unit's key (a named collection, or `data/person`/
//! `data/provenance`), decrypts every `.enc` file back to plaintext and
//! removes the ciphertext.
//!
//! **Takes the raw key directly, not a SLIP-0039 quorum of shares.**
//! `ptc encrypt` never implemented SLIP-0039 splitting (no solid Rust
//! crate exists -- see `cipher.rs`/`encrypt.rs`), so it hands the whole
//! key to the creator directly; `ptc unlock` is this command's mirror
//! image, taking that same whole key back in. There's no quorum
//! reconstruction step here to build, because there was never a split to
//! reverse.
//!
//! **Validates before writing anything.** Every `.enc` file in the unit is
//! decrypted into memory first; only once *all* of them succeed does
//! `--apply` write the plaintexts and remove the ciphertexts. A wrong key
//! (or a key for the wrong unit) fails on the very first file -- since one
//! key belongs to exactly one unit -- with nothing on disk touched at
//! all, rather than leaving the unit in a half-unlocked state. Dry run
//! (the default) stops there regardless: it's a legitimate way to check
//! "is this the right key for this collection?" without committing to
//! anything.
//!
//! Logs an `unlock` event to `preservation-log.yaml` on `--apply` --
//! unlock is `LIFECYCLE.md` stage 10, long after sealing, so like
//! `ptc encrypt` it never touches `capsule.yaml` itself (frozen forever
//! once sealed -- see `INTEGRITY.md` and `encrypt.rs`'s doc comment).
//!
//! ROADMAP.md item 28.

use crate::cipher;
use crate::model::{self, Capsule, EventFields, EventLog, LifecycleStage};
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct UnlockSummary {
    pub name: String,
    pub path: String,
    pub files_unlocked: usize,
    pub applied: bool,
}

pub fn unlock(root: &Path, unit_name: &str, key_hex: &str, apply: bool) -> Result<UnlockSummary> {
    let capsule_yaml_path = root.join("capsule.yaml");
    let capsule = model::read_capsule_yaml(&capsule_yaml_path).with_context(|| {
        format!(
            "{} must exist and parse -- is {} a capsule directory?",
            capsule_yaml_path.display(),
            root.display()
        )
    })?;

    if capsule.status.lifecycle_stage != LifecycleStage::Sealing {
        bail!(
            "capsule.yaml's status.lifecycle_stage is {:?}, not sealing -- ptc unlock only runs \
             on a sealed capsule. Note: capsule.yaml is frozen forever once sealed (INTEGRITY.md), \
             so this field can never show anything past \"sealing\" even long after real \
             encryption/unlock/etc. happen -- check preservation-log.yaml's most recent entry for \
             a capsule's actual current stage.",
            capsule.status.lifecycle_stage
        );
    }

    let cipher = cipher::default_cipher();
    let key = cipher::from_hex(key_hex).context("parsing --key")?;
    if key.len() != cipher.key_len() {
        bail!(
            "key must be {} bytes ({} hex characters), got {} bytes",
            cipher.key_len(),
            cipher.key_len() * 2,
            key.len()
        );
    }

    let rel_path = resolve_unit_path(&capsule, unit_name);
    let dir = root.join(&rel_path);
    if !dir.is_dir() {
        bail!(
            "{} has no directory at {} -- nothing to unlock",
            unit_name,
            dir.display()
        );
    }

    let enc_files = find_enc_files(&dir)?;
    if enc_files.is_empty() {
        return Ok(UnlockSummary {
            name: unit_name.to_string(),
            path: rel_path,
            files_unlocked: 0,
            applied: false,
        });
    }

    // Decrypt everything into memory before writing anything -- a wrong
    // key fails here, on the first file, with nothing on disk touched.
    let mut decrypted: Vec<(PathBuf, PathBuf, Vec<u8>)> = Vec::with_capacity(enc_files.len());
    for enc_path in &enc_files {
        let ciphertext =
            fs::read(enc_path).with_context(|| format!("reading {}", enc_path.display()))?;
        let plaintext = cipher
            .decrypt(&key, &ciphertext)
            .with_context(|| format!("decrypting {}", enc_path.display()))?;
        let plain_path = strip_enc_extension(enc_path);
        decrypted.push((enc_path.clone(), plain_path, plaintext));
    }

    if !apply {
        return Ok(UnlockSummary {
            name: unit_name.to_string(),
            path: rel_path,
            files_unlocked: decrypted.len(),
            applied: false,
        });
    }

    for (enc_path, plain_path, plaintext) in &decrypted {
        fs::write(plain_path, plaintext)
            .with_context(|| format!("writing {}", plain_path.display()))?;
        fs::remove_file(enc_path)
            .with_context(|| format!("removing ciphertext {}", enc_path.display()))?;
    }

    let is_declared_collection = capsule
        .collections
        .iter()
        .flatten()
        .any(|c| c.name == unit_name);
    let note = format!(
        "{rel_path}: {} file(s) decrypted with {}",
        decrypted.len(),
        cipher.algorithm_id()
    );
    model::append_event(
        root,
        EventLog::Preservation,
        &EventFields {
            event_type: "unlock",
            agent: &format!("ptc CLI v{}", env!("CARGO_PKG_VERSION")),
            collection: is_declared_collection.then_some(unit_name),
            path: None,
            source: None,
            outcome: None,
            note: Some(&note),
        },
    )?;

    Ok(UnlockSummary {
        name: unit_name.to_string(),
        path: rel_path,
        files_unlocked: decrypted.len(),
        applied: true,
    })
}

/// Mirrors `encrypt.rs`'s own per-candidate resolution: a declared
/// collection's own path, or the `data/person`/`data/provenance`
/// convention, or `data/<name>` as a last resort for anything else.
fn resolve_unit_path(capsule: &Capsule, unit_name: &str) -> String {
    if let Some(c) = capsule
        .collections
        .iter()
        .flatten()
        .find(|c| c.name == unit_name)
    {
        return c.path.clone();
    }
    match unit_name {
        "person" => "data/person".to_string(),
        "provenance" => "data/provenance".to_string(),
        _ => format!("data/{unit_name}"),
    }
}

fn find_enc_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(dir) {
        let entry = entry.with_context(|| format!("walking {}", dir.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().is_some_and(|e| e == "enc") {
            files.push(entry.path().to_path_buf());
        }
    }
    Ok(files)
}

fn strip_enc_extension(path: &Path) -> PathBuf {
    path.with_extension("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encrypt;
    use crate::model::{
        Access, CapsuleIdentity, Collection, Integrity, ManifestAlgorithm, PrivacyZone,
        ReleasePolicy, ReleasePolicyType, Status, SPEC_VERSION,
    };

    fn write_test_capsule(root: &Path, collections: Option<Vec<Collection>>) {
        let capsule = Capsule {
            capsule: CapsuleIdentity {
                spec_version: SPEC_VERSION.to_string(),
                id: "urn:uuid:00000000-0000-0000-0000-000000000000".to_string(),
                created: "2026-08-30".to_string(),
                creator: None,
            },
            purpose: None,
            series: None,
            collections,
            access: Access {
                default_privacy_zone: PrivacyZone::Restricted,
                default_release_policy: ReleasePolicy {
                    policy_type: ReleasePolicyType::AfterDeath,
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
                lifecycle_stage: LifecycleStage::Sealing,
            },
        };
        model::write_capsule_yaml(&root.join("capsule.yaml"), &capsule, "").unwrap();
    }

    /// Build a small capsule with one collection, encrypt it via the real
    /// `ptc encrypt`, and return the key it issued plus the original bytes.
    fn encrypted_email_collection(root: &Path) -> (String, Vec<u8>) {
        write_test_capsule(
            root,
            Some(vec![Collection {
                name: "email".to_string(),
                path: "data/email".to_string(),
                privacy_zone: None,
                release_policy: None,
                encryption: None,
            }]),
        );
        fs::create_dir_all(root.join("data").join("email")).unwrap();
        let original = b"Dear diary, this is private.".to_vec();
        fs::write(root.join("data").join("email").join("msg.eml"), &original).unwrap();

        let summary = encrypt::encrypt(root, true).unwrap();
        let unit = summary.units.iter().find(|u| u.name == "email").unwrap();
        (unit.key_hex.clone().unwrap(), original)
    }

    #[test]
    fn dry_run_validates_key_without_writing_anything() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let (key_hex, _original) = encrypted_email_collection(root);

        let summary = unlock(root, "email", &key_hex, false).unwrap();
        assert_eq!(summary.files_unlocked, 1);
        assert!(!summary.applied);

        assert!(root
            .join("data")
            .join("email")
            .join("msg.eml.enc")
            .is_file());
        assert!(!root.join("data").join("email").join("msg.eml").exists());
        // ptc encrypt already wrote this file for its own "encryption"
        // event; the dry run must not have added an "unlock" entry to it.
        let log = fs::read_to_string(root.join("preservation-log.yaml")).unwrap();
        assert!(!log.contains("type: unlock"));
    }

    #[test]
    fn apply_restores_plaintext_and_logs_an_unlock_event() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let (key_hex, original) = encrypted_email_collection(root);

        let summary = unlock(root, "email", &key_hex, true).unwrap();
        assert_eq!(summary.files_unlocked, 1);
        assert!(summary.applied);

        let restored = fs::read(root.join("data").join("email").join("msg.eml")).unwrap();
        assert_eq!(restored, original);
        assert!(!root.join("data").join("email").join("msg.eml.enc").exists());

        let log = fs::read_to_string(root.join("preservation-log.yaml")).unwrap();
        assert!(log.contains("type: unlock"));
        assert!(log.contains("collection: email"));
        assert!(log.contains("xchacha20-poly1305"));
    }

    #[test]
    fn wrong_key_fails_cleanly_without_touching_anything() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let (_key_hex, _original) = encrypted_email_collection(root);

        let wrong_key = cipher::to_hex(&cipher::default_cipher().generate_key());
        let result = unlock(root, "email", &wrong_key, true);
        assert!(result.is_err());

        assert!(root
            .join("data")
            .join("email")
            .join("msg.eml.enc")
            .is_file());
        assert!(!root.join("data").join("email").join("msg.eml").exists());
        // Same as above: preservation-log.yaml already exists from
        // encrypt's own event; a failed unlock must not have added to it.
        let log = fs::read_to_string(root.join("preservation-log.yaml")).unwrap();
        assert!(!log.contains("type: unlock"));
    }

    #[test]
    fn no_encrypted_files_is_a_safe_noop() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(
            root,
            Some(vec![Collection {
                name: "email".to_string(),
                path: "data/email".to_string(),
                privacy_zone: None,
                release_policy: None,
                encryption: None,
            }]),
        );
        fs::create_dir_all(root.join("data").join("email")).unwrap();
        fs::write(root.join("data").join("email").join("msg.eml"), b"plain").unwrap();

        let key_hex = cipher::to_hex(&cipher::default_cipher().generate_key());
        let summary = unlock(root, "email", &key_hex, true).unwrap();
        assert_eq!(summary.files_unlocked, 0);
        assert!(!summary.applied);
    }

    #[test]
    fn refuses_an_unknown_unit_directory() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, None);

        let key_hex = cipher::to_hex(&cipher::default_cipher().generate_key());
        assert!(unlock(root, "nonexistent", &key_hex, true).is_err());
    }
}
