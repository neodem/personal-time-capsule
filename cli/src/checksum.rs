//! SHA-256 file checksums, matching STANDARDS.md's SHA-256 entry and the
//! `<lowercase-hex>  <path>` line format `examples/minimal-capsule/`'s
//! manifests already use (the same format `sha256sum --text` produces).

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Compute the lowercase-hex SHA-256 digest of the file at `path`, reading
/// it in fixed-size chunks rather than loading it fully into memory (a
/// capsule can contain multi-gigabyte video files).
pub fn sha256_file(path: &Path) -> Result<String> {
    let file = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = reader
            .read(&mut buf)
            .with_context(|| format!("reading {}", path.display()))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// Format one manifest line the way `manifest-sha256.txt`/`tagmanifest-sha256.txt`
/// already do throughout this project: lowercase hex hash, two spaces, then
/// the path relative to the bag root, using forward slashes regardless of
/// platform (BagIt manifests are cross-platform text files).
pub fn manifest_line(hash: &str, relative_path: &str) -> String {
    format!("{}  {}", hash, relative_path.replace('\\', "/"))
}

/// Parse one manifest line back into `(hash, relative_path)` -- the
/// inverse of `manifest_line`, used by `ptc verify` to read
/// `manifest-sha256.txt`/`tagmanifest-sha256.txt`. `None` for anything
/// that doesn't have the two-space-separated shape every manifest this
/// project writes always has.
pub fn parse_manifest_line(line: &str) -> Option<(String, String)> {
    let (hash, path) = line.split_once("  ")?;
    if hash.is_empty() || path.is_empty() {
        return None;
    }
    Some((hash.to_string(), path.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn hashes_known_content() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("hello.txt");
        let mut f = std::fs::File::create(&path).expect("create");
        f.write_all(b"hello world").expect("write");
        drop(f);

        let hash = sha256_file(&path).expect("hash");
        // sha256("hello world"), a well-known test vector (no trailing newline).
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn manifest_line_uses_forward_slashes_and_two_spaces() {
        let line = manifest_line("abc123", "data\\photos\\a.jpg");
        assert_eq!(line, "abc123  data/photos/a.jpg");
    }

    #[test]
    fn parse_manifest_line_is_the_inverse_of_manifest_line() {
        let line = manifest_line("abc123", "data/photos/a.jpg");
        assert_eq!(
            parse_manifest_line(&line),
            Some(("abc123".to_string(), "data/photos/a.jpg".to_string()))
        );
    }

    #[test]
    fn parse_manifest_line_rejects_malformed_input() {
        assert_eq!(parse_manifest_line("not a manifest line"), None);
        assert_eq!(parse_manifest_line(""), None);
    }
}
