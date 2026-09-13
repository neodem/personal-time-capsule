//! `ptc inspect` -- a read-only summary of a capsule's current state,
//! sitting (per LIFECYCLE.md's command ordering) between `import`/
//! `annotation` and `scan`/`screening`: "what's actually in here right
//! now," not integrity verification (`ptc verify`, ROADMAP.md item 27,
//! doesn't exist yet) or sensitive-data scanning (`ptc scan`, item 24).
//!
//! ROADMAP.md item 23.

use crate::model::{self, Capsule, PrivacyZone, ReleasePolicy, ReleasePolicyType};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct InspectReport {
    pub capsule: Capsule,
    pub collections: Vec<CollectionReport>,
    pub undeclared_directories: Vec<DirectoryReport>,
    pub person_layer: PersonLayerReport,
    pub provenance: Option<ProvenanceReport>,
    pub privacy_overrides_count: Option<usize>,
    pub legal_files: Option<Vec<String>>,
    pub preservation_log_entries: Option<usize>,
    pub manifest_present: bool,
    pub tagmanifest_present: bool,
    pub total_payload_files: usize,
    pub total_payload_bytes: u64,
}

pub struct CollectionReport {
    pub name: String,
    pub path: String,
    pub exists: bool,
    pub file_count: usize,
    pub total_bytes: u64,
    pub privacy_zone: String,
    pub release_policy: String,
}

pub struct DirectoryReport {
    pub name: String,
    pub file_count: usize,
    pub total_bytes: u64,
}

pub struct PersonLayerReport {
    pub biography_present: bool,
    pub timeline_present: bool,
    pub people_count: Option<usize>,
    pub places_count: Option<usize>,
}

pub struct ProvenanceReport {
    pub event_count: usize,
    pub most_recent: Option<EventLine>,
}

pub struct EventLine {
    pub date: String,
    pub event_type: String,
    pub agent: String,
}

/// Deliberately lenient (no `deny_unknown_fields`, unlike `model::Capsule`):
/// this is a read-only reporting tool, not a validator -- a hand-edited
/// events.yaml with an extra field or a future schema addition shouldn't
/// make `ptc inspect` itself fail to run.
#[derive(serde::Deserialize)]
struct Event {
    date: String,
    #[serde(rename = "type")]
    event_type: String,
    agent: String,
}

pub fn inspect(root: &Path) -> Result<InspectReport> {
    let capsule_yaml_path = root.join("capsule.yaml");
    let capsule = model::read_capsule_yaml(&capsule_yaml_path).with_context(|| {
        format!(
            "{} must exist and parse -- is {} a capsule directory?",
            capsule_yaml_path.display(),
            root.display()
        )
    })?;

    let data_dir = root.join("data");
    let mut covered_dirs: Vec<PathBuf> = vec![data_dir.join("person"), data_dir.join("provenance")];

    let mut collections = Vec::new();
    for c in capsule.collections.iter().flatten() {
        let dir = root.join(&c.path);
        covered_dirs.push(dir.clone());
        let (file_count, total_bytes) = if dir.is_dir() {
            count_files(&dir)?
        } else {
            (0, 0)
        };
        collections.push(CollectionReport {
            name: c.name.clone(),
            path: c.path.clone(),
            exists: dir.is_dir(),
            file_count,
            total_bytes,
            privacy_zone: describe_privacy_zone(
                c.privacy_zone,
                capsule.access.default_privacy_zone,
            ),
            release_policy: match &c.release_policy {
                Some(p) => describe_release_policy(p, false),
                None => describe_release_policy(&capsule.access.default_release_policy, true),
            },
        });
    }

    let undeclared_directories = collect_undeclared_directories(&data_dir, &covered_dirs)?;

    let person_dir = data_dir.join("person");
    let person_layer = PersonLayerReport {
        biography_present: person_dir.join("biography.md").is_file(),
        timeline_present: person_dir.join("timeline.md").is_file(),
        people_count: count_yaml_list(&person_dir.join("people.yaml")),
        places_count: count_yaml_list(&person_dir.join("places.yaml")),
    };

    let provenance = read_provenance(&data_dir.join("provenance").join("events.yaml"))?;

    let privacy_overrides_count = count_yaml_list(&root.join("privacy-overrides.yaml"));

    let legal_dir = root.join("legal");
    let legal_files = if legal_dir.is_dir() {
        let mut files: Vec<String> = fs::read_dir(&legal_dir)
            .with_context(|| format!("reading {}", legal_dir.display()))?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        files.sort();
        Some(files)
    } else {
        None
    };

    let preservation_log_entries = count_yaml_list(&root.join("preservation-log.yaml"));

    let (total_payload_files, total_payload_bytes) = if data_dir.is_dir() {
        count_files(&data_dir)?
    } else {
        (0, 0)
    };

    Ok(InspectReport {
        capsule,
        collections,
        undeclared_directories,
        person_layer,
        provenance,
        privacy_overrides_count,
        legal_files,
        preservation_log_entries,
        manifest_present: root.join("manifest-sha256.txt").is_file(),
        tagmanifest_present: root.join("tagmanifest-sha256.txt").is_file(),
        total_payload_files,
        total_payload_bytes,
    })
}

