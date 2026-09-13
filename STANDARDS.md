# Standards & Tooling

This document is the formal reference for the external standards and technical building blocks the Personal Time Capsule project depends on. For each one: what it is, why we depend on it rather than inventing something new, what specifically we use it for in this project, and a plain-language explainer so a newcomer — an archivist, a lawyer, a future contributor — can get full context without leaving this repository.

Per [`PRINCIPLES.md`](./PRINCIPLES.md) #5, the rule is: build on existing, well-established preservation and security standards wherever possible, and scope new work to the genuine gap — a personal-scale, human-usable profile that ties them together.

## BagIt ([RFC 8493](https://www.rfc-editor.org/rfc/rfc8493))

**Why we depend on it:** BagIt is the payload-packaging format underneath a capsule. It lets us keep every file in its native format while still giving the whole package a verifiable, standard shape — directly serving Principle #3 (no proprietary lock-in).

**What it's used for here:** A capsule's on-disk structure is a Bag: payload files sit untouched in a `data/` directory, and BagIt-style manifests record a checksum for every file (see SHA-256, below). Sealing a capsule (`LIFECYCLE.md` stage 5) means generating this manifest; verifying a capsule (by a custodian, an auditor, or a future researcher) means recomputing checksums and comparing them against it.

**In plain terms:** BagIt is a simple, twenty-year-old, IETF-standardized convention for packaging a folder of files for storage or transfer so that anyone — using nothing more exotic than a checksum utility — can later confirm nothing in it has been altered or gone missing. It was created by the Library of Congress and is already the de facto packaging format across the digital preservation field, which is exactly why we reuse it instead of designing our own container.

## PREMIS ([Library of Congress](https://www.loc.gov/standards/premis/))

**Why we depend on it:** PREMIS gives us a vocabulary for the preservation metadata that has to travel alongside payload content — provenance, format, integrity events, and the actions taken on an object over time — instead of us having to invent metadata fields that a real archivist wouldn't recognize.

**What it's used for here:** The history of preservation-relevant events (import, screening, sealing, fixity checks, succession transfers) described in `LIFECYCLE.md` and `THREAT_MODEL.md` #2, #3, #5, and #9 is recorded in `data/provenance/events.yaml`, at event/batch granularity rather than a full per-file PREMIS Object record for every payload item — see [`METADATA.md`](./METADATA.md) (`ROADMAP.md` item 10) for the model and the reasoning behind that scope decision.

**In plain terms:** PREMIS (PREservation Metadata: Implementation Strategies) is a data dictionary — a defined, widely-adopted set of metadata fields — that digital archives use to record what a digital object is, where it came from, and what has happened to it, so that a future custodian who has never seen the object before can still understand its history and trustworthiness.

## OAIS / ISO 14721 ([ISO](https://www.iso.org/standard/81907.html))

**Why we depend on it:** OAIS is the reference model we position the whole project against, rather than a component we directly implement. It gives every role and process in `ACTORS.md` and `LIFECYCLE.md` a shared vocabulary that any archivist or preservation specialist reviewing this project will already recognize.

**What it's used for here:** Concepts like "Submission Information Package," "Archival Information Package," and "Dissemination Information Package" map loosely onto our deposit, dormant-preservation, and access stages. We don't require strict OAIS conformance, but we deliberately avoid naming or structuring things in ways that would conflict with it.

**In plain terms:** OAIS (Open Archival Information System) is an ISO standard reference model — not software, not a file format, but a conceptual framework — describing what any long-term digital archive needs to do: ingest content, preserve it, manage its metadata, and eventually give access to it. It's the shared mental model the digital preservation field has used for over two decades, which is why aligning with its terminology matters more than any specific technology choice.

## LOCKSS ([lockss.org](https://www.lockss.org/))

