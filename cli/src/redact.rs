//! `ptc redact` -- LIFECYCLE.md stage 4 (Screening) tooling, alongside
//! `ptc scan`. Where `ptc scan` only reports candidates, `ptc redact`
//! actually neutralizes them in one creator-named file -- but only the
//! findings it can neutralize safely (see `scan::Finding::redactable`).
//!
//! Answers PRIVACY.md's own open question about redaction mechanics
//! ("kept alongside in a stricter zone? overwritten entirely?", item 25's
//! job to decide): this implementation overwrites the target file in
//! place, with no automatic backup copy. The safety net is the workflow,
//! not a hidden file -- `ptc redact` defaults to a dry run identical to
//! what `ptc scan` would show for this one file, and only writes anything
//! when `--apply` is passed explicitly. A creator who wants an undo path
//! should keep their own copy (or use version control) before applying.
//!
//! ROADMAP.md item 25.

use crate::model;
use crate::scan::{Detectors, Finding};
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;

pub struct RedactSummary {
    pub path: String,
    pub findings: Vec<Finding>,
    pub applied: bool,
    pub redactions_applied: usize,
}

pub fn redact(root: &Path, target: &str, apply: bool) -> Result<RedactSummary> {
    let capsule_yaml_path = root.join("capsule.yaml");
    let capsule = model::read_capsule_yaml(&capsule_yaml_path).with_context(|| {
        format!(
            "{} must exist and parse -- is {} a capsule directory?",
            capsule_yaml_path.display(),
            root.display()
        )
    })?;

    model::ensure_pre_seal_stage(capsule.status.lifecycle_stage)?;

    let rel = target.replace('\\', "/");
    if !rel.starts_with("data/") {
        bail!("--target must be a path under data/ (payload), not a tag file (got {rel:?})");
    }

    let file_path = root.join(&rel);
    if !file_path.is_file() {
        bail!("{} is not a file -- nothing to redact", file_path.display());
    }

    let bytes = fs::read(&file_path).with_context(|| format!("reading {}", file_path.display()))?;
    if crate::scan::looks_binary(&bytes) {
        bail!(
            "{} looks like a binary file -- ptc redact only handles text content",
            file_path.display()
        );
    }

    let text = String::from_utf8_lossy(&bytes);
    let detectors = Detectors::new();

    // Dry-run-style findings (identical to what `ptc scan` would report
    // for this one file) -- built either way, so the CLI can show exactly
    // what changed, or would change, in the same shape.
    let mut findings = Vec::new();
    for (i, line) in text.lines().enumerate() {
        detectors.scan_line(&rel, i + 1, line, &mut findings);
    }

    let mut redactions_applied = 0usize;
    if apply && !findings.is_empty() {
        let mut new_lines = Vec::with_capacity(text.lines().count());
        for line in text.lines() {
            let result = detectors.redact_line(line);
            redactions_applied += result.categories.len();
            new_lines.push(result.text);
        }

        if redactions_applied > 0 {
            let mut new_text = new_lines.join("\n");
            if text.ends_with('\n') {
                new_text.push('\n');
            }
            fs::write(&file_path, new_text)
                .with_context(|| format!("writing {}", file_path.display()))?;

            let outcome = format!("{redactions_applied} redaction(s) applied by ptc redact");
            model::append_event(
                root,
                model::EventLog::Provenance,
                &model::EventFields {
                    event_type: "screening",
                    agent: &format!("ptc CLI v{}", env!("CARGO_PKG_VERSION")),
                    collection: None,
                    path: Some(&rel),
                    source: None,
                    outcome: Some(&outcome),
                    note: None,
                },
            )?;
        }
    }

    Ok(RedactSummary {
        path: rel,
        findings,
        applied: apply && redactions_applied > 0,
        redactions_applied,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        Access, Capsule, CapsuleIdentity, Integrity, LifecycleStage, ManifestAlgorithm,
        PrivacyZone, ReleasePolicy, ReleasePolicyType, Status, SPEC_VERSION,
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
    fn dry_run_reports_findings_but_does_not_modify_the_file() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, LifecycleStage::Screening);
        fs::create_dir_all(root.join("data").join("writing")).unwrap();
        let file = root.join("data").join("writing").join("note.txt");
        let original = "My SSN is 123-45-6789.\n";
        fs::write(&file, original).unwrap();

        let summary = redact(root, "data/writing/note.txt", false).unwrap();
        assert_eq!(summary.findings.len(), 1);
        assert!(!summary.applied);
        assert_eq!(summary.redactions_applied, 0);
        assert_eq!(fs::read_to_string(&file).unwrap(), original);
    }

    #[test]
    fn apply_redacts_ssn_and_credit_card_and_logs_a_screening_event() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, LifecycleStage::Screening);
        fs::create_dir_all(root.join("data").join("email")).unwrap();
        let file = root.join("data").join("email").join("msg.eml");
        fs::write(
            &file,
            "SSN: 123-45-6789\nCard: 4111111111111111\nHello there.\n",
        )
        .unwrap();

        let summary = redact(root, "data/email/msg.eml", true).unwrap();
        assert!(summary.applied);
        assert_eq!(summary.redactions_applied, 2);

        let contents = fs::read_to_string(&file).unwrap();
        assert_eq!(
            contents,
            "SSN: [REDACTED]\nCard: [REDACTED]\nHello there.\n"
        );

        let events =
            fs::read_to_string(root.join("data").join("provenance").join("events.yaml")).unwrap();
        assert!(events.contains("type: screening"));
        assert!(events.contains(r#"path: "data/email/msg.eml""#));
        assert!(events.contains("2 redaction(s) applied"));
    }

    #[test]
    fn apply_redacts_credential_value_but_keeps_the_label() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, LifecycleStage::Screening);
        fs::create_dir_all(root.join("data").join("writing")).unwrap();
        let file = root.join("data").join("writing").join("secrets.txt");
        fs::write(&file, "password: hunter2hunter2\n").unwrap();

        redact(root, "data/writing/secrets.txt", true).unwrap();

        let contents = fs::read_to_string(&file).unwrap();
        assert_eq!(contents, "password: [REDACTED]\n");
    }

    #[test]
    fn apply_leaves_a_private_key_header_untouched_and_does_not_claim_it_was_redacted() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, LifecycleStage::Screening);
        fs::create_dir_all(root.join("data").join("writing")).unwrap();
        let file = root.join("data").join("writing").join("key.pem");
        let original = "-----BEGIN RSA PRIVATE KEY-----\nMIIBAAdummybody\n";
        fs::write(&file, original).unwrap();

        let summary = redact(root, "data/writing/key.pem", true).unwrap();
        // The header is a finding (so the creator sees it) but not
        // auto-redactable, and nothing else on this file is redactable, so
        // apply results in zero actual redactions and an untouched file.
        assert_eq!(summary.findings.len(), 1);
        assert!(!summary.findings[0].redactable);
        assert_eq!(summary.redactions_applied, 0);
        assert!(!summary.applied);
        assert_eq!(fs::read_to_string(&file).unwrap(), original);
    }

    #[test]
    fn refuses_after_sealing() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, LifecycleStage::Sealing);
        fs::create_dir_all(root.join("data").join("writing")).unwrap();
        let file = root.join("data").join("writing").join("note.txt");
        fs::write(&file, "SSN: 123-45-6789\n").unwrap();

        let result = redact(root, "data/writing/note.txt", true);
        assert!(result.is_err());
        assert_eq!(
            fs::read_to_string(&file).unwrap(),
            "SSN: 123-45-6789\n",
            "file must be untouched when the guard rejects the request"
        );
    }

    #[test]
    fn refuses_a_target_outside_data() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, LifecycleStage::Screening);
        fs::write(root.join("capsule.yaml.bak"), "SSN: 123-45-6789\n").unwrap();

        assert!(redact(root, "capsule.yaml.bak", true).is_err());
    }

    #[test]
    fn refuses_a_binary_target() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, LifecycleStage::Screening);
        fs::create_dir_all(root.join("data").join("photos")).unwrap();
        let file = root.join("data").join("photos").join("a.jpg");
        fs::write(&file, [0u8, 1, 2, 3]).unwrap();

        assert!(redact(root, "data/photos/a.jpg", true).is_err());
    }

    #[test]
    fn refuses_a_missing_target() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, LifecycleStage::Screening);

        assert!(redact(root, "data/writing/nonexistent.txt", true).is_err());
    }
}
