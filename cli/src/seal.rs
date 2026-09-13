//! `ptc seal` -- LIFECYCLE.md stage 5. Computes `manifest-sha256.txt` over
//! `data/`, writes `bag-info.txt`, confirms `bagit.txt`, computes
//! `tagmanifest-sha256.txt` over the known tag files that exist, and marks
//! `capsule.yaml`'s `status.lifecycle_stage` as `sealing`.
//!
//! ROADMAP.md item 25a -- no command for this LIFECYCLE stage existed
//! before; `ptc verify`/`ptc encrypt`/`ptc deposit` all assume a manifest
//! that this command is what actually produces.

use crate::checksum::{manifest_line, sha256_file};
use crate::model::{self, ReleasePolicyType};
use anyhow::{bail, Context, Result};
use chrono::Utc;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Tag files considered for `tagmanifest-sha256.txt`, in a fixed order,
/// followed by everything under `legal/` (sorted). Only files that
/// actually exist are included -- `privacy-overrides.yaml` and `legal/`
/// are both optional per FORMAT.md/PRIVACY.md/DEED_OF_GIFT.md.
const FIXED_TAG_FILES: &[&str] = &[
    "bagit.txt",
    "bag-info.txt",
    "capsule.yaml",
    "privacy-overrides.yaml",
];

pub struct SealSummary {
    pub payload_files_hashed: usize,
    pub payload_bytes: u64,
    pub tag_files_hashed: Vec<String>,
}

pub fn seal(root: &Path) -> Result<SealSummary> {
    let capsule_yaml_path = root.join("capsule.yaml");
    let capsule = model::read_capsule_yaml(&capsule_yaml_path).with_context(|| {
        format!(
            "{} must exist and parse before sealing",
            capsule_yaml_path.display()
        )
    })?;

    guard_against_placeholder_release_policy(&capsule.access.default_release_policy)?;

    let data_dir = root.join("data");
    if !data_dir.is_dir() {
        bail!("{} does not exist -- nothing to seal", data_dir.display());
    }

    let (manifest_lines, payload_bytes, payload_files_hashed) = hash_payload(root, &data_dir)?;
    fs::write(
        root.join("manifest-sha256.txt"),
        lines_to_file(&manifest_lines),
    )
    .context("writing manifest-sha256.txt")?;

    ensure_bagit_txt(root)?;
    write_bag_info_txt(root, &capsule, payload_bytes, payload_files_hashed)?;

    // capsule.yaml must be patched to its final on-disk content BEFORE it's
    // hashed for tagmanifest-sha256.txt below -- otherwise the recorded
    // checksum would describe the pre-patch content while the file itself
    // ends up patched afterward, and `sha256sum -c` would (correctly) call
    // that a mismatch. Caught by manual `sha256sum -c` verification against
    // a real sealed capsule while building this -- see the integration
    // test's own independent-hash assertions, added after the fact.
    model::patch_lifecycle_stage(&capsule_yaml_path, "sealing")?;

    let tag_files_hashed = hash_tag_files(root)?;
    let tag_lines: Vec<String> = tag_files_hashed
        .iter()
        .map(|(rel, hash)| manifest_line(hash, rel))
        .collect();
    fs::write(
        root.join("tagmanifest-sha256.txt"),
        lines_to_file(&tag_lines),
    )
    .context("writing tagmanifest-sha256.txt")?;

    Ok(SealSummary {
        payload_files_hashed,
        payload_bytes,
        tag_files_hashed: tag_files_hashed.into_iter().map(|(rel, _)| rel).collect(),
    })
}

fn guard_against_placeholder_release_policy(policy: &model::ReleasePolicy) -> Result<()> {
    if policy.policy_type == ReleasePolicyType::Custom {
        if let Some(desc) = &policy.description {
            if desc.contains("TBD") {
                bail!(
                    "capsule.yaml's access.default_release_policy is still `ptc init`'s placeholder \
                     (type: custom, description containing \"TBD\"). Set a real release policy before \
                     sealing -- see FORMAT.md's `access` section for the available types."
                );
            }
        }
    }
    Ok(())
}

