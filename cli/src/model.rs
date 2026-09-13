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

use anyhow::{bail, Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub const SPEC_VERSION: &str = "PTC-0.1";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Capsule {
    pub capsule: CapsuleIdentity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<Purpose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Series>,
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
    pub extra: BTreeMap<String, yaml_serde::Value>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Purpose {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, yaml_serde::Value>,
}

/// Optional, empty on almost every capsule -- FORMAT.md's `series` section
/// (ROADMAP.md item 8a). Deliberately one-directional (`continues` only,
/// never a forward-pointing `continued_by`) and deliberately not a merge
/// mechanism -- see that section for why.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Series {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continues: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
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
    pub extra: BTreeMap<String, yaml_serde::Value>,
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
    pub extra: BTreeMap<String, yaml_serde::Value>,
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
    yaml_serde::from_str(&text)
        .with_context(|| format!("parsing {} against the capsule.yaml model", path.display()))
}

/// Serialize `capsule` to YAML, prefixed with `header_comment` (expected to
/// already end in a newline). Used only for freshly created capsules
/// (`ptc init`) -- an existing, hand-edited capsule.yaml is never
/// round-tripped through this, since re-serializing via serde would strip
/// any comments a creator added. See `seal.rs` for how `ptc seal` instead
/// patches a single field in place.
pub fn write_capsule_yaml(path: &Path, capsule: &Capsule, header_comment: &str) -> Result<()> {
    let body = yaml_serde::to_string(capsule).context("serializing capsule.yaml")?;
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

/// Replace just the `lifecycle_stage:` line's value in `capsule.yaml`,
/// leaving every other line -- including any hand-written comments --
/// untouched. Deliberately not a full serde re-serialize: see
/// `write_capsule_yaml`'s doc comment for why that would lose comments.
/// Shared by `ptc seal` and `ptc import`.
pub fn patch_lifecycle_stage(path: &Path, new_stage: &str) -> Result<()> {
    let original =
        fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;

    let mut found = false;
    let mut patched_lines: Vec<String> = Vec::new();
    for line in original.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("lifecycle_stage:") {
            let indent_len = line.len() - trimmed.len();
            let indent = &line[..indent_len];
            patched_lines.push(format!("{}lifecycle_stage: {}", indent, new_stage));
            found = true;
        } else {
            patched_lines.push(line.to_string());
        }
    }

    if !found {
        bail!(
            "could not find a `lifecycle_stage:` line in {} to update",
            path.display()
        );
    }

    let mut patched = patched_lines.join("\n");
    if original.ends_with('\n') {
        patched.push('\n');
    }

    fs::write(path, patched).with_context(|| format!("writing {}", path.display()))
}

/// Append a new entry to `collections:` in `capsule.yaml`, or add a
/// `collections:` section (right before `access:`, which the schema
/// requires, so it's always present) if none exists yet. Text-surgical,
/// like `patch_lifecycle_stage` above, to avoid stripping comments via a
/// full re-serialize.
///
/// Caller is responsible for checking the collection isn't already
/// registered first (via the already-parsed `Capsule.collections`) --
/// this function always appends unconditionally.
pub fn patch_add_collection(path: &Path, name: &str, collection_path: &str) -> Result<()> {
    let original =
        fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let lines: Vec<&str> = original.lines().collect();

    // (where to insert, whether a `collections:` header line is also needed)
    let (insertion_index, needs_header) = if let Some(collections_idx) =
        lines.iter().position(|l| *l == "collections:")
    {
        // Find the end of the collections block: the first line after it
        // that's non-blank and unindented (a new top-level key), or EOF.
        let mut end = collections_idx + 1;
        while end < lines.len()
            && (lines[end].trim().is_empty() || lines[end].starts_with([' ', '\t']))
        {
            end += 1;
        }
        // Back up over trailing blank lines so the new entry lands right
        // after the last real entry, ahead of any blank line that
        // already separates the block from whatever follows.
        let mut content_end = end;
        while content_end > collections_idx + 1 && lines[content_end - 1].trim().is_empty() {
            content_end -= 1;
        }
        (content_end, false)
    } else if let Some(access_idx) = lines.iter().position(|l| *l == "access:") {
        // `access` is a required top-level field per capsule.schema.json,
        // so it's always present to anchor a brand-new section against.
        (access_idx, true)
    } else {
        bail!(
                "could not find a `collections:` or `access:` line in {} to anchor a new collection entry",
                path.display()
            );
    };

    let mut new_lines: Vec<String> = Vec::new();
    if needs_header {
        new_lines.push("collections:".to_string());
    } else {
        new_lines.push(String::new());
    }
    new_lines.push(format!("  - name: {name}"));
    new_lines.push(format!("    path: {collection_path}"));
    if needs_header {
        new_lines.push(String::new());
    }

    let mut patched_lines: Vec<String> = lines[..insertion_index]
        .iter()
        .map(|l| l.to_string())
        .collect();
    patched_lines.extend(new_lines);
    patched_lines.extend(lines[insertion_index..].iter().map(|l| l.to_string()));

    let mut patched = patched_lines.join("\n");
    if original.ends_with('\n') {
        patched.push('\n');
    }

    fs::write(path, patched).with_context(|| format!("writing {}", path.display()))
}

/// The pre-seal amend loop (`LIFECYCLE.md` stages 1-4) is only valid before
/// sealing -- once a manifest exists, mutating payload content without
/// re-sealing would silently invalidate it instead of being caught as
/// tampering. Shared by `ptc import` and `ptc redact`.
pub fn ensure_pre_seal_stage(stage: LifecycleStage) -> Result<()> {
    use LifecycleStage::*;
    match stage {
        Origination | Import | Annotation | Screening => Ok(()),
        _ => bail!(
            "capsule.yaml's status.lifecycle_stage is {stage:?}, past the pre-seal amend loop -- \
             modifying data/ now would silently invalidate an existing manifest instead of being \
             caught as tampering. Re-run `ptc seal` afterward if material still needs to change \
             this late."
        ),
    }
}

/// Which append-only event log to write to -- `data/provenance/events.yaml`
/// (pre-seal history, METADATA.md) or `preservation-log.yaml` (post-seal
/// history, INTEGRITY.md). The two share the same entry shape, but live at
/// different paths, get different first-write headers, and -- critically
/// -- have different relationships to sealing: `events.yaml` is payload,
/// frozen at sealing like everything else under `data/`; `preservation-log.yaml`
/// is a deliberately unsealed tag file, excluded from both manifests,
/// specifically so post-seal events (encryption, deposit, fixity checks,
/// ...) have somewhere honest to be recorded once `capsule.yaml` itself is
/// frozen. Never conflate which one a given event type belongs in -- see
/// each document's own "Event types" section.
pub enum EventLog {
    Provenance,
    Preservation,
}

impl EventLog {
    fn path(&self, root: &Path) -> PathBuf {
        match self {
            EventLog::Provenance => root.join("data").join("provenance").join("events.yaml"),
            EventLog::Preservation => root.join("preservation-log.yaml"),
        }
    }

    fn header(&self) -> &'static str {
        match self {
            EventLog::Provenance => {
                "# data/provenance/events.yaml -- see METADATA.md for the field reference:\n\
                 # https://github.com/neodem/personal-time-capsule/blob/main/METADATA.md\n\n"
            }
            EventLog::Preservation => {
                "# preservation-log.yaml -- see INTEGRITY.md for the field reference. NOT\n\
                 # covered by manifest-sha256.txt or tagmanifest-sha256.txt -- see INTEGRITY.md\n\
                 # for why this file, unlike capsule.yaml, is expected to keep changing after\n\
                 # sealing:\n\
                 # https://github.com/neodem/personal-time-capsule/blob/main/INTEGRITY.md\n\n"
            }
        }
    }
}

