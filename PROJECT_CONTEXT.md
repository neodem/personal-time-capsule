# Project Context (for Claude Code sessions)

This file exists so any Claude Code session starting fresh in this repo has full context without the user needing to re-explain. Read this first.

## What this project is

**Personal Time Capsule** — an open-source, non-commercial, public-good standard and toolkit that lets ordinary people preserve their digital lives (email, photos, writing, audio, video) as historically useful, privacy-respecting, legally governed archives for future generations. Not a company, not monetized.

Full framing lives in `README.md`, `VISION.md`, and `PRINCIPLES.md` at repo root — read those before doing design work.

## Where things stand

- **Phase**: Phase 1 (Definition) — name, mission, and principles are done. Still need: threat model, actors, capsule lifecycle.
- `README.md`, `VISION.md`, `PRINCIPLES.md`, and `research/origin/2026-08-20-initial-conversation.md` (the founding design conversation, kept as provenance) are already pushed to `main`.
- No specification, schema, or code has been written yet. Nothing in `specification/`, `reference-implementation/`, `capsule-examples/`, or `legal/` exists yet.

## Roadmap (from VISION.md)

1. **Definition** (current) — name, mission, principles, threat model, actors, capsule lifecycle
2. **Specification** — directory structure, `capsule.yaml`, metadata/integrity/privacy/encryption/access/custody models
3. **Legal** — deed of gift, access restriction agreement, custodian agreement, succession instructions
4. **Software** — reference CLI (`init`, `import`, `inspect`, `scan`, `redact`, `encrypt`, `verify`, `unlock`, `deposit`)
5. **Real-world validation** — get in front of archivists, preservation specialists, estate lawyers; expect heavy revision

## Key design decisions already locked in

- Build on **BagIt** (packaging), **PREMIS** (preservation metadata), **OAIS/ISO 14721** (reference model), and **LOCKSS** philosophy — do not reinvent these.
- Capsule payload stays as **plain files**, readable without any of our tooling.
- **Preservation and access are separate** — an institution can hold/verify/replicate a capsule while being cryptographically unable to read it before a release condition is met.
- Release policy is **configurable per collection** (e.g., photos vs. email vs. private journal can each have different lock periods), not one global unlock date.
- No promise of "cryptographically impossible to open" — layered protection (crypto + legal + custodial), documented assumptions, no dependency on any single external service surviving decades.
- **Preserve the mess** — raw/unedited material is welcome; curation is additive, never required.
- Privacy tooling (sensitive-data scanning, redaction, privacy zones: public/restricted/never-release) is core, not an add-on.
- License likely Apache-2.0 or MIT for code, CC BY or similar for the spec — not finalized.

## Working style / workflow preferences

- Changes go through a **feature branch + PR**, not direct pushes to `main`.
- The user is non-technical-by-trade but hands-on; explain tradeoffs plainly, don't assume deep git/crypto background.
- The project is explicitly meant to be developed **in public** — docs should read as if archivists/historians/lawyers may be watching and are invited to disagree.
- Keep tone of docs matter-of-fact and a little idealistic, not corporate/marketing.

## Suggested first task for a new session

Start a branch (e.g. `docs/phase-1-foundation`) and draft the remaining Phase 1 pieces:
- `THREAT_MODEL.md` — what/who the design protects against (data loss, format obsolescence, institutional collapse, unauthorized early access, coercion, etc.)
- `ACTORS.md` — the roles in the system (creator, custodian institution, successor institution, trustee/key-holder, future researcher, etc.) and what each can/can't do
- `LIFECYCLE.md` — the stages a capsule goes through from creation to eventual public access

Open a PR against `main` for review before merging.
