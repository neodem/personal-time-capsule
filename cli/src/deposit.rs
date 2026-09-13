//! `ptc deposit` -- LIFECYCLE.md stage 7. The sealed (and possibly
//! encrypted) capsule is transferred to a custodian institution, which
//! verifies it and confirms acceptance.
//!
//! **Not a giant zip as the capsule's real format.** A capsule's canonical
//! form stays a directory -- every `manifest-sha256.txt` entry is an
//! individually addressable file, which is the whole point (an opaque
//! blob defeats "verify this specific file's checksum"). What this
//! command optionally produces with `--archive` is BagIt's own
//! "serialization" concept (RFC 8493): a single `.tar.gz` purely as a
//! transport convenience, unpacked back into the same directory at the
//! other end before anyone actually verifies anything against it.
//!
//! **Doesn't invent a transfer protocol.** There's no custodian-facing
//! upload API or transport mechanism defined anywhere in this spec, and
//! manufacturing one here would overstep into `CUSTODIAN_AGREEMENT.md`/
//! real institutional-process territory -- the same restraint
//! `ptc encrypt` showed by not inventing SLIP-0039 key distribution. What
//! this command actually does, scoped to what's mechanically well-defined:
//!
//! 1. **Verify first.** Runs the same checksum verification as
//!    `ptc verify` and refuses to proceed at all if it doesn't pass --
//!    depositing a capsule with known integrity problems is nonsensical.
//! 2. **Optionally serialize** (`--archive PATH`) into one `.tar.gz`.
//! 3. **Record the deposit.** Logs a `deposit` event to
//!    `preservation-log.yaml` -- naming the custodian, per LIFECYCLE.md's
//!    exit condition ("custodian has an independently verified copy and
//!    has agreed... to hold it"). Like `ptc encrypt`/`ptc unlock`, never
//!    touches `capsule.yaml` itself (frozen forever once sealed).
//!
//! Dry run by default, same convention as `ptc redact`/`ptc encrypt`/
//! `ptc unlock`: without `--apply`, verification still runs (and still
//! gates everything else), but no archive is written and nothing is
//! logged.
//!
//! ROADMAP.md item 29.

use crate::model::{self, EventFields, EventLog, LifecycleStage};
use crate::verify::{self, VerifyReport};
use anyhow::{bail, Context, Result};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::path::Path;

pub struct DepositSummary {
    pub verify_report: VerifyReport,
    pub archive_path: Option<String>,
    pub applied: bool,
}

pub fn deposit(
    root: &Path,
    custodian: &str,
    archive_path: Option<&Path>,
    apply: bool,
) -> Result<DepositSummary> {
    if custodian.trim().is_empty() {
        bail!("--custodian must not be empty");
    }

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
            "capsule.yaml's status.lifecycle_stage is {:?}, not sealing -- ptc deposit only runs \
             on a sealed capsule. Note: capsule.yaml is frozen forever once sealed (INTEGRITY.md), \
             so this field can never show anything past \"sealing\" even long after real \
             deposit/dormant-preservation/etc. happen -- check preservation-log.yaml's most recent \
             entry for a capsule's actual current stage.",
            capsule.status.lifecycle_stage
        );
    }

    let verify_report = verify::verify(root).context("verifying capsule before deposit")?;
    if !verify_report.passed() {
        bail!(
            "capsule failed verification -- refusing to deposit a capsule with integrity \
             problems. Run `ptc verify` for details."
        );
    }

    if let Some(path) = archive_path {
        check_archive_path(root, path)?;
    }

    if !apply {
        return Ok(DepositSummary {
            verify_report,
            archive_path: archive_path.map(|p| p.display().to_string()),
            applied: false,
        });
    }

    if let Some(path) = archive_path {
        create_archive(root, path)?;
    }

    let note = match archive_path {
        Some(p) => format!(
            "Verified locally via `ptc verify` before deposit; archived to {}",
            p.display()
        ),
        None => "Verified locally via `ptc verify` before deposit.".to_string(),
    };
    model::append_event(
        root,
        EventLog::Preservation,
        &EventFields {
            event_type: "deposit",
            agent: custodian,
            collection: None,
            path: None,
            source: None,
            outcome: Some("Manifest verified; accepted for deposit"),
            note: Some(&note),
        },
    )?;

    Ok(DepositSummary {
        verify_report,
        archive_path: archive_path.map(|p| p.display().to_string()),
        applied: true,
    })
}

/// Refuses an `--archive` destination inside the capsule itself --
/// writing the archive while simultaneously reading the same tree into it
/// would at best include a partial copy of itself, at worst corrupt the
/// write. Checked up front (before `--apply` does anything) so a dry run
/// catches this too.
fn check_archive_path(root: &Path, archive_path: &Path) -> Result<()> {
    let root_abs =
        std::fs::canonicalize(root).with_context(|| format!("resolving {}", root.display()))?;

    let parent = archive_path.parent().filter(|p| !p.as_os_str().is_empty());
    let parent_abs = match parent {
        Some(p) if p.is_dir() => {
            std::fs::canonicalize(p).with_context(|| format!("resolving {}", p.display()))?
        }
        Some(p) => {
            // Parent doesn't exist yet -- fall back to its own parent
            // chain, or the current directory, just to catch the common
            // "pointed it inside the capsule" mistake; `create_archive`
            // will create the real directory later, under --apply.
            std::env::current_dir().unwrap_or_default().join(p)
        }
        None => std::env::current_dir().unwrap_or_default(),
    };

    if parent_abs.starts_with(&root_abs) {
        bail!(
            "--archive path {} would land inside the capsule directory {} -- choose a \
             destination outside it",
            archive_path.display(),
            root.display()
        );
    }
    Ok(())
}