/// Top-level directories under `data/` that aren't `person`/`provenance`
/// (structural, not `collections[]` entries) and aren't any declared
/// collection's own directory -- material that landed under `data/`
/// without going through `ptc import` (e.g. hand-copied), so it's
/// invisible to `capsule.yaml` and won't get a privacy zone or release
/// policy from it.
fn collect_undeclared_directories(
    data_dir: &Path,
    covered_dirs: &[PathBuf],
) -> Result<Vec<DirectoryReport>> {
    if !data_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut entries: Vec<_> = fs::read_dir(data_dir)
        .with_context(|| format!("reading {}", data_dir.display()))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let mut result = Vec::new();
    for entry in entries {
        let dir = entry.path();
        if covered_dirs.contains(&dir) {
            continue;
        }
        let (file_count, total_bytes) = count_files(&dir)?;
        result.push(DirectoryReport {
            name: entry.file_name().to_string_lossy().to_string(),
            file_count,
            total_bytes,
        });
    }
    Ok(result)
}

fn read_provenance(events_path: &Path) -> Result<Option<ProvenanceReport>> {
    if !events_path.is_file() {
        return Ok(None);
    }
    let text = fs::read_to_string(events_path)
        .with_context(|| format!("reading {}", events_path.display()))?;
    let events: Vec<Event> = yaml_serde::from_str(&text)
        .with_context(|| format!("parsing {}", events_path.display()))?;
    // Append-only and ordered per METADATA.md, so the last entry is the
    // most recent -- no need to sort by date.
    let most_recent = events.last().map(|e| EventLine {
        date: e.date.clone(),
        event_type: e.event_type.clone(),
        agent: e.agent.clone(),
    });
    Ok(Some(ProvenanceReport {
        event_count: events.len(),
        most_recent,
    }))
}

fn count_files(dir: &Path) -> Result<(usize, u64)> {
    let mut count = 0usize;
    let mut bytes = 0u64;
    for entry in WalkDir::new(dir) {
        let entry = entry.with_context(|| format!("walking {}", dir.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }
        bytes += fs::metadata(entry.path())
            .with_context(|| format!("reading metadata for {}", entry.path().display()))?
            .len();
        count += 1;
    }
    Ok((count, bytes))
}

/// `None` if the file doesn't exist, can't be read, or doesn't parse as a
/// YAML list -- lenient like `Event` above, since this is a report, not a
/// validator (a structural check belongs to `ptc verify`, not yet built).
fn count_yaml_list(path: &Path) -> Option<usize> {
    let text = fs::read_to_string(path).ok()?;
    let value: Vec<yaml_serde::Value> = yaml_serde::from_str(&text).ok()?;
    Some(value.len())
}

fn describe_privacy_zone(zone: Option<PrivacyZone>, default: PrivacyZone) -> String {
    match zone {
        Some(z) => privacy_zone_str(z).to_string(),
        None => format!("{} (capsule default)", privacy_zone_str(default)),
    }
}

fn privacy_zone_str(zone: PrivacyZone) -> &'static str {
    match zone {
        PrivacyZone::Public => "public",
        PrivacyZone::Restricted => "restricted",
        PrivacyZone::NeverRelease => "never-release",
    }
}

