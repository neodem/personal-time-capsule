# Personal Time Capsule

> **You don't have to be famous to leave an archive.**

An open standard and open-source toolkit for preserving ordinary people's digital lives — email, photos, writing, audio, video, and the general mess of a life lived online — as historically useful, privacy-respecting, legally governed archives for future generations.

This is a **non-commercial, public-good project**. There is no product to buy and no company behind it. The goal is a standard good enough that archivists, historians, and ordinary people would actually want to use it.

## The problem

Most of what makes up a life today — email threads, phone photos, text messages, documents, home video — lives in accounts and formats that don't survive their owner by more than a few years. Cloud accounts get closed. Hard drives rot. Formats go obsolete. Unlike the letters and photo albums historians prize from a century ago, almost none of this ordinary digital record is built to last, and there's no accessible path for a non-famous person to donate it anywhere.

## The idea

A **capsule**: a standardized, self-describing package of a person's digital records, built on existing preservation standards rather than a new proprietary format, with:

- **Standard formats as payload** — content stays as ordinary files in their native format (text, JPEG, MP4, whatever they actually are), not repackaged into a proprietary container, and openable without our software even decades from now
- **A human context layer** — biography, timeline, people, places: the annotations that turn a folder of files into a historical record
- **Integrity guarantees** — cryptographic manifests (SHA-256) so corruption or tampering can be detected
- **Privacy as a first-class concern** — sensitive-data scanning, redaction tooling, and multiple privacy zones (public, restricted, never-release)
- **Separation of preservation and access** — an archive can be safely held and verified by an institution today while remaining cryptographically locked until a release date or condition
- **Configurable release policies** — immediate, a fixed date, "after death," "after death + N years," different policies per collection
- **Legal and custodial succession** — deed-of-gift and access-restriction templates, plus a chain of fallback custodians so the archive isn't orphaned if one institution disappears

## What we're building on

We are **not** trying to replace the existing digital preservation field. We're building a thin, human-facing layer on top of it:

- [BagIt](https://www.rfc-editor.org/rfc/rfc8493) — payload packaging and manifests
- [PREMIS](https://www.loc.gov/standards/premis/) — preservation metadata
- [OAIS](https://www.iso.org/standard/81907.html) (ISO 14721) — the broader reference model
- [LOCKSS](https://www.lockss.org/) — distributed, "lots of copies keep stuff safe" preservation philosophy

See [`STANDARDS.md`](./STANDARDS.md) for what each of these is, why we depend on it, and what it's specifically used for here. See [`PRINCIPLES.md`](./PRINCIPLES.md) for the design commitments and [`VISION.md`](./VISION.md) for the fuller mission and roadmap.

## Status

Early design phase. We're currently working through Phase 1 (name, mission, principles, threat model, actors, capsule lifecycle) before touching schemas or code. See [`research/`](./research) for open questions and background reading, and the [issues](../../issues) for active discussion.

We want archivists, historians, preservation specialists, cryptographers, and estate lawyers to poke holes in this. If something here is naive, wrong, or has already been solved better elsewhere, please say so — see [`CONTRIBUTING.md`](./CONTRIBUTING.md) for how.

## License

TBD — likely Apache-2.0 or MIT for code/tooling, and something like CC BY for the specification itself. Open to discussion.
