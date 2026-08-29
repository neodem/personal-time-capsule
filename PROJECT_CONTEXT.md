# Project Context (for Claude Code sessions)

This file exists so any Claude Code session starting fresh in this repo has full context without the user needing to re-explain. Read this first.

## What this project is

**Personal Time Capsule** — an open-source, non-commercial, public-good standard and toolkit that lets ordinary people preserve their digital lives (email, photos, writing, audio, video) as historically useful, privacy-respecting, legally governed archives for future generations. Not a company, not monetized.

Full framing lives in `README.md`, `VISION.md`, and `PRINCIPLES.md` at repo root — read those before doing design work.

## Where things stand

- **Phase**: Phase 2 (Specification), per [`ROADMAP.md`](./ROADMAP.md) — the canonical, numbered, status-tracked list of everything left to do across all five phases. Read that file, not this section, for current progress; this section is deliberately not a duplicate of it.
- Phase 1 (Definition) is fully done: `README.md`, `VISION.md`, `PRINCIPLES.md`, `THREAT_MODEL.md`, `ACTORS.md`, `LIFECYCLE.md`, `STANDARDS.md` are all on `main`.
- Phase 2 is underway: `FORMAT.md` and `examples/capsule.yaml` draft the directory structure and `capsule.yaml` (ROADMAP items 7–8). Items 9–15 (human-readable docs, metadata/integrity/privacy/encryption/access/custody models) are not started.
- Licensing is decided: Apache-2.0 for code/tooling (`LICENSE`), CC BY 4.0 for the specification/docs (`LICENSE-SPEC.md`) — no longer TBD.
- `CONTRIBUTING.md` and a "challenge an assumption" GitHub issue template exist, giving Principle #10 (build in public, invite disagreement) an actual mechanism.
- `research/origin/2026-08-20-initial-conversation.md` is the founding design conversation, kept as a frozen provenance record — don't treat it as a living roadmap; that's what `ROADMAP.md` is for.
- No reference CLI or other code exists yet (Phase 4).

## Key design decisions already locked in

- Build on **BagIt** (packaging), **PREMIS** (preservation metadata), **OAIS/ISO 14721** (reference model), **LOCKSS** philosophy, and **SHA-256** (integrity) — do not reinvent these. See `STANDARDS.md` for why each one and what it's used for.
- A capsule **is a BagIt bag**. `capsule.yaml` is a root-level BagIt *tag file* (checksummed via `tagmanifest-sha256.txt`), not payload — see `FORMAT.md`.
- The **human-context layer is payload**, not metadata-about-metadata: `data/person/` (biography, timeline, people, places) lives under `data/` and is checksummed in `manifest-sha256.txt` alongside photos/email/etc., per Principle #6.
- Privacy zone and release policy are declared **capsule-wide by default, with optional per-named-collection overrides** in `capsule.yaml`. Per-item granularity, if ever needed, is deferred to a separate manifest — `capsule.yaml` stays a bounded, readable file.
- **Encryption is deliberately unspecified so far.** `capsule.yaml`'s `encryption` section only declares the shape of key custody (a quorum scheme); no algorithm or key-splitting mechanism (e.g. Shamir's Secret Sharing) is committed to yet — that's ROADMAP item 13, a dedicated future decision.
- **Preservation and access are separate** — an institution can hold/verify/replicate a capsule while being cryptographically unable to read it before a release condition is met.
- Release policy is **configurable per collection** (e.g., photos vs. email vs. private journal can each have different lock periods), not one global unlock date.
- No promise of "cryptographically impossible to open" — layered protection (crypto + legal + custodial), documented assumptions, no dependency on any single external service surviving decades.
- **Preserve the mess** — raw/unedited material is welcome; curation is additive, never required.
- Privacy tooling (sensitive-data scanning, redaction, privacy zones: public/restricted/never-release) is core, not an add-on.

## Working style / workflow preferences

- Changes go through a **feature branch + PR**, not direct pushes to `main`. This has been the actual pattern used (PRs #1–4 so far) — keep doing it.
- The user is non-technical-by-trade but hands-on; explain tradeoffs plainly, don't assume deep git/crypto background.
- The project is explicitly meant to be developed **in public** — docs should read as if archivists/historians/lawyers may be watching and are invited to disagree.
- Keep tone of docs matter-of-fact and a little idealistic, not corporate/marketing.
- When a design decision has real tradeoffs and isn't purely technical, ask the user rather than picking silently — this has worked well for licensing and the `capsule.yaml` structural decisions.
- Avoid duplicating living status/tracking information across multiple docs (e.g. phase progress) — point to the one canonical doc (`ROADMAP.md`) instead, to prevent drift. This file was itself a repeat offender until this rewrite.

## Suggested first task for a new session

Continue Phase 2 per `ROADMAP.md` items 9–15. The most natural next pieces, since `FORMAT.md`'s "Open questions" section already flags them as gaps in the current `capsule.yaml` draft:

- **Privacy model** (item 12) — how per-item overrides work when a collection-level zone isn't granular enough
- **Access/release model** (item 14) — what actually satisfies each `release_policy.type` (e.g. what confirms an `after_death` trigger)
- **Custody/succession model** (item 15) — what "successor activation" looks like procedurally

Open a PR against `main` for review before merging, and update `ROADMAP.md`'s status markers as part of that PR.
