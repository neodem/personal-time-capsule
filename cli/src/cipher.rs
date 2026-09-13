//! A small seam between `ptc encrypt`/`ptc unlock` and whatever concrete
//! AEAD cipher backs them, so the two never call a crypto crate's API
//! directly.
//!
//! `ENCRYPTION.md` locks `capsule.yaml`'s `encryption.algorithm` to the
//! single value `"xchacha20-poly1305"`, deliberately -- creator-facing
//! crypto-agility is called out there as a downgrade-attack and
//! maintenance risk, not something this project wants. That decision is
//! about the *format* (what a capsule can declare), not this codebase's
//! *internals*: nothing stops a future `spec_version` bump from retiring
//! XChaCha20-Poly1305 for a different vetted algorithm, and when that
//! happens it should mean writing one new `Cipher` impl and changing
//! `default_cipher`'s one line, not re-deriving `encrypt.rs`'s
//! file-walking, key-management, and error-handling logic from scratch
//! around a new crate's API shape.

use anyhow::{anyhow, bail, Result};
use chacha20poly1305::aead::{Aead, Generate, KeyInit};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};

/// A symmetric AEAD (authenticated encryption) cipher used to encrypt one
/// capsule payload file at a time. Implementations own every detail of
/// their own on-disk representation (nonce size and placement, additional
/// authenticated data, anything else) -- callers pass plaintext in and
/// get back a self-contained blob `decrypt` can later reverse given only
/// the same key, never touching nonces or other cipher internals
/// themselves.
pub trait Cipher {
    /// The `capsule.yaml`/`schema/capsule.schema.json`
    /// `encryption.algorithm` value this implementation corresponds to.
    fn algorithm_id(&self) -> &'static str;

    /// Key length in bytes `generate_key` produces and `encrypt`/`decrypt`
    /// expect.
    fn key_len(&self) -> usize;

    /// Generate a fresh, random key suitable for one encryption unit
    /// (`ENCRYPTION.md`: one key per collection, never reused across
    /// units, never derived from a passphrase).
    fn generate_key(&self) -> Vec<u8>;

    fn encrypt(&self, key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>>;

    /// Fails (rather than returning corrupted output) if `key` is wrong or
    /// `ciphertext` was truncated/tampered with -- an AEAD's whole point.
    fn decrypt(&self, key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>>;
}

/// The one algorithm `ENCRYPTION.md` currently allows. Swapping this for a
/// different `Cipher` impl later is the one line this function exists to
/// isolate.
pub fn default_cipher() -> Box<dyn Cipher> {
    Box::new(XChaCha20Poly1305Cipher)
}

pub struct XChaCha20Poly1305Cipher;

const KEY_LEN: usize = 32; // 256-bit, per ENCRYPTION.md
const NONCE_LEN: usize = 24; // 192-bit, per ENCRYPTION.md

impl Cipher for XChaCha20Poly1305Cipher {
    fn algorithm_id(&self) -> &'static str {
        "xchacha20-poly1305"
    }

    fn key_len(&self) -> usize {
        KEY_LEN
    }

    fn generate_key(&self) -> Vec<u8> {
        Key::generate().to_vec()
    }

    fn encrypt(&self, key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
        let cipher = build_cipher(key)?;
        // Randomly generated per file, per ENCRYPTION.md -- XChaCha20's
        // 192-bit nonce is exactly what makes this safe to do without any
        // counter or cross-file coordination.
        let nonce = XNonce::generate();
        let ciphertext = cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| anyhow!("encryption failed: {e}"))?;

        let mut out = Vec::with_capacity(NONCE_LEN + ciphertext.len());
        out.extend_from_slice(&nonce);
        out.extend_from_slice(&ciphertext);
        Ok(out)
    }

    fn decrypt(&self, key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < NONCE_LEN {
            bail!("ciphertext is too short to contain a nonce -- truncated or not actually ciphertext");
        }
        let cipher = build_cipher(key)?;
        let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);
        let nonce = XNonce::try_from(nonce_bytes)
            .map_err(|_| anyhow!("malformed nonce (wrong length) in ciphertext"))?;
        cipher.decrypt(&nonce, ciphertext).map_err(|e| {
            anyhow!("decryption failed -- wrong key, or the ciphertext was corrupted or tampered with: {e}")
        })
    }
}

fn build_cipher(key: &[u8]) -> Result<XChaCha20Poly1305> {
    if key.len() != KEY_LEN {
        bail!("key must be {KEY_LEN} bytes, got {}", key.len());
    }
    let key = Key::try_from(key).map_err(|_| anyhow!("malformed key (wrong length)"))?;
    Ok(XChaCha20Poly1305::new(&key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_plaintext() {
        let cipher = default_cipher();
        let key = cipher.generate_key();
        let plaintext = b"a payload file's actual bytes";

        let ciphertext = cipher.encrypt(&key, plaintext).unwrap();
        assert_ne!(ciphertext, plaintext);

        let decrypted = cipher.decrypt(&key, &ciphertext).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn same_plaintext_encrypts_differently_each_time() {
        // Confirms the nonce is actually random per call, not fixed --
        // the property ENCRYPTION.md relies on to call random generation
        // safe with no counter.
        let cipher = default_cipher();
        let key = cipher.generate_key();
        let plaintext = b"identical content";

        let a = cipher.encrypt(&key, plaintext).unwrap();
        let b = cipher.encrypt(&key, plaintext).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn wrong_key_fails_to_decrypt() {
        let cipher = default_cipher();
        let key = cipher.generate_key();
        let wrong_key = cipher.generate_key();
        let ciphertext = cipher.encrypt(&key, b"secret").unwrap();

        assert!(cipher.decrypt(&wrong_key, &ciphertext).is_err());
    }

    #[test]
    fn tampered_ciphertext_fails_to_decrypt() {
        let cipher = default_cipher();
        let key = cipher.generate_key();
        let mut ciphertext = cipher.encrypt(&key, b"secret").unwrap();
        let last = ciphertext.len() - 1;
        ciphertext[last] ^= 0xFF;

        assert!(cipher.decrypt(&key, &ciphertext).is_err());
    }

    #[test]
    fn rejects_a_wrong_length_key() {
        let cipher = default_cipher();
        let short_key = vec![0u8; 16];
        assert!(cipher.encrypt(&short_key, b"secret").is_err());
    }
}
