//! `ptc scan` -- LIFECYCLE.md stage 4 (Screening) tooling. Walks a
//! capsule's payload looking for sensitive-data patterns and reports
//! *candidates* for the creator to review -- it never writes anything
//! (not even a draft) to `privacy-overrides.yaml` itself. Per
//! `LIFECYCLE.md`'s own framing of this stage: "the tooling surfaces
//! candidates, the creator decides" (`PRINCIPLES.md` #2). See `PRIVACY.md`
//! for the category vocabulary this reuses (`financial`, `credentials`,
//! `government-id`) and for why *classifying* an item is this project's
//! job while *detecting* it wasn't designed until now.
//!
//! Detects three of `PRIVACY.md`'s six recommended categories --
//! `medical` and `third-party-privacy` need semantic understanding no
//! regex can responsibly claim, and `other` isn't a detectable pattern at
//! all, just a catch-all label for a creator's own manual entries.
//!
//! ROADMAP.md item 24.

use crate::model;
use anyhow::{bail, Context, Result};
use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub struct ScanReport {
    pub scan_root: String,
    pub files_scanned: usize,
    pub files_skipped_binary: usize,
    pub findings: Vec<Finding>,
}

pub struct Finding {
    pub path: String,
    pub line: usize,
    pub category: &'static str,
    pub description: &'static str,
    /// A masked excerpt safe to print (e.g. `***-**-6789`), or `None` when
    /// no part of the match is safe to echo at all (a real credential).
    pub snippet: Option<String>,
    /// Whether `ptc redact` can safely neutralize this finding on its own.
    /// `false` only for a private key block header: the regex only spots
    /// the `-----BEGIN...-----` line, not the multi-line key body that
    /// follows, so "redacting" just that line would blank the label while
    /// leaving the actual key material intact -- worse than not touching
    /// it, since it would look handled. See `Detectors::redact_line`.
    pub redactable: bool,
}

