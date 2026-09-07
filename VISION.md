# Vision

## Mission

An open standard and open-source toolkit for preserving ordinary people's digital lives for future generations.

## Why this matters

Historians and archivists have always valued the personal record of ordinary people, not just the famous or powerful — a box of letters, a diary, a family photo album from a century ago is often more historically revealing than official records of the same period. The Society of American Archivists and institutions like the Library of Congress (through its Personal Digital Archiving program) explicitly recognize this.

But the shift to digital life has created a quiet crisis: the ordinary documentary record of a life — email, texts, phone photos, home video, casual writing — is disappearing almost as fast as it's created. It sits in formats and accounts that were never designed to outlive their owner, with no accessible path for a non-famous person to preserve it or donate it anywhere. The infrastructure and standards for long-term digital preservation already exist (BagIt, PREMIS, OAIS, LOCKSS); what's missing is the simple, human-facing layer that lets an ordinary person actually use them.

**The historically valuable part isn't necessarily the exceptional stuff. It's the mundane stuff, in context, at scale, over time.**

## What "done" looks like

A person with no special technical expertise can:

1. **Create** a capsule from the digital records they already have (email exports, photo libraries, documents, audio, video)
2. **Annotate** it with human context — who's in it, what mattered, what the world was like — without being forced to over-curate or "clean up" their life first
3. **Screen** it for sensitive material (SSNs, financial data, credentials, third-party privacy concerns) and choose what to redact, restrict, or exclude
4. **Protect** it with encryption and a configurable, documented release policy — including, if they want, a genuine multi-decade lock enforced by both cryptography and legal/social custody
5. **Deposit** it with an institution (or several, for redundancy) that can preserve, verify, and replicate the capsule *without* being able to read it before the release date
6. **Trust** that if their first-choice institution disappears, a defined succession chain keeps the archive from being orphaned

And a future archivist or historian, decades or centuries later, can open a capsule with nothing more exotic than a filesystem, ordinary standard software (a text editor, an image viewer, a media player — whatever the file actually calls for), and a published specification — no proprietary software required.

## What this project is not

- Not a company, not a product, not monetized in any way
- Not a replacement for BagIt, PREMIS, OAIS, or LOCKSS — a profile built on top of them
- Not a promise of unbreakable 50-year cryptographic secrecy — a documented, layered access policy combining cryptography with legal and institutional custody
- Not a mandate to curate your life into a tidy story — raw, messy, unedited material is explicitly welcome and often more valuable

## Roadmap (high level)

**Phase 1 — Definition**: name, mission, principles, threat model, actors, capsule lifecycle
**Phase 2 — Specification**: directory structure, `capsule.yaml`, metadata/integrity/privacy/encryption/access/custody models
**Phase 3 — Legal**: deed of gift, access restriction agreement, custodian agreement, succession instructions
**Phase 4 — Software**: reference CLI (`init`, `import`, `inspect`, `scan`, `redact`, `encrypt`, `verify`, `unlock`, `deposit`)
**Phase 5 — Real-world validation**: put it in front of archivists, preservation specialists, and estate lawyers, and expect to revise heavily

See [`ROADMAP.md`](./ROADMAP.md) for the full numbered item list and current status. Phase 2 is complete; we are moving into **Phase 3**.

## Working slogan

> You don't have to be famous to leave an archive.
