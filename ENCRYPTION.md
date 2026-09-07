# Encryption Model (draft)

Phase 2 (Specification) — [`ROADMAP.md`](./ROADMAP.md) item 13: the algorithm and key-splitting scheme `capsule.yaml`'s `encryption` section deliberately left unstubbed — see [`FORMAT.md`](./FORMAT.md). Implements [`PRINCIPLES.md`](./PRINCIPLES.md) #8 (preservation and access are different things) and responds to [`THREAT_MODEL.md`](./THREAT_MODEL.md) #4 (unauthorized early access), #6 (coercion), and #7 (loss of legitimate access). A worked, filled-in `encryption` block lives in [`examples/capsule.yaml`](./examples/capsule.yaml).

## What already existed

`capsule.yaml`'s `encryption` section declared the *shape* of key custody — `status` (`planned`/`encrypted`/`none`) and a `quorum` (`required`-of-`of` trustees) — without naming an actual algorithm or secret-sharing mechanism. This document names both.

## Algorithm: XChaCha20-Poly1305

A symmetric, authenticated-encryption (AEAD) cipher: it provides confidentiality *and* built-in tamper detection (a forged or corrupted ciphertext fails to decrypt) in one primitive, the same property SHA-256-based manifests give the payload's integrity, just for content that also needs to stay secret until release.

**Why this over AES-256-GCM**, the other well-studied, widely-used AEAD option: the deciding factor is nonce-misuse safety, not raw popularity. AES-GCM's 96-bit nonce creates a real collision risk if a future, unaudited reimplementation generates nonces randomly instead of carefully managing a counter — and nonce reuse under AES-GCM doesn't just weaken confidentiality, it silently destroys the tamper-detection guarantee too. XChaCha20-Poly1305's extended 192-bit nonce makes random generation safe indefinitely, removing an entire, well-documented class of catastrophic implementation mistakes. That matters more here than in most projects: this spec has to stay correctly implementable by people who aren't this project's own contributors, possibly decades from now, with no dedicated cryptographer reviewing their code (`THREAT_MODEL.md` #10 — capsule validity can't depend on trusting any particular binary, including ours).

The usual counter-argument — AES-GCM is in every language's standard library, XChaCha20-Poly1305 often needs libsodium — carries less weight for this project's own reference tooling specifically: this project's reference CLI is Rust (`cli/`), and in Rust **neither** algorithm is in the standard library (Rust's `std` has no cryptography at all). Both come from the same RustCrypto project as pure-Rust crates with no C dependency — `aes-gcm` and `chacha20poly1305` — equally trivial to add, equally maintained. That neutralizes AES-GCM's main practical advantage for this project's own implementation, leaving XChaCha20-Poly1305's safety margin as the deciding factor. (A future non-Rust implementation may find libsodium a heavier dependency than its language's built-in AES-GCM support — a real cost, acknowledged, not dismissed; it was outweighed here.)

- **Key size:** 256-bit, generated fresh per encryption unit (see "Encryption granularity," below) — never derived from a passphrase directly.
- **Nonce:** 192-bit, generated randomly per file; safe to do so without any counter or coordination.
- `capsule.yaml`'s `encryption.algorithm` field is locked to the single value `"xchacha20-poly1305"` (see `schema/capsule.schema.json`) — deliberately not creator-selectable. Crypto-agility (letting each capsule pick its own algorithm) is a well-known source of long-term maintenance and downgrade-attack risk; one vetted default, changeable only via a future `spec_version` bump, is simpler to implement correctly and to verify independently.

## Key-splitting: SLIP-0039

The mechanism behind `key_custody`'s `quorum` (`required`-of-`of` trustees): [SLIP-0039](https://github.com/satoshilabs/slips/blob/master/slip-0039.md), a fully-specified, testable standard for splitting a secret into shares via Shamir's Secret Sharing, with a share-encoding layer purpose-built for exactly this project's situation — a secret held by non-technical people, for potentially decades, recoverable independently of any software.

