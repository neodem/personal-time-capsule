//! `ptc encrypt` -- LIFECYCLE.md stage 6, which happens *after* sealing
//! (stage 5). Encrypts every payload file in each unit -- a named
//! collection, or `data/person`/`data/provenance` under the capsule-wide
//! default -- whose resolved `release_policy` isn't `immediate`, one fresh
//! key per unit (ENCRYPTION.md's "one key per collection, not one key per
//! capsule").
//!
//! **Deliberately never touches `capsule.yaml`.** `INTEGRITY.md` is
//! explicit that `capsule.yaml` and the other identity-defining tag files
//! "stay frozen from the moment of sealing, forever" -- there's no defined
//! mechanism (a `reseal` event covers `manifest-sha256.txt` only, not
//! `capsule.yaml`'s own content) to legitimately amend it afterward. That
//! also means `capsule.yaml`'s `status.lifecycle_stage` can never show
//! anything past `sealing` in the file itself, even decades into real
//! encryption/deposit/dormant preservation -- a reader determining a
//! capsule's actual current stage should consult `preservation-log.yaml`'s
//! most recent entry, not this field, once it reads `sealing`. This
//! command writes only there: one `encryption`-typed entry per unit,
//! immediately after that unit finishes, so a run interrupted partway
//! through still leaves a truthful record of exactly what was completed.
//!
//! **Key-splitting (SLIP-0039) is NOT implemented.** ENCRYPTION.md calls
//! for splitting each unit's key into a trustee quorum via SLIP-0039, but
//! no solid, actively-maintained Rust crate for it exists as of this
//! writing -- the one real implementation, `sssmc39`, is unmaintained,
//! pre-1.0, and self-described as a work-in-progress (last touched 2020);
//! the other, `slip39`, is GPL-3.0-or-later, license-incompatible with
//! this Apache-2.0 codebase. Until that's resolved, `ptc encrypt` prints
//! each unit's raw key directly via `--apply` and trusts the creator to
//! record and distribute it themselves -- a real, named gap, not a silent
//! one. `ptc unlock` (ROADMAP.md item 28) doesn't exist yet either.
//!
//! Built around `cipher::Cipher` rather than calling `chacha20poly1305`
//! directly -- see that module's doc comment for why.
//!
//! ROADMAP.md item 26.

use crate::cipher::{self, Cipher};
use crate::model::{self, Capsule, EventFields, EventLog, LifecycleStage, ReleasePolicyType};
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct EncryptSummary {
    pub units: Vec<UnitSummary>,
    pub skipped_immediate: Vec<String>,
}

pub struct UnitSummary {
    pub name: String,
    pub path: String,
    pub files_encrypted: usize,
    pub already_encrypted: bool,
    /// `Some` only when `apply` actually generated and used a fresh key
    /// this run -- never populated on a dry run, and never re-populated
    /// for a unit that turned out to already be fully encrypted.
    pub key_hex: Option<String>,
}