pub fn scan(root: &Path, collection: Option<&str>) -> Result<ScanReport> {
    let capsule_yaml_path = root.join("capsule.yaml");
    let capsule = model::read_capsule_yaml(&capsule_yaml_path).with_context(|| {
        format!(
            "{} must exist and parse -- is {} a capsule directory?",
            capsule_yaml_path.display(),
            root.display()
        )
    })?;

    let (scan_dir, scan_root_rel) = match collection {
        Some(name) => {
            let rel_path = capsule
                .collections
                .iter()
                .flatten()
                .find(|c| c.name == name)
                .map(|c| c.path.clone())
                .unwrap_or_else(|| format!("data/{name}"));
            let dir = root.join(&rel_path);
            if !dir.is_dir() {
                bail!(
                    "collection {name:?} has no directory at {} -- nothing to scan",
                    dir.display()
                );
            }
            (dir, rel_path)
        }
        None => {
            let dir = root.join("data");
            if !dir.is_dir() {
                bail!("{} does not exist -- nothing to scan", dir.display());
            }
            (dir, "data".to_string())
        }
    };

    let detectors = Detectors::new();
    let mut findings = Vec::new();
    let mut files_scanned = 0usize;
    let mut files_skipped_binary = 0usize;

    for entry in WalkDir::new(&scan_dir) {
        let entry = entry.with_context(|| format!("walking {}", scan_dir.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }
        let abs = entry.path();
        let rel = abs
            .strip_prefix(root)
            .with_context(|| format!("computing a path relative to {}", root.display()))?
            .to_string_lossy()
            .replace('\\', "/");

        let bytes = fs::read(abs).with_context(|| format!("reading {}", abs.display()))?;
        if looks_binary(&bytes) {
            files_skipped_binary += 1;
            continue;
        }
        files_scanned += 1;

        let text = String::from_utf8_lossy(&bytes);
        for (i, line) in text.lines().enumerate() {
            detectors.scan_line(&rel, i + 1, line, &mut findings);
        }
    }

    Ok(ScanReport {
        scan_root: scan_root_rel,
        files_scanned,
        files_skipped_binary,
        findings,
    })
}

/// Same heuristic `git` itself uses: a NUL byte anywhere in a leading
/// sample means "not text." Cheap, no file-extension allowlist to keep in
/// sync, and correctly skips the photos/audio/video that make up most of
/// a real capsule's bytes.
pub(crate) fn looks_binary(bytes: &[u8]) -> bool {
    let sample = &bytes[..bytes.len().min(8000)];
    sample.contains(&0)
}

pub(crate) struct Detectors {
    ssn: Regex,
    credit_card: Regex,
    aws_key: Regex,
    private_key: Regex,
    credential_assignment: Regex,
}

impl Detectors {
    pub(crate) fn new() -> Self {
        Detectors {
            ssn: Regex::new(r"\b\d{3}-\d{2}-\d{4}\b").expect("valid hardcoded regex"),
            // 13-19 digits total, optionally grouped with spaces/dashes --
            // covers Visa/Mastercard/Amex/Discover lengths. Luhn-checked
            // afterward (see scan_line) since the shape alone matches all
            // kinds of unrelated long digit runs (phone numbers, tracking
            // IDs, ...).
            credit_card: Regex::new(r"\b(?:\d[ -]?){12,18}\d\b").expect("valid hardcoded regex"),
            aws_key: Regex::new(r"\bAKIA[0-9A-Z]{16}\b").expect("valid hardcoded regex"),
            private_key: Regex::new(
                r"-----BEGIN (?:RSA |EC |OPENSSH |DSA |ENCRYPTED )?PRIVATE KEY-----",
            )
            .expect("valid hardcoded regex"),
            // Group 2 captures just the secret value, not the "password: "
            // label -- redact_line needs to replace only the value so the
            // redacted line still reads sensibly (see below).
            credential_assignment: Regex::new(
                r#"(?i)\b(?:api[_-]?key|secret|password|passwd|access[_-]?token)\b\s*[:=]\s*(["']?)([A-Za-z0-9_\-/+=]{8,})(["']?)"#,
            )
            .expect("valid hardcoded regex"),
        }
    }

    pub(crate) fn scan_line(
        &self,
        path: &str,
        line_number: usize,
        line: &str,
        findings: &mut Vec<Finding>,
    ) {
        if let Some(m) = self.ssn.find(line) {
            findings.push(Finding {
                path: path.to_string(),
                line: line_number,
                category: "government-id",
                description: "Possible SSN",
                snippet: Some(mask_keep_last(m.as_str(), 4)),
                redactable: true,
            });
        }

        for m in self.credit_card.find_iter(line) {
            let digits: String = m.as_str().chars().filter(char::is_ascii_digit).collect();
            if luhn_valid(&digits) {
                findings.push(Finding {
                    path: path.to_string(),
                    line: line_number,
                    category: "financial",
                    description: "Possible credit card number (Luhn-valid)",
                    snippet: Some(mask_keep_last(&digits, 4)),
                    redactable: true,
                });
            }
        }

        if self.aws_key.is_match(line) {
            findings.push(Finding {
                path: path.to_string(),
                line: line_number,
                category: "credentials",
                description: "Possible AWS access key ID",
                snippet: None,
                redactable: true,
            });
        }

        if self.private_key.is_match(line) {
            findings.push(Finding {
                path: path.to_string(),
                line: line_number,
                category: "credentials",
                description: "Private key block header",
                snippet: None,
                redactable: false,
            });
        }

        if self.credential_assignment.is_match(line) {
            findings.push(Finding {
                path: path.to_string(),
                line: line_number,
                category: "credentials",
                description: "Possible credential assignment (password/api key/token)",
                snippet: None,
                redactable: true,
            });
        }
    }

    /// Return `line` with every auto-redactable finding's matched span
    /// replaced by `[REDACTED]`. A private key header is deliberately left
    /// untouched (see `Finding::redactable`'s doc comment) -- callers
    /// should still surface it via `scan_line`'s findings so a creator
    /// knows to handle it by hand.
    pub(crate) fn redact_line(&self, line: &str) -> RedactedLine {
        let mut spans: Vec<(usize, usize, &'static str)> = Vec::new();

        if let Some(m) = self.ssn.find(line) {
            spans.push((m.start(), m.end(), "government-id"));
        }
        for m in self.credit_card.find_iter(line) {
            let digits: String = m.as_str().chars().filter(char::is_ascii_digit).collect();
            if luhn_valid(&digits) {
                spans.push((m.start(), m.end(), "financial"));
            }
        }
        if let Some(m) = self.aws_key.find(line) {
            spans.push((m.start(), m.end(), "credentials"));
        }
        // Only the captured value (group 2) is replaced, not the
        // "password: " label, so the redacted line still reads sensibly.
        for caps in self.credential_assignment.captures_iter(line) {
            if let Some(value) = caps.get(2) {
                spans.push((value.start(), value.end(), "credentials"));
            }
        }

        // Drop any span that overlaps one already kept (ascending by
        // start), then apply replacements right-to-left so earlier,
        // still-original byte offsets stay valid as `redacted` mutates.
        spans.sort_by_key(|s| s.0);
        let mut kept: Vec<(usize, usize, &'static str)> = Vec::new();
        let mut last_end = 0usize;
        for (start, end, category) in spans {
            if start < last_end {
                continue;
            }
            last_end = end;
            kept.push((start, end, category));
        }

        let mut redacted = line.to_string();
        for (start, end, _) in kept.iter().rev() {
            redacted.replace_range(*start..*end, "[REDACTED]");
        }

        RedactedLine {
            text: redacted,
            categories: kept.into_iter().map(|(_, _, c)| c).collect(),
        }
    }
}

pub(crate) struct RedactedLine {
    pub text: String,
    /// One entry per span actually replaced on this line, in left-to-right
    /// order. Empty means nothing on this line was auto-redactable.
    pub categories: Vec<&'static str>,
}

/// Replace every character except the last `keep` with `*`, preserving
/// length and any separators -- `123-45-6789` -> `***-**-6789`. Never
/// echoes more of a real identifier than necessary to let a creator
/// recognize which one was flagged.
fn mask_keep_last(s: &str, keep: usize) -> String {
    let len = s.chars().count();
    let mask_len = len.saturating_sub(keep);
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if i < mask_len && c.is_ascii_digit() {
                '*'
            } else {
                c
            }
        })
        .collect()
}

fn luhn_valid(digits: &str) -> bool {
    let values: Vec<u32> = digits.chars().filter_map(|c| c.to_digit(10)).collect();
    if values.len() < 13 || values.len() > 19 {
        return false;
    }
    let sum: u32 = values
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| {
            if i % 2 == 1 {
                let doubled = d * 2;
                if doubled > 9 {
                    doubled - 9
                } else {
                    doubled
                }
            } else {
                d
            }
        })
        .sum();
    sum.is_multiple_of(10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        Access, Capsule, CapsuleIdentity, Collection, Integrity, LifecycleStage, ManifestAlgorithm,
        PrivacyZone, ReleasePolicy, ReleasePolicyType, Status, SPEC_VERSION,
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
    }

    #[test]
    fn detects_ssn_and_masks_all_but_last_four() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, None);
        fs::create_dir_all(root.join("data").join("writing")).unwrap();
        fs::write(
            root.join("data").join("writing").join("note.txt"),
            "My SSN is 123-45-6789, don't tell anyone.\n",
        )
        .unwrap();

        let report = scan(root, None).unwrap();
        assert_eq!(report.findings.len(), 1);
        let f = &report.findings[0];
        assert_eq!(f.category, "government-id");
        assert_eq!(f.path, "data/writing/note.txt");
        assert_eq!(f.line, 1);
        assert_eq!(f.snippet.as_deref(), Some("***-**-6789"));
    }

    #[test]
    fn detects_luhn_valid_card_and_ignores_luhn_invalid_digits() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, None);
        fs::create_dir_all(root.join("data").join("email")).unwrap();
        // 4111111111111111 is a well-known Luhn-valid Visa test number.
        // 1234567890123456 is the same length but Luhn-invalid.
        fs::write(
            root.join("data").join("email").join("msg.eml"),
            "Card: 4111111111111111\nTracking: 1234567890123456\n",
        )
        .unwrap();

        let report = scan(root, None).unwrap();
        assert_eq!(report.findings.len(), 1);
        assert_eq!(report.findings[0].category, "financial");
        assert_eq!(report.findings[0].line, 1);
        assert_eq!(
            report.findings[0].snippet.as_deref(),
            Some("************1111")
        );
    }

    #[test]
    fn detects_aws_key_and_private_key_and_credential_assignment_without_a_snippet() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, None);
        fs::create_dir_all(root.join("data").join("writing")).unwrap();
        fs::write(
            root.join("data").join("writing").join("secrets.txt"),
            "aws_key = AKIAABCDEFGHIJKLMNOP\n\
             -----BEGIN RSA PRIVATE KEY-----\n\
             password: hunter2hunter2\n",
        )
        .unwrap();

        let report = scan(root, None).unwrap();
        assert_eq!(report.findings.len(), 3);
        assert!(report.findings.iter().all(|f| f.category == "credentials"));
        assert!(report.findings.iter().all(|f| f.snippet.is_none()));
    }

    #[test]
    fn skips_binary_files() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, None);
        fs::create_dir_all(root.join("data").join("photos")).unwrap();
        let mut bytes = b"123-45-6789".to_vec();
        bytes.insert(0, 0u8); // NUL byte marks it binary
        fs::write(root.join("data").join("photos").join("a.jpg"), &bytes).unwrap();

        let report = scan(root, None).unwrap();
        assert_eq!(report.files_scanned, 0);
        assert_eq!(report.files_skipped_binary, 1);
        assert!(report.findings.is_empty());
    }

    #[test]
    fn can_be_scoped_to_one_collection() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(
            root,
            Some(vec![
                Collection {
                    name: "email".to_string(),
                    path: "data/email".to_string(),
                    privacy_zone: None,
                    release_policy: None,
                    encryption: None,
                },
                Collection {
                    name: "writing".to_string(),
                    path: "data/writing".to_string(),
                    privacy_zone: None,
                    release_policy: None,
                    encryption: None,
                },
            ]),
        );
        fs::create_dir_all(root.join("data").join("email")).unwrap();
        fs::create_dir_all(root.join("data").join("writing")).unwrap();
        fs::write(
            root.join("data").join("email").join("msg.eml"),
            "SSN: 123-45-6789\n",
        )
        .unwrap();
        fs::write(
            root.join("data").join("writing").join("note.txt"),
            "SSN: 111-22-3333\n",
        )
        .unwrap();

        let report = scan(root, Some("writing")).unwrap();
        assert_eq!(report.scan_root, "data/writing");
        assert_eq!(report.files_scanned, 1);
        assert_eq!(report.findings.len(), 1);
        assert_eq!(report.findings[0].path, "data/writing/note.txt");
    }

    #[test]
    fn bails_when_capsule_yaml_missing() {
        let dir = tempfile::tempdir().unwrap();
        assert!(scan(dir.path(), None).is_err());
    }

    #[test]
    fn bails_when_named_collection_directory_does_not_exist() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, None);
        assert!(scan(root, Some("nonexistent")).is_err());
    }
}