/// Fields for one event-log entry -- see METADATA.md's (or INTEGRITY.md's)
/// field reference. `date` is always "now" and isn't a parameter.
pub struct EventFields<'a> {
    pub event_type: &'a str,
    pub agent: &'a str,
    pub collection: Option<&'a str>,
    pub path: Option<&'a str>,
    pub source: Option<&'a str>,
    pub outcome: Option<&'a str>,
    pub note: Option<&'a str>,
}

/// Append one entry to whichever event log `log` names, creating it with a
/// short header if it doesn't exist yet. Shared by `ptc import`/`ptc redact`
/// (both log against `data/provenance/events.yaml`) and `ptc encrypt`
/// (logs against `preservation-log.yaml`, since encryption is a post-seal
/// event -- see `EventLog`'s own doc comment).
pub fn append_event(root: &Path, log: EventLog, fields: &EventFields) -> Result<()> {
    let events_path = log.path(root);
    let date = chrono::Utc::now().format("%Y-%m-%d");

    let mut entry = format!(
        "- date: \"{date}\"\n  type: {}\n  agent: \"{}\"\n",
        fields.event_type,
        escape_yaml_double_quoted(fields.agent)
    );
    if let Some(c) = fields.collection {
        entry.push_str(&format!("  collection: {c}\n"));
    }
    if let Some(p) = fields.path {
        entry.push_str(&format!("  path: \"{}\"\n", escape_yaml_double_quoted(p)));
    }
    if let Some(s) = fields.source {
        entry.push_str(&format!("  source: \"{}\"\n", escape_yaml_double_quoted(s)));
    }
    if let Some(o) = fields.outcome {
        entry.push_str(&format!(
            "  outcome: \"{}\"\n",
            escape_yaml_double_quoted(o)
        ));
    }
    if let Some(n) = fields.note {
        entry.push_str(&format!("  note: \"{}\"\n", escape_yaml_double_quoted(n)));
    }

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
        let contents = format!("{}{entry}", log.header());
        fs::write(&events_path, contents)
            .with_context(|| format!("writing {}", events_path.display()))
    }
}