fn create_archive(root: &Path, archive_path: &Path) -> Result<()> {
    if let Some(parent) = archive_path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }

    let file = File::create(archive_path)
        .with_context(|| format!("creating {}", archive_path.display()))?;
    let encoder = GzEncoder::new(file, Compression::default());
    let mut builder = tar::Builder::new(encoder);

    // Named after the capsule's own directory, so unpacking the archive
    // produces a sensibly named folder rather than dumping loose files
    // into whatever directory it's extracted in.
    let root_name = root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "capsule".to_string());

    builder.append_dir_all(&root_name, root).with_context(|| {
        format!(
            "archiving {} into {}",
            root.display(),
            archive_path.display()
        )
    })?;
    builder
        .into_inner()
        .context("finishing tar stream")?
        .finish()
        .context("finishing gzip stream")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        Access, Capsule, CapsuleIdentity, Integrity, ManifestAlgorithm, PrivacyZone, ReleasePolicy,
        ReleasePolicyType, Status, SPEC_VERSION,
    };
    use crate::seal;
    use std::fs;

    fn sealed_capsule(root: &Path) {
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
                lifecycle_stage: LifecycleStage::Screening,
            },
        };
        model::write_capsule_yaml(&root.join("capsule.yaml"), &capsule, "").unwrap();
        fs::create_dir_all(root.join("data").join("writing")).unwrap();
        fs::write(
            root.join("data").join("writing").join("note.txt"),
            b"hello capsule",
        )
        .unwrap();
        seal::seal(root).unwrap();
    }

    #[test]
    fn dry_run_verifies_but_writes_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("capsule");
        fs::create_dir_all(&root).unwrap();
        sealed_capsule(&root);

        let summary = deposit(&root, "Springfield Historical Society", None, false).unwrap();
        assert!(summary.verify_report.passed());
        assert!(!summary.applied);
        assert!(!root.join("preservation-log.yaml").exists());
    }

    #[test]
    fn apply_logs_a_deposit_event() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("capsule");
        fs::create_dir_all(&root).unwrap();
        sealed_capsule(&root);

        let summary = deposit(&root, "Springfield Historical Society", None, true).unwrap();
        assert!(summary.applied);

        let log = fs::read_to_string(root.join("preservation-log.yaml")).unwrap();
        assert!(log.contains("type: deposit"));
        assert!(log.contains("Springfield Historical Society"));
        assert!(log.contains("Manifest verified"));
    }

    #[test]
    fn apply_creates_a_tar_gz_containing_the_capsule() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("capsule");
        fs::create_dir_all(&root).unwrap();
        sealed_capsule(&root);
        let archive_path = dir.path().join("out").join("capsule.tar.gz");

        let summary = deposit(
            &root,
            "Springfield Historical Society",
            Some(&archive_path),
            true,
        )
        .unwrap();
        assert_eq!(
            summary.archive_path.as_deref(),
            Some(archive_path.to_string_lossy().to_string().as_str())
        );
        assert!(archive_path.is_file());

        // Round-trip: the archive actually contains the sealed capsule's
        // files under a "capsule/" prefix.
        let file = File::open(&archive_path).unwrap();
        let decoder = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(decoder);
        let entries: Vec<String> = archive
            .entries()
            .unwrap()
            .map(|e| e.unwrap().path().unwrap().to_string_lossy().to_string())
            .map(|p| p.replace('\\', "/"))
            .collect();
        assert!(entries.iter().any(|p| p == "capsule/capsule.yaml"));
        assert!(entries.iter().any(|p| p == "capsule/data/writing/note.txt"));
        assert!(entries.iter().any(|p| p == "capsule/manifest-sha256.txt"));
    }

    #[test]
    fn refuses_an_archive_path_inside_the_capsule() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("capsule");
        fs::create_dir_all(&root).unwrap();
        sealed_capsule(&root);
        let bad_path = root.join("data").join("capsule.tar.gz");

        let result = deposit(
            &root,
            "Springfield Historical Society",
            Some(&bad_path),
            true,
        );
        assert!(result.is_err());
        assert!(!bad_path.exists());
        assert!(!root.join("preservation-log.yaml").exists());
    }

    #[test]
    fn refuses_verification_failure() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("capsule");
        fs::create_dir_all(&root).unwrap();
        sealed_capsule(&root);
        fs::write(
            root.join("data").join("writing").join("note.txt"),
            b"tampered!",
        )
        .unwrap();

        let result = deposit(&root, "Springfield Historical Society", None, true);
        assert!(result.is_err());
        assert!(!root.join("preservation-log.yaml").exists());
    }

    #[test]
    fn refuses_an_empty_custodian_name() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("capsule");
        fs::create_dir_all(&root).unwrap();
        sealed_capsule(&root);

        assert!(deposit(&root, "   ", None, true).is_err());
    }

    #[test]
    fn refuses_when_not_sealed() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("capsule");
        fs::create_dir_all(&root).unwrap();
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
                lifecycle_stage: LifecycleStage::Screening,
            },
        };
        model::write_capsule_yaml(&root.join("capsule.yaml"), &capsule, "").unwrap();

        assert!(deposit(&root, "Springfield Historical Society", None, true).is_err());
    }
}