/// Walk `data/`, hash every file, and return manifest lines sorted by
/// relative path (not by hash -- sorting by the formatted line would sort
/// by hash prefix first, which isn't useful to a human reading the file).
fn hash_payload(root: &Path, data_dir: &Path) -> Result<(Vec<String>, u64, usize)> {
    let mut entries: Vec<(String, String, u64)> = Vec::new(); // (relative_path, hash, size)

    for entry in WalkDir::new(data_dir) {
        let entry = entry.with_context(|| format!("walking {}", data_dir.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }
        let abs = entry.path();
        let rel = abs
            .strip_prefix(root)
            .with_context(|| format!("computing a path relative to {}", root.display()))?
            .to_string_lossy()
            .replace('\\', "/");
        let hash = sha256_file(abs)?;
        let size = fs::metadata(abs)
            .with_context(|| format!("reading metadata for {}", abs.display()))?
            .len();
        entries.push((rel, hash, size));
    }

    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let total_bytes: u64 = entries.iter().map(|(_, _, size)| size).sum();
    let count = entries.len();
    let lines = entries
        .iter()
        .map(|(rel, hash, _)| manifest_line(hash, rel))
        .collect();
    Ok((lines, total_bytes, count))
}

fn ensure_bagit_txt(root: &Path) -> Result<()> {
    let path = root.join("bagit.txt");
    if !path.exists() {
        fs::write(
            &path,
            "BagIt-Version: 1.0\nTag-File-Character-Encoding: UTF-8\n",
        )
        .context("writing bagit.txt")?;
    }
    Ok(())
}

fn write_bag_info_txt(
    root: &Path,
    capsule: &model::Capsule,
    payload_bytes: u64,
    payload_file_count: usize,
) -> Result<()> {
    let mut contents = String::new();
    contents.push_str(&format!(
        "Bagging-Date: {}\n",
        Utc::now().format("%Y-%m-%d")
    ));
    contents.push_str(&format!(
        "Payload-Oxum: {}.{}\n",
        payload_bytes, payload_file_count
    ));
    contents.push_str(&format!(
        "Bag-Software-Agent: ptc CLI v{}\n",
        env!("CARGO_PKG_VERSION")
    ));
    contents.push_str(&format!("External-Identifier: {}\n", capsule.capsule.id));
    if let Some(purpose) = &capsule.purpose {
        if let Some(description) = &purpose.description {
            contents.push_str(&format!(
                "External-Description: {}\n",
                description.replace('\n', " ")
            ));
        }
    }
    fs::write(root.join("bag-info.txt"), contents).context("writing bag-info.txt")
}

/// Hash whichever known tag files actually exist: the fixed list in a
/// stable order, then everything under `legal/`, sorted by filename.
fn hash_tag_files(root: &Path) -> Result<Vec<(String, String)>> {
    let mut result = Vec::new();

    for rel in FIXED_TAG_FILES {
        let abs = root.join(rel);
        if abs.is_file() {
            result.push((rel.to_string(), sha256_file(&abs)?));
        }
    }

    let legal_dir = root.join("legal");
    if legal_dir.is_dir() {
        let mut legal_rel: Vec<String> = fs::read_dir(&legal_dir)
            .with_context(|| format!("reading {}", legal_dir.display()))?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| format!("legal/{}", e.file_name().to_string_lossy()))
            .collect();
        legal_rel.sort();
        for rel in legal_rel {
            let abs = root.join(&rel);
            result.push((rel, sha256_file(&abs)?));
        }
    }

    Ok(result)
}

fn lines_to_file(lines: &[String]) -> String {
    if lines.is_empty() {
        String::new()
    } else {
        format!("{}\n", lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_placeholder_release_policy() {
        let policy = model::ReleasePolicy {
            policy_type: ReleasePolicyType::Custom,
            date: None,
            years: None,
            description: Some("TBD -- set by the creator".to_string()),
        };
        assert!(guard_against_placeholder_release_policy(&policy).is_err());
    }

    #[test]
    fn accepts_real_release_policy() {
        let policy = model::ReleasePolicy {
            policy_type: ReleasePolicyType::FixedDate,
            date: Some("2076-08-20".to_string()),
            years: None,
            description: None,
        };
        assert!(guard_against_placeholder_release_policy(&policy).is_ok());
    }
}