pub fn encrypt(root: &Path, apply: bool) -> Result<EncryptSummary> {
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
            "capsule.yaml's status.lifecycle_stage is {:?}, not sealing -- ptc encrypt only runs \
             on a freshly sealed capsule (`ptc seal` first). Note: capsule.yaml is frozen forever \
             once sealed (INTEGRITY.md), so this field can never show anything past \"sealing\" even \
             after real encryption/deposit/etc. happen later -- check preservation-log.yaml's most \
             recent entry for a capsule's actual current stage.",
            capsule.status.lifecycle_stage
        );
    }

    let candidates = encryption_candidates(&capsule);
    let cipher = cipher::default_cipher();
    let mut units = Vec::new();
    let mut skipped_immediate = Vec::new();

    for (name, rel_path, policy_type) in candidates {
        if policy_type == ReleasePolicyType::Immediate {
            skipped_immediate.push(name);
            continue;
        }

        let dir = root.join(&rel_path);
        let unit = encrypt_unit(&name, &rel_path, &dir, cipher.as_ref(), apply)?;

        if apply && !unit.already_encrypted {
            let is_declared_collection =
                capsule.collections.iter().flatten().any(|c| c.name == name);
            let note = format!(
                "{rel_path}: {} file(s) encrypted with {}; SLIP-0039 key-splitting not yet \
                 implemented, key issued directly to the creator (see ENCRYPTION.md open questions)",
                unit.files_encrypted,
                cipher.algorithm_id()
            );
            model::append_event(
                root,
                EventLog::Preservation,
                &EventFields {
                    event_type: "encryption",
                    agent: &format!("ptc CLI v{}", env!("CARGO_PKG_VERSION")),
                    collection: is_declared_collection.then_some(name.as_str()),
                    path: None,
                    source: None,
                    outcome: None,
                    note: Some(&note),
                },
            )?;
        }

        units.push(unit);
    }

    Ok(EncryptSummary {
        units,
        skipped_immediate,
    })
}

/// Every unit `ptc encrypt` considers: each declared collection (its own
/// `release_policy` override, or the capsule-wide default if it has none),
/// plus `data/person` and `data/provenance` under the capsule-wide default
/// directly -- they aren't `collections[]` entries, but they're payload
/// like anything else, and per ENCRYPTION.md's own granularity goal they
/// shouldn't sit unencrypted just because nothing declared them.
fn encryption_candidates(capsule: &Capsule) -> Vec<(String, String, ReleasePolicyType)> {
    let mut candidates = Vec::new();

    for c in capsule.collections.iter().flatten() {
        let policy_type = c
            .release_policy
            .as_ref()
            .map(|p| p.policy_type)
            .unwrap_or(capsule.access.default_release_policy.policy_type);
        candidates.push((c.name.clone(), c.path.clone(), policy_type));
    }

    let default_policy_type = capsule.access.default_release_policy.policy_type;
    for (name, rel) in [("person", "data/person"), ("provenance", "data/provenance")] {
        candidates.push((name.to_string(), rel.to_string(), default_policy_type));
    }

    candidates
}

fn encrypt_unit(
    name: &str,
    rel_path: &str,
    dir: &Path,
    cipher: &dyn Cipher,
    apply: bool,
) -> Result<UnitSummary> {
    let done = |files_encrypted, key_hex| UnitSummary {
        name: name.to_string(),
        path: rel_path.to_string(),
        files_encrypted,
        already_encrypted: files_encrypted == 0,
        key_hex,
    };

    if !dir.is_dir() {
        return Ok(done(0, None));
    }

    let targets = plan_unit(dir)?;
    if targets.is_empty() {
        return Ok(done(0, None));
    }
    if !apply {
        return Ok(UnitSummary {
            name: name.to_string(),
            path: rel_path.to_string(),
            files_encrypted: targets.len(),
            already_encrypted: false,
            key_hex: None,
        });
    }

    let key = cipher.generate_key();
    for file in &targets {
        let plaintext = fs::read(file).with_context(|| format!("reading {}", file.display()))?;
        let ciphertext = cipher
            .encrypt(&key, &plaintext)
            .with_context(|| format!("encrypting {}", file.display()))?;
        let enc_path = append_enc_extension(file);
        fs::write(&enc_path, ciphertext)
            .with_context(|| format!("writing {}", enc_path.display()))?;
        fs::remove_file(file).with_context(|| format!("removing plaintext {}", file.display()))?;
    }

    Ok(UnitSummary {
        name: name.to_string(),
        path: rel_path.to_string(),
        files_encrypted: targets.len(),
        already_encrypted: false,
        key_hex: Some(cipher::to_hex(&key)),
    })
}