**Why we depend on it:** LOCKSS isn't software we run — it's the philosophy behind our approach to data loss (`THREAT_MODEL.md` #1) and institutional collapse (`THREAT_MODEL.md` #3): redundancy across independent custodians beats trusting any single copy or provider.

**What it's used for here:** It motivates the succession-chain design in `ACTORS.md` (successor custodian) and the deposit stage in `LIFECYCLE.md` (deposit "to a custodian institution, or several, for redundancy"). A capsule design that assumed one authoritative copy would violate this principle.

**In plain terms:** LOCKSS stands for "Lots of Copies Keep Stuff Safe" — a preservation strategy, originally built into a real piece of software developed at Stanford for libraries, based on the simple idea that the best defense against loss is many independently held, independently verifiable copies rather than one heavily-protected original.

## SHA-256 cryptographic hashing

**Why we depend on it:** Integrity verification (`PRINCIPLES.md` #3, `THREAT_MODEL.md` #5) requires a way to detect any change to a file, however small, without needing to trust whoever is presenting the capsule. SHA-256 is the hash algorithm BagIt manifests use by convention, and it's well-studied, collision-resistant at any practically relevant scale, and universally supported.

**What it's used for here:** Every payload file gets a SHA-256 checksum recorded in the capsule's BagIt manifest at sealing time (`LIFECYCLE.md` stage 5). Any custodian, auditor, or future researcher (`ACTORS.md`) can recompute these checksums independently to confirm the capsule hasn't been tampered with or corrupted — without needing any decryption capability at all (`PRINCIPLES.md` #8). How often that recomputation should happen, and what a mismatch should trigger, is [`INTEGRITY.md`](./INTEGRITY.md) (`ROADMAP.md` item 11).

**In plain terms:** SHA-256 is a cryptographic hash function — an algorithm that turns a file of any size into a fixed-length fingerprint such that changing even one byte of the original file produces a completely different fingerprint. Comparing fingerprints is how you prove a file is byte-for-byte identical to what it was at sealing time, without needing to compare the (possibly huge) files directly or trust anyone's word for it.

## XChaCha20-Poly1305 encryption

**Why we depend on it:** Once a capsule has a release policy other than immediate, its content needs to stay unreadable until that condition is met (`PRINCIPLES.md` #8) without depending on any single external service or proprietary tool surviving decades (`PRINCIPLES.md` #9). XChaCha20-Poly1305 is a well-studied, IETF-documented authenticated encryption cipher chosen specifically for its resistance to nonce-reuse mistakes — the most common way real-world implementations of similar ciphers (like AES-GCM) fail catastrophically — which matters for a spec that has to stay correctly implementable by people other than this project's own contributors, for decades.

**What it's used for here:** Every payload file that isn't in an `immediate`-release collection is encrypted in place with a 256-bit XChaCha20-Poly1305 key at the Encryption & policy binding lifecycle stage (`LIFECYCLE.md` stage 6). See [`ENCRYPTION.md`](./ENCRYPTION.md) (`ROADMAP.md` item 13) for the full reasoning, including why it was chosen over AES-256-GCM for this project specifically.

**In plain terms:** XChaCha20-Poly1305 is a modern encryption algorithm that both scrambles content so it can't be read without the right key, and detects tampering automatically — if even one byte of the encrypted data is altered, decryption fails outright rather than silently returning corrupted content.

## SLIP-0039 key splitting

**Why we depend on it:** A single person holding the only copy of a decryption key is both a coercion risk (`THREAT_MODEL.md` #6) and a single point of failure (`THREAT_MODEL.md` #7) — the key custody model needs to split a key across multiple trustees, any threshold of whom can reconstruct it, in a form recoverable without any of this project's own software.

**What it's used for here:** `capsule.yaml`'s `key_custody.splitting_scheme` splits each encryption key via SLIP-0039 (built on Shamir's Secret Sharing) into shares distributed to key-holding trustees (`ACTORS.md`), with a creator-chosen quorum required to reconstruct the key. See [`ENCRYPTION.md`](./ENCRYPTION.md) for why this was chosen over a simpler, unencoded secret-sharing scheme.

**In plain terms:** SLIP-0039 splits a secret into several pieces — encoded as sequences of ordinary words rather than random-looking characters, with a built-in checksum — such that a defined minimum number of pieces (but not fewer) can reconstruct the original secret. It was originally designed so someone could safely back up a cryptocurrency wallet across several trusted people or locations; this project reuses the same mechanism for the same underlying problem (a secret that must survive a long time without any single point of failure), unrelated to cryptocurrency itself.

## Status

First pass, written during Phase 1 (Definition); updated during Phase 2 once the encryption model (`ROADMAP.md` item 13) settled on XChaCha20-Poly1305 and SLIP-0039. This document does not yet cover specific sensitive-data-scanning tooling in detail; `ptc scan` (`ROADMAP.md` item 24) now exists and uses plain regex matching (SSN format, Luhn-checked credit card numbers, AWS key/private-key/credential patterns) rather than any named third-party technique or library, so there's no external standard to cite here yet. `PRIVACY.md` describes the recommended category vocabulary `ptc scan`'s findings map onto. This document should grow if that changes -- e.g. if a real NLP-based approach is ever adopted for the categories regex can't reach (`medical`, `third-party-privacy`).
