//! Typed mirror of `schema/capsule.schema.json`. Field names, required-ness,
//! and enum values here must stay in sync with that schema and with
//! `FORMAT.md`. `deny_unknown_fields` is used on the schema's
//! `additionalProperties: false` objects so a malformed `capsule.yaml`
//! fails to parse loudly instead of silently dropping data -- this is the
//! runtime check `cli/README.md` has flagged as missing since `ptc init`
//! first shipped.
//!
//! Objects the schema marks `additionalProperties: true` (`creator`,
//! `purpose`, `custodian`, `trustee`) instead keep an `extra` field via
//! `#[serde(flatten)]`, so a hand-edited capsule.yaml with fields this
//! model doesn't know about round-trips without data loss.

use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub const SPEC_VERSION: &str = "PTC-0.1";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Capsule {
    pub capsule: CapsuleIdentity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<Purpose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<Collection>>,
    pub access: Access,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custody: Option<Custody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    pub integrity: Integrity,
    pub status: Status,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapsuleIdentity {
    pub spec_version: String,
    pub id: String,
    /// Kept as a plain string, not a date type -- FORMAT.md documents that
    /// this field must round-trip as a quoted YAML string, and re-parsing
    /// it as a date and re-emitting it risks exactly the unquoted-date
    /// footgun that document warns about.
    pub created: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Creator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Purpose {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Collection {
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_zone: Option<PrivacyZone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_policy: Option<ReleasePolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Access {
    pub default_privacy_zone: PrivacyZone,
    pub default_release_policy: ReleasePolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PrivacyZone {
    Public,
    Restricted,
    NeverRelease,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleasePolicy {
    #[serde(rename = "type")]
    pub policy_type: ReleasePolicyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleasePolicyType {
    Immediate,
    FixedDate,
    AfterDeath,
    AfterDeathPlusYears,
    Never,
    Custom,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Custody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<Custodian>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successors: Option<Vec<Custodian>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Custodian {
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub custodian_type: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Encryption {
    pub status: EncryptionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_custody: Option<KeyCustody>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncryptionStatus {
    Planned,
    Encrypted,
    None,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeyCustody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splitting_scheme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quorum: Option<Quorum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trustees: Option<Vec<Trustee>>,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Quorum {
    pub required: u32,
    pub of: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Trustee {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_index: Option<u32>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Integrity {
    pub manifest_algorithm: ManifestAlgorithm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ManifestAlgorithm {
    Sha256,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Status {
    pub lifecycle_stage: LifecycleStage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleStage {
    Origination,
    Import,
    Annotation,
    Screening,
    Sealing,
    Encryption,
    Deposit,
    DormantPreservation,
    ReleaseTrigger,
    Unlock,
    Access,
}

/// Read and parse a `capsule.yaml` at `path`. A parse failure here is the
/// runtime validation `cli/README.md` previously documented as missing --
/// `deny_unknown_fields` plus the required/optional shape above reject a
/// structurally malformed file instead of silently accepting it.
pub fn read_capsule_yaml(path: &Path) -> Result<Capsule> {
    let text = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    serde_yaml::from_str(&text)
        .with_context(|| format!("parsing {} against the capsule.yaml model", path.display()))
}

/// Serialize `capsule` to YAML, prefixed with `header_comment` (expected to
/// already end in a newline). Used only for freshly created capsules
/// (`ptc init`) -- an existing, hand-edited capsule.yaml is never
/// round-tripped through this, since re-serializing via serde would strip
/// any comments a creator added. See `seal.rs` for how `ptc seal` instead
/// patches a single field in place.
pub fn write_capsule_yaml(path: &Path, capsule: &Capsule, header_comment: &str) -> Result<()> {
    let body = serde_yaml::to_string(capsule).context("serializing capsule.yaml")?;
    let body = force_quote_date_like_fields(&body);
    let mut contents = String::with_capacity(header_comment.len() + body.len());
    contents.push_str(header_comment);
    contents.push_str(&body);
    fs::write(path, contents).with_context(|| format!("writing {}", path.display()))
}

/// Force double-quotes around the value of any `created:` or `date:` line
/// the YAML emitter left unquoted. This is exactly the gotcha FORMAT.md
/// documents: an unquoted `2026-08-30`-shaped scalar gets auto-resolved to
/// a native date/timestamp by many YAML parsers (confirmed against PyYAML
/// while building this), which then fails capsule.schema.json's `"type":
/// "string"` check on `capsule.created` / `release_policy.date` -- the
/// serializer itself doesn't know these particular keys need string-typed
/// values preserved, so this patches its output rather than trusting it.
fn force_quote_date_like_fields(yaml: &str) -> String {
    let mut out = String::with_capacity(yaml.len() + 16);
    for line in yaml.lines() {
        let trimmed = line.trim_start();
        let indent = &line[..line.len() - trimmed.len()];
        let mut rewritten = None;
        for key in ["created", "date"] {
            let prefix = format!("{key}: ");
            if let Some(value) = trimmed.strip_prefix(&prefix) {
                if !value.starts_with('"') && !value.starts_with('\'') {
                    rewritten = Some(format!("{indent}{key}: \"{value}\""));
                }
                break;
            }
        }
        out.push_str(&rewritten.unwrap_or_else(|| line.to_string()));
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_capsule() -> Capsule {
        Capsule {
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
                    policy_type: ReleasePolicyType::Custom,
                    date: None,
                    years: None,
                    description: Some("TBD -- set by the creator".to_string()),
                },
            },
            custody: None,
            encryption: None,
            integrity: Integrity {
                manifest_algorithm: ManifestAlgorithm::Sha256,
            },
            status: Status {
                lifecycle_stage: LifecycleStage::Origination,
            },
        }
    }

    #[test]
    fn round_trips_through_yaml() {
        let original = minimal_capsule();
        let yaml = serde_yaml::to_string(&original).expect("serialize");
        let parsed: Capsule = serde_yaml::from_str(&yaml).expect("deserialize");
        assert_eq!(parsed.capsule.id, original.capsule.id);
        assert_eq!(parsed.access.default_privacy_zone, PrivacyZone::Restricted);
        assert_eq!(
            parsed.access.default_release_policy.policy_type,
            ReleasePolicyType::Custom
        );
        assert_eq!(parsed.status.lifecycle_stage, LifecycleStage::Origination);
    }

    #[test]
    fn enum_values_match_schema_strings() {
        assert_eq!(
            serde_yaml::to_string(&PrivacyZone::NeverRelease)
                .unwrap()
                .trim(),
            "never-release"
        );
        assert_eq!(
            serde_yaml::to_string(&ReleasePolicyType::AfterDeathPlusYears)
                .unwrap()
                .trim(),
            "after_death_plus_years"
        );
        assert_eq!(
            serde_yaml::to_string(&LifecycleStage::DormantPreservation)
                .unwrap()
                .trim(),
            "dormant_preservation"
        );
        assert_eq!(
            serde_yaml::to_string(&ManifestAlgorithm::Sha256)
                .unwrap()
                .trim(),
            "sha256"
        );
    }

    #[test]
    fn write_capsule_yaml_quotes_the_created_date() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("capsule.yaml");
        write_capsule_yaml(&path, &minimal_capsule(), "").expect("write");

        let text = fs::read_to_string(&path).expect("read back");
        assert!(
            text.lines().any(|l| l.trim() == r#"created: "2026-08-30""#),
            "expected a quoted created date; got:\n{text}"
        );
        // Confirm it's not just quoted-looking but actually re-readable as
        // our own String-typed field too (belt and suspenders).
        let reparsed = read_capsule_yaml(&path).expect("re-read");
        assert_eq!(reparsed.capsule.created, "2026-08-30");
    }

    #[test]
    fn write_capsule_yaml_quotes_a_fixed_release_date() {
        let mut capsule = minimal_capsule();
        capsule.access.default_release_policy = ReleasePolicy {
            policy_type: ReleasePolicyType::FixedDate,
            date: Some("2076-08-20".to_string()),
            years: None,
            description: None,
        };

        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("capsule.yaml");
        write_capsule_yaml(&path, &capsule, "").expect("write");

        let text = fs::read_to_string(&path).expect("read back");
        assert!(
            text.lines().any(|l| l.trim() == r#"date: "2076-08-20""#),
            "expected a quoted release date; got:\n{text}"
        );
    }

    #[test]
    fn rejects_unknown_top_level_field() {
        let yaml = r#"
capsule:
  spec_version: "PTC-0.1"
  id: "urn:uuid:x"
  created: "2026-08-30"
access:
  default_privacy_zone: public
  default_release_policy:
    type: immediate
integrity:
  manifest_algorithm: sha256
status:
  lifecycle_stage: origination
not_a_real_field: true
"#;
        let result: Result<Capsule, _> = serde_yaml::from_str(yaml);
        assert!(
            result.is_err(),
            "expected an unknown top-level field to be rejected"
        );
    }
}
