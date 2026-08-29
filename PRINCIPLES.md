# Principles

These are the design commitments the Personal Time Capsule project holds itself to. Specification and tooling decisions should be traceable back to one or more of these.

## 1. Ordinary lives have historical value

The project makes no assumption that a creator is famous, wealthy, or otherwise historically significant. That's the entire point. A detailed, contextualized record of an unremarkable life may be more valuable to future historians than a curated record of a remarkable one.

## 2. The archive belongs to the creator

The creator decides what goes in, what doesn't, who receives it, when it becomes accessible, what stays private, and what happens if their intended repository ceases to exist. Nothing in the standard should take these decisions out of the creator's hands.

## 3. No proprietary lock-in

A capsule must remain intelligible without our software. If this project and every tool it ever produced vanished tomorrow, a capsule created today should still be readable with a filesystem, ordinary non-proprietary software (a text editor for text, standard viewers/players for images, audio, and video), and the published specification. This is why the project builds on BagIt rather than inventing a new binary container — payload content stays in its native, standard format, not something trapped inside a proprietary serialization. "Plain" here means *no proprietary wrapper*, not that every file is human-readable text — a JPEG or an MP4 is still a plain file in this sense, just a binary one.

## 4. Open source and open standard, forever

Code and specification are freely licensed with no plans to change that. License specifics (likely Apache-2.0 or MIT for tooling, CC BY or similar for the specification) are tracked in `LICENSE` files, not this document.

## 5. Build on existing standards wherever possible

We are not trying to replace or compete with the existing digital preservation field. We reuse:

- **BagIt** for packaging
- **PREMIS** for preservation metadata
- **OAIS** (ISO 14721) as the reference model for how the pieces relate
- **LOCKSS** philosophy ("lots of copies keep stuff safe") for distributed, verifiable redundancy

New specification work should be scoped to the genuine gap: a personal-scale, human-usable profile that ties these together, not a reinvention of any of them.

## 6. Preserve the mess

Creators are not required to curate their lives into a tidy narrative before preserving them. Raw, unedited material — a chaotic inbox, a poorly-named folder, thousands of unsorted photos — is explicitly welcome. The relationships between records, including the disorganized ones, can themselves carry historical value. Curation and human-context annotation are encouraged as an *addition*, never a *replacement*, for the raw record.

## 7. Privacy is fundamental, not an afterthought

Personal archives inevitably contain private correspondence, information about living third parties, financial data, credentials accidentally left in files, medical information, and other sensitive material. The standard and tooling must treat identifying and handling this material as a core feature — sensitive-data scanning, redaction workflows, and support for multiple privacy zones (e.g., public-after-release, restricted, never-release) — not a footnote.

## 8. Preservation and access are different things

An institution should be able to receive, verify, replicate, and preserve a capsule today while being cryptographically unable to read its contents until a documented release condition is met. This lets preservation start immediately without forcing a premature decision about access, and lets the technical and legal/custodial protections work together rather than relying on either alone.

## 9. No brittle promises

The project does not promise that content is "cryptographically impossible to open" for a fixed period — that depends on threat models and key custody that aren't fully in the creator's control. Instead, capsules document a specific, layered access policy (cryptographic + legal + custodial) and are explicit about its assumptions and limitations. The standard must not depend on any single external service (e.g., a particular time-lock provider) existing decades from now.

## 10. Build in public, invite disagreement

Design work happens in the open, in this repository, from day one. We actively want archivists, preservation specialists, cryptographers, and estate lawyers to challenge our assumptions before we invest heavily in implementation. Being told we're wrong early is a feature of this process, not a failure of it.