fn describe_release_policy(policy: &ReleasePolicy, is_default: bool) -> String {
    let base = match policy.policy_type {
        ReleasePolicyType::Immediate => "immediate".to_string(),
        ReleasePolicyType::FixedDate => {
            format!("fixed_date ({})", policy.date.as_deref().unwrap_or("?"))
        }
        ReleasePolicyType::AfterDeath => "after_death".to_string(),
        ReleasePolicyType::AfterDeathPlusYears => {
            format!("after_death_plus_years (+{}y)", policy.years.unwrap_or(0))
        }
        ReleasePolicyType::Never => "never".to_string(),
        ReleasePolicyType::Custom => match &policy.description {
            Some(d) => format!("custom: {d}"),
            None => "custom".to_string(),
        },
    };
    if is_default {
        format!("{base} (capsule default)")
    } else {
        base
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        Access, Capsule, CapsuleIdentity, Collection, Integrity, LifecycleStage, ManifestAlgorithm,
        Status, SPEC_VERSION,
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
            collections,
            access: Access {
                default_privacy_zone: PrivacyZone::Public,
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
                lifecycle_stage: LifecycleStage::Import,
            },
        };
        model::write_capsule_yaml(&root.join("capsule.yaml"), &capsule, "").unwrap();
    }

    #[test]
    fn reports_collection_sizes_and_resolves_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(
            root,
            Some(vec![
                Collection {
                    name: "photos".to_string(),
                    path: "data/photos".to_string(),
                    privacy_zone: None,
                    release_policy: None,
                    encryption: None,
                },
                Collection {
                    name: "email".to_string(),
                    path: "data/email".to_string(),
                    privacy_zone: Some(PrivacyZone::Restricted),
                    release_policy: Some(ReleasePolicy {
                        policy_type: ReleasePolicyType::AfterDeathPlusYears,
                        date: None,
                        years: Some(50),
                        description: None,
                    }),
                    encryption: None,
                },
            ]),
        );

        fs::create_dir_all(root.join("data").join("photos")).unwrap();
        fs::write(root.join("data").join("photos").join("a.jpg"), b"12345").unwrap();
        fs::write(root.join("data").join("photos").join("b.jpg"), b"1234567").unwrap();
        // "email" collection declared but nothing on disk yet.

        let report = inspect(root).unwrap();
        assert_eq!(report.collections.len(), 2);

        let photos = &report.collections[0];
        assert_eq!(photos.file_count, 2);
        assert_eq!(photos.total_bytes, 12);
        assert!(photos.exists);
        assert_eq!(photos.privacy_zone, "public (capsule default)");
        assert_eq!(photos.release_policy, "immediate (capsule default)");

        let email = &report.collections[1];
        assert!(!email.exists);
        assert_eq!(email.file_count, 0);
        assert_eq!(email.privacy_zone, "restricted");
        assert_eq!(email.release_policy, "after_death_plus_years (+50y)");
    }

    #[test]
    fn detects_undeclared_directory_but_not_person_or_provenance() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, None);

        fs::create_dir_all(root.join("data").join("person")).unwrap();
        fs::create_dir_all(root.join("data").join("provenance")).unwrap();
        let mystery_dir = root.join("data").join("mystery");
        fs::create_dir_all(&mystery_dir).unwrap();
        fs::write(mystery_dir.join("x.bin"), b"abc").unwrap();

        let report = inspect(root).unwrap();
        assert_eq!(report.undeclared_directories.len(), 1);
        assert_eq!(report.undeclared_directories[0].name, "mystery");
        assert_eq!(report.undeclared_directories[0].file_count, 1);
        assert_eq!(report.undeclared_directories[0].total_bytes, 3);
    }

    #[test]
    fn summarizes_person_layer() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, None);

        let person_dir = root.join("data").join("person");
        fs::create_dir_all(&person_dir).unwrap();
        fs::write(person_dir.join("biography.md"), "# Biography\n").unwrap();
        fs::write(
            person_dir.join("people.yaml"),
            "- id: a\n  relationship: spouse\n- id: b\n  relationship: son\n",
        )
        .unwrap();

        let report = inspect(root).unwrap();
        assert!(report.person_layer.biography_present);
        assert!(!report.person_layer.timeline_present);
        assert_eq!(report.person_layer.people_count, Some(2));
        assert_eq!(report.person_layer.places_count, None);
    }

    #[test]
    fn summarizes_provenance_using_the_last_event_as_most_recent() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, None);

        let provenance_dir = root.join("data").join("provenance");
        fs::create_dir_all(&provenance_dir).unwrap();
        fs::write(
            provenance_dir.join("events.yaml"),
            "- date: \"2026-08-25\"\n  type: origination\n  agent: \"Jane Smith (creator)\"\n\n\
             - date: \"2026-08-30\"\n  type: import\n  agent: \"ptc CLI v0.1\"\n",
        )
        .unwrap();

        let report = inspect(root).unwrap();
        let provenance = report.provenance.expect("provenance report");
        assert_eq!(provenance.event_count, 2);
        let most_recent = provenance.most_recent.expect("most recent event");
        assert_eq!(most_recent.date, "2026-08-30");
        assert_eq!(most_recent.event_type, "import");
        assert_eq!(most_recent.agent, "ptc CLI v0.1");
    }

    #[test]
    fn detects_optional_tag_files_and_sealed_manifests() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_test_capsule(root, None);

        fs::write(root.join("privacy-overrides.yaml"), "- path: data/a\n").unwrap();
        fs::create_dir_all(root.join("legal")).unwrap();
        fs::write(root.join("legal").join("deed-of-gift.md"), "...").unwrap();
        fs::write(root.join("manifest-sha256.txt"), "abc  data/a\n").unwrap();

        let report = inspect(root).unwrap();
        assert_eq!(report.privacy_overrides_count, Some(1));
        assert_eq!(
            report.legal_files,
            Some(vec!["deed-of-gift.md".to_string()])
        );
        assert_eq!(report.preservation_log_entries, None);
        assert!(report.manifest_present);
        assert!(!report.tagmanifest_present);
    }

    #[test]
    fn bails_when_capsule_yaml_missing() {
        let dir = tempfile::tempdir().unwrap();
        assert!(inspect(dir.path()).is_err());
    }
}
