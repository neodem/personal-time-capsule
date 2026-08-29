# Standards & Tooling

This document is the formal reference for the external standards and technical building blocks the Personal Time Capsule project depends on. For each one: what it is, why we depend on it rather than inventing something new, what specifically we use it for in this project, and a plain-language explainer so a newcomer — an archivist, a lawyer, a future contributor — can get full context without leaving this repository.

Per [`PRINCIPLES.md`](./PRINCIPLES.md) #5, the rule is: build on existing, well-established preservation and security standards wherever possible, and scope new work to the genuine gap — a personal-scale, human-usable profile that ties them together.

## BagIt ([RFC 8493](https://www.rfc-editor.org/rfc/rfc8493))

**Why we depend on it:** BagIt is the payload-packaging format underneath a capsule. It lets us keep every file in its native format while still giving the whole package a verifiable, standard shape — directly serving Principle #3 (no proprietary lock-in).

**What it's used for here:** A capsule's on-disk structure is a Bag: payload files sit untouched in a `data/` directory, and BagIt-style manifests record a checksum for every file (see SHA-256, below). Sealing a capsule (`LIFECYCLE.md` stage 5) means generating this manifest; verifying a capsule (by a custodian, an auditor, or a future researcher) means recomputing checksums and comparing them against it.

**In plain terms:** BagIt is a simple, twenty-year-old, IETF-standardized convention for packaging a folder of files for storage or transfer so that anyone — using nothing more exotic than a checksum utility — can later confirm nothing in it has been altered or gone missing. It was created by the Library of Congress and is already the de facto packaging format across the digital preservation field, which is exactly why we reuse it instead of designing our own container.

## PREMIS ([Library of Congress](https://www.loc.gov/standards/premis/))

**Why we depend on it:** PREMIS gives us a vocabulary for the preservation metadata that has to travel alongside payload content — provenance, format, integrity events, and the actions taken on an object over time — instead of us having to invent metadata fields that a real archivist wouldn't recognize.

**What it's used for here:** Format information (what a file is, what produced it) and the history of preservation-relevant events (import, screening, sealing, fixity checks, succession transfers) described in `LIFECYCLE.md` and `THREAT_MODEL.md` #2 and #9 are expected to be expressed in PREMIS-compatible terms once Phase 2 defines the capsule's metadata schema.

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

**What it's used for here:** Every payload file gets a SHA-256 checksum recorded in the capsule's BagIt manifest at sealing time (`LIFECYCLE.md` stage 5). Any custodian, auditor, or future researcher (`ACTORS.md`) can recompute these checksums independently to confirm the capsule hasn't been tampered with or corrupted — without needing any decryption capability at all (`PRINCIPLES.md` #8).

**In plain terms:** SHA-256 is a cryptographic hash function — an algorithm that turns a file of any size into a fixed-length fingerprint such that changing even one byte of the original file produces a completely different fingerprint. Comparing fingerprints is how you prove a file is byte-for-byte identical to what it was at sealing time, without needing to compare the (possibly huge) files directly or trust anyone's word for it.

## Status

First pass, written during Phase 1 (Definition). This document currently covers the standards the project has already committed to (per the README and `PRINCIPLES.md`); it does not yet cover encryption schemes, key-splitting/secret-sharing approaches, or specific sensitive-data-scanning tooling, since those choices haven't been made — `THREAT_MODEL.md` and `LIFECYCLE.md` describe the *requirements* (e.g. quorum-based key custody) those future choices need to satisfy, but naming specific tools for them is Phase 2/4 work. This document should grow as those choices are made.