/// Escape `"` and `\` for embedding in a double-quoted YAML scalar. Several
/// event fields (`agent`, `source`, `outcome`, `note`) are free text a
/// creator or another tool supplies, so this can't assume they're
/// quote-free.
fn escape_yaml_double_quoted(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
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
            series: None,
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
        let yaml = yaml_serde::to_string(&original).expect("serialize");
        let parsed: Capsule = yaml_serde::from_str(&yaml).expect("deserialize");
        assert_eq!(parsed.capsule.id, original.capsule.id);
        assert_eq!(parsed.access.default_privacy_zone, PrivacyZone::Restricted);
        assert_eq!(
            parsed.access.default_release_policy.policy_type,
            ReleasePolicyType::Custom
        );
        assert_eq!(parsed.status.lifecycle_stage, LifecycleStage::Origination);
    }

    #[test]
    fn series_field_round_trips_and_is_absent_by_default() {
        let mut capsule = minimal_capsule();
        assert!(capsule.series.is_none());

        capsule.series = Some(Series {
            continues: Some("urn:uuid:earlier-capsule".to_string()),
            note: Some("picks up where the earlier one left off".to_string()),
        });
        let yaml = yaml_serde::to_string(&capsule).expect("serialize");
        assert!(
            yaml.contains("series:"),
            "expected a series: block; got:\n{yaml}"
        );

        let parsed: Capsule = yaml_serde::from_str(&yaml).expect("deserialize");
        let series = parsed.series.expect("series should round-trip");
        assert_eq!(
            series.continues.as_deref(),
            Some("urn:uuid:earlier-capsule")
        );
        assert_eq!(
            series.note.as_deref(),
            Some("picks up where the earlier one left off")
        );

        // Absent by default -- serializing minimal_capsule() shouldn't emit
        // a series: key at all (skip_serializing_if).
        let minimal_yaml = yaml_serde::to_string(&minimal_capsule()).expect("serialize");
        assert!(
            !minimal_yaml.contains("series:"),
            "series should be omitted when None; got:\n{minimal_yaml}"
        );
    }

    #[test]
    fn enum_values_match_schema_strings() {
        assert_eq!(
            yaml_serde::to_string(&PrivacyZone::NeverRelease)
                .unwrap()
                .trim(),
            "never-release"
        );
        assert_eq!(
            yaml_serde::to_string(&ReleasePolicyType::AfterDeathPlusYears)
                .unwrap()
                .trim(),
            "after_death_plus_years"
        );
        assert_eq!(
            yaml_serde::to_string(&LifecycleStage::DormantPreservation)
                .unwrap()
                .trim(),
            "dormant_preservation"
        );
        assert_eq!(
            yaml_serde::to_string(&ManifestAlgorithm::Sha256)
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
    fn patch_lifecycle_stage_preserves_comments() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("capsule.yaml");
        fs::write(
            &path,
            "status:\n  # a hand-written comment\n  lifecycle_stage: origination\nother: true\n",
        )
        .unwrap();

        patch_lifecycle_stage(&path, "import").unwrap();

        let result = fs::read_to_string(&path).unwrap();
        assert_eq!(
            result,
            "status:\n  # a hand-written comment\n  lifecycle_stage: import\nother: true\n"
        );
    }

    #[test]
    fn patch_add_collection_appends_to_existing_block() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("capsule.yaml");
        fs::write(
            &path,
            "collections:\n  - name: person\n    path: data/person\n\naccess:\n  default_privacy_zone: public\n",
        )
        .unwrap();

        patch_add_collection(&path, "photos", "data/photos").unwrap();

        let result = fs::read_to_string(&path).unwrap();
        assert_eq!(
            result,
            "collections:\n  - name: person\n    path: data/person\n\n  - name: photos\n    path: data/photos\n\naccess:\n  default_privacy_zone: public\n"
        );
    }

    #[test]
    fn patch_add_collection_creates_section_before_access() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("capsule.yaml");
        fs::write(
            &path,
            "capsule:\n  id: x\naccess:\n  default_privacy_zone: public\n",
        )
        .unwrap();

        patch_add_collection(&path, "photos", "data/photos").unwrap();

        let result = fs::read_to_string(&path).unwrap();
        assert_eq!(
            result,
            "capsule:\n  id: x\ncollections:\n  - name: photos\n    path: data/photos\n\naccess:\n  default_privacy_zone: public\n"
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
        let result: Result<Capsule, _> = yaml_serde::from_str(yaml);
        assert!(
            result.is_err(),
            "expected an unknown top-level field to be rejected"
        );
    }
}