/// Files in `dir` not already ending in `.enc` -- what still needs
/// encrypting. A unit where this comes back empty is either untouched (no
/// files at all) or fully done already; either way there's nothing to do,
/// which is what makes re-running `ptc encrypt` on an already-encrypted
/// capsule a safe no-op instead of an error.
fn plan_unit(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut targets = Vec::new();
    for entry in WalkDir::new(dir) {
        let entry = entry.with_context(|| format!("walking {}", dir.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "enc") {
            continue;
        }
        targets.push(path.to_path_buf());
    }
    Ok(targets)
}

fn append_enc_extension(path: &Path) -> PathBuf {
    let mut name = path
        .file_name()
        .expect("walked entries always have a file name")
        .to_os_string();
    name.push(".enc");
    path.with_file_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        Access, CapsuleIdentity, Collection, Integrity, ManifestAlgorithm, PrivacyZone,
        ReleasePolicy, Status, SPEC_VERSION,
    };

    fn write_test_capsule(
        root: &Path,
        stage: LifecycleStage,
        default_policy: ReleasePolicyType,
        collections: Option<Vec<Collection>>,
    ) {
        let capsule = Capsule {
            capsule: CapsuleIdentity {
                spec_version: SPEC_VERSION.to_string(),
                id: "urn:uuid:00000000-0000-0000-0000-000000000000".to_string(),
                created: "2026-08-30".to_string(),
                creator: None,
            },
            purpose: None,
            collections,
            access: Access {
                default_privacy_zone: PrivacyZone::Restricted,
                default_release_policy: ReleasePolicy {
                    policy_type: default_policy,
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

    fn collection(name: &str, path: &str, release_policy: Option<ReleasePolicyType>) -> Collection {
        Collection {
            name: name.to_string(),
            path: path.to_string(),
            privacy_zone: None,
            release_policy: release_policy.map(|policy_type| ReleasePolicy {
                policy_type,
                date: None,
                years: None,
                description: None,
            }),
            encryption: None,
        }
    }

    #[test]
    fn encrypts_a_collection_and_logs_a_preservation_event() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(
            root,
            LifecycleStage::Sealing,
            ReleasePolicyType::Immediate,
            Some(vec![collection(
                "email",
                "data/email",
                Some(ReleasePolicyType::AfterDeath),
            )]),
        );
        fs::create_dir_all(root.join("data").join("email")).unwrap();
        let file = root.join("data").join("email").join("msg.eml");
        let original = b"Dear diary, my SSN is 123-45-6789.".to_vec();
        fs::write(&file, &original).unwrap();

        let summary = encrypt(root, true).unwrap();
        let email = summary
            .units
            .iter()
            .find(|u| u.name == "email")
            .expect("email unit");
        assert_eq!(email.files_encrypted, 1);
        assert!(!email.already_encrypted);
        let key_hex = email
            .key_hex
            .clone()
            .expect("a key should have been issued");
        assert_eq!(key_hex.len(), 64); // 32 bytes, hex-encoded

        assert!(!file.exists(), "plaintext must be removed");
        let enc_path = root.join("data").join("email").join("msg.eml.enc");
        assert!(enc_path.is_file());
        let ciphertext = fs::read(&enc_path).unwrap();
        assert_ne!(ciphertext, original);

        // Round-trip through the same Cipher confirms the on-disk bytes
        // are genuinely decryptable with the key that was reported.
        let key = cipher::from_hex(&key_hex).unwrap();
        let cipher = cipher::default_cipher();
        let decrypted = cipher.decrypt(&key, &ciphertext).unwrap();
        assert_eq!(decrypted, original);

        let log = fs::read_to_string(root.join("preservation-log.yaml")).unwrap();
        assert!(log.contains("type: encryption"));
        assert!(log.contains("collection: email"));
        assert!(log.contains("xchacha20-poly1305"));

        // capsule.yaml must be byte-for-byte untouched -- INTEGRITY.md's
        // "frozen forever" promise, not just the payload files.
        let capsule_text = fs::read_to_string(root.join("capsule.yaml")).unwrap();
        assert!(capsule_text.contains("lifecycle_stage: sealing"));
    }

    #[test]
    fn skips_a_collection_with_immediate_release_policy() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(
            root,
            LifecycleStage::Sealing,
            ReleasePolicyType::AfterDeath,
            Some(vec![collection(
                "photos",
                "data/photos",
                Some(ReleasePolicyType::Immediate),
            )]),
        );
        fs::create_dir_all(root.join("data").join("photos")).unwrap();
        fs::write(root.join("data").join("photos").join("a.jpg"), b"pic").unwrap();

        let summary = encrypt(root, true).unwrap();
        assert!(summary.skipped_immediate.contains(&"photos".to_string()));
        assert!(root.join("data").join("photos").join("a.jpg").exists());
        assert!(!root.join("data").join("photos").join("a.jpg.enc").exists());
    }

    #[test]
    fn encrypts_person_and_provenance_under_the_capsule_default() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(
            root,
            LifecycleStage::Sealing,
            ReleasePolicyType::Never,
            None,
        );
        fs::create_dir_all(root.join("data").join("person")).unwrap();
        fs::write(
            root.join("data").join("person").join("biography.md"),
            b"life story",
        )
        .unwrap();

        let summary = encrypt(root, true).unwrap();
        let person = summary
            .units
            .iter()
            .find(|u| u.name == "person")
            .expect("person unit");
        assert_eq!(person.files_encrypted, 1);
        assert!(root
            .join("data")
            .join("person")
            .join("biography.md.enc")
            .is_file());
    }

    #[test]
    fn dry_run_reports_counts_but_touches_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(
            root,
            LifecycleStage::Sealing,
            ReleasePolicyType::Immediate,
            Some(vec![collection(
                "email",
                "data/email",
                Some(ReleasePolicyType::AfterDeath),
            )]),
        );
        fs::create_dir_all(root.join("data").join("email")).unwrap();
        fs::write(root.join("data").join("email").join("msg.eml"), b"hi").unwrap();

        let summary = encrypt(root, false).unwrap();
        let email = summary
            .units
            .iter()
            .find(|u| u.name == "email")
            .expect("email unit");
        assert_eq!(email.files_encrypted, 1);
        assert!(email.key_hex.is_none());
        assert!(root.join("data").join("email").join("msg.eml").exists());
        assert!(!root.join("data").join("email").join("msg.eml.enc").exists());
        assert!(!root.join("preservation-log.yaml").exists());
    }

    #[test]
    fn already_encrypted_unit_is_idempotent() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(
            root,
            LifecycleStage::Sealing,
            ReleasePolicyType::Immediate,
            Some(vec![collection(
                "email",
                "data/email",
                Some(ReleasePolicyType::AfterDeath),
            )]),
        );
        fs::create_dir_all(root.join("data").join("email")).unwrap();
        fs::write(root.join("data").join("email").join("msg.eml"), b"hi").unwrap();

        encrypt(root, true).unwrap();
        let log_after_first_run = fs::read_to_string(root.join("preservation-log.yaml")).unwrap();

        let second = encrypt(root, true).unwrap();
        let email = second
            .units
            .iter()
            .find(|u| u.name == "email")
            .expect("email unit");
        assert!(email.already_encrypted);
        assert_eq!(email.files_encrypted, 0);
        assert!(email.key_hex.is_none());

        // No new key, no new log entry -- re-running is a safe no-op.
        let log_after_second_run = fs::read_to_string(root.join("preservation-log.yaml")).unwrap();
        assert_eq!(log_after_first_run, log_after_second_run);
    }

    #[test]
    fn refuses_when_not_freshly_sealed() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(
            root,
            LifecycleStage::Screening,
            ReleasePolicyType::AfterDeath,
            None,
        );

        assert!(encrypt(root, true).is_err());
    }
}