**Why this over raw/unencoded Shamir's Secret Sharing:** the underlying math is identical either way — SLIP-0039's actual advantage is the share format. A SLIP-0039 share is a checksum-protected sequence of common words (e.g. `"academic acid academic easy ..."`), not a raw hex or base64 blob. A trustee can transcribe or read theirs aloud without a computer, and a mistyped word is caught immediately by the built-in checksum rather than silently corrupting the share — discovered only when reconstruction is attempted, potentially decades later, with the person who could've caught the mistake long gone. For a scheme whose entire premise is "an ordinary person holds this safely for a very long time" (`ACTORS.md`'s key-holding trustee role), that self-checking property is worth more than the marginally simpler reimplementation of raw Shamir's Secret Sharing.

SLIP-0039 was originally designed for cryptocurrency wallet backup — its tooling and documentation lean that direction — but the mechanism itself (split a secret, encode shares as checksummed word sequences) is domain-agnostic. This project uses only its simple *k*-of-*n* threshold feature, matching `key_custody.quorum.required`/`.of`; SLIP-0039's more elaborate group-of-groups structure (e.g. "2-of-3 family members, OR any 1 lawyer") isn't used here, though nothing here would need to change to adopt it later if a real capsule's custody needs turn out to require it.

- `capsule.yaml`'s `key_custody.splitting_scheme` field is locked to the single value `"slip-0039"` (see `schema/capsule.schema.json`), for the same crypto-agility reasoning as `algorithm`.
- Each share can optionally be passphrase-protected — SLIP-0039 supports this natively. Whether to use one is the creator's (or trustee's) call, per `PRINCIPLES.md` #2, not something this spec mandates.

## Trustee records

`key_custody.trustees` is no longer an unconstrained array — each entry now has a defined (if minimal) shape: a required `name`, optional `contact`, and optional `share_index` (which numbered share, out of `.of`, this trustee holds — bookkeeping only, e.g. so a creator can tell who to contact if share 3 needs replacing).

**The actual share — the SLIP-0039 word sequence itself — never appears in `capsule.yaml`, or anywhere else in the bag.** `capsule.yaml` is checksummed, deposited, and potentially replicated across multiple custodians; it has to be safe to treat as effectively public. Share material lives only with each trustee, by whatever physical or digital means they choose, entirely outside the capsule's own structure.

## What actually gets encrypted, and how

Per-file, in place, under `data/` — a payload file's plaintext bytes are replaced with its XChaCha20-Poly1305 ciphertext (convention: append `.enc` to the filename, so it's visually obvious at a glance which files are locked). This preserves the directory structure and per-item addressability `privacy-overrides.yaml` (`PRIVACY.md`, item 12) already depends on, rather than bundling a whole collection into one opaque encrypted archive.

**Anything with `release_policy.type: immediate` is never encrypted.** There's no reason to cryptographically lock content the creator has already decided should be available right away — encryption exists to enforce a *wait*, not to gate access to public, immediately-releasable material. Everything with any other `release_policy.type` (`fixed_date`, `after_death`, `after_death_plus_years`, `never`, `custom`) gets encrypted.

## Encryption granularity: one key per collection, not one key per capsule

`capsule.yaml`'s top-level `encryption` block covers the capsule-wide default. But `PRIVACY.md` and `FORMAT.md` already established that `release_policy` can be overridden per collection — a restricted email collection releasing 50 years after death, alongside a never-release journal, in the same capsule. If both were encrypted under one shared key and quorum, unlocking the email collection at its release date would hand trustees the ability to decrypt the journal too, regardless of what its `release_policy` says. That would make the journal's "never" policy a label, not a real guarantee.

So: **`collections[].encryption`** (see `schema/capsule.schema.json`) lets a collection declare its own full `encryption` block — its own key, algorithm, and trustee quorum — the same full-replacement-not-merge convention `privacy_zone` and `release_policy` overrides already use. In practice, every distinct `release_policy` present in a capsule (beyond `immediate`) should generally get its own collection with its own `encryption` override, so unlocking one doesn't imply unlocking another.

**Known limitation, not solved here:** an *item-level* override in `privacy-overrides.yaml` that changes a single item's `release_policy` does **not** get its own cryptographic isolation — it's encrypted under its collection's key, same as every other item in that collection. Its distinct `release_policy` is a recorded intent, enforceable procedurally (a custodian instructed not to hand over that specific item even after the collection's quorum unlocks) but not cryptographically, at least in this version. A creator who needs one item to be *genuinely* unrecoverable independent of the rest of its collection should give it its own collection instead. `PRIVACY.md` is cross-referenced with this limitation.

## A design property worth naming: no public-key cryptography anywhere in this flow

XChaCha20-Poly1305 is symmetric; SLIP-0039 splits a symmetric key via Shamir's Secret Sharing, also a purely symmetric/information-theoretic mechanism. Nothing in this design uses RSA, elliptic-curve key exchange, or any other public-key primitive. That matters for a project explicitly designed around 50-100 year timelines: the "harvest now, decrypt later" quantum-computing threat that worries most long-lived encrypted systems is almost entirely a threat to *public-key key exchange* (broken completely by Shor's algorithm), not to well-chosen symmetric ciphers with large keys (only quadratically weakened by Grover's algorithm — a 256-bit key stays enormously strong even against that). By never introducing public-key crypto into the core encryption/key-custody flow, this design sidesteps most of that threat by construction, not by promise — a small, genuine point in favor of `PRINCIPLES.md` #9 (no brittle promises).

## How this interacts with sealing and fixity (a correction to items 10 and 11)

Writing this document surfaced a real inconsistency in the two schemas it builds on. `LIFECYCLE.md` orders **sealing (stage 5) before encryption (stage 6)** — meaning `data/provenance/events.yaml` (`METADATA.md`, item 10) is already checksummed and frozen by the time an encryption event exists to log. `schema/events.schema.json` had mistakenly allowed `encryption` (and, in fact, every post-seal stage type) in its `type` enum since item 10 was written. That's now corrected: `events.yaml`'s allowed types are the 5 pre-seal stages only, and `encryption` has been added to `preservation-log.yaml`'s type enum (`INTEGRITY.md`, item 11) where it belongs — see both documents' "Event types" sections.

This also clarifies what `manifest-sha256.txt` actually means for an encrypted capsule. It's generated at sealing, over *plaintext* payload. Once encryption (stage 6) replaces those files with ciphertext, the on-disk bytes no longer match the manifest for encrypted files — expected, not a bug: the manifest describes the capsule's true logical content, fully re-verifiable only after unlock. In the meantime, XChaCha20-Poly1305's own authentication tag is what protects the ciphertext from undetected tampering, and a custodian's own periodic fixity checks (`INTEGRITY.md`) against encrypted content are necessarily a "have these bytes changed since I received them" check — a baseline the custodian can establish themselves at deposit — not a full manifest-verification check, until the content is eventually decrypted.

## What's deliberately out of scope here

- **Implementation.** `ptc encrypt` and `ptc unlock` (`ROADMAP.md` items 26, 28) don't exist yet — this document specifies what they need to produce, not how they're built.
- **Passphrase guidance for share protection.** Left to the creator and trustees; future tooling can prompt for one without this spec mandating it.
- **Item-level cryptographic isolation.** See "Encryption granularity," above — a known, named gap, not a solved problem.
- **A formal ciphertext-fixity file format.** Deferred to each custodian's own recordkeeping, per `INTEGRITY.md`'s existing framework — SHA-256 doesn't care whether bytes are plaintext or ciphertext, so no new mechanism is needed for a custodian to track "has what I hold changed," only documentation that they may need to (see above).

## Open questions

- **Item-level cryptographic isolation isn't solved**, only named — see "Encryption granularity." Revisit if real capsules show this is more than a theoretical gap.
- **Whether the spec should eventually *recommend* (not require) passphrase-protected shares** as a default rather than leaving it silent — currently just left to the creator.
- **SLIP-0039's group-of-groups structure is unused.** Fine for now (this project only needs simple *k*-of-*n*), but worth remembering it's available without a format change if `ROADMAP.md` item 15 (custody/succession model) ever wants more elaborate trustee structures (e.g. "any 1 of 3 lawyers, OR 2 of 3 family members").

## Status

First pass, written during Phase 2 (Specification), addressing [`ROADMAP.md`](./ROADMAP.md) item 13. Still open to challenge per [`CONTRIBUTING.md`](./CONTRIBUTING.md) — this is the document on this project most in need of review by an actual cryptographer; the reasoning here is careful but not expert-audited.
