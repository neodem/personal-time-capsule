# Roadmap

The canonical, numbered list of what this project still needs to do, grouped into the five phases first scoped in the founding conversation ([`research/origin/2026-08-20-initial-conversation.md`](./research/origin/2026-08-20-initial-conversation.md)). That conversation is a frozen historical record and isn't updated as work progresses — **this document is**. Other docs (e.g. [`FORMAT.md`](./FORMAT.md)) cross-reference items here by number; item numbers are stable identifiers and shouldn't be reused or renumbered — append new items, or use sub-numbers (e.g. `8a`), if scope grows.

Status legend: ✅ done · 🚧 draft / in progress · ⬜ not started

## Phase 1 — Definition ✅

1. ✅ Name — `README.md`
2. ✅ Mission — `VISION.md`
3. ✅ Principles — `PRINCIPLES.md`
4. ✅ Threat model — `THREAT_MODEL.md`
5. ✅ Actors — `ACTORS.md`
6. ✅ Capsule lifecycle — `LIFECYCLE.md`

## Phase 2 — Specification 🚧

7. ✅ Directory structure — `FORMAT.md`, `examples/minimal-capsule/`
8. ✅ `capsule.yaml` — `FORMAT.md`, `schema/capsule.schema.json`, `examples/capsule.yaml`, `examples/minimal-capsule/capsule.yaml`
9. ✅ Human-readable documentation — `CREATOR_GUIDE.md` (a plain-language guide for a non-technical creator, distinct from the technical spec)
10. ✅ Metadata model — `METADATA.md`, `schema/events.schema.json`, `examples/minimal-capsule/data/provenance/events.yaml` (PREMIS-aligned object/event/provenance fields — see `STANDARDS.md`)
11. ✅ Integrity model — `INTEGRITY.md`, `schema/preservation-log.schema.json`, `examples/minimal-capsule/preservation-log.yaml` (beyond `capsule.yaml`'s `manifest_algorithm` field — fixity-check cadence, what a failed check triggers)
12. ⬜ Privacy model (per-item overrides when a collection-level zone isn't granular enough; sensitive-data-scanning categories)
13. ⬜ Encryption model (algorithm, key-splitting scheme — deliberately left unstubbed in `capsule.yaml`, see `FORMAT.md`)
14. ⬜ Access/release model (what actually satisfies each `release_policy.type` in `capsule.yaml`, e.g. what confirms an `after_death` trigger)
15. ⬜ Custody/succession model (what "successor activation" looks like procedurally)

## Phase 3 — Legal ⬜

16. ⬜ Deed of gift
17. ⬜ Access restriction agreement
18. ⬜ Custodian agreement
19. ⬜ Succession instructions
20. ⬜ Creator instructions

## Phase 4 — Software ⬜

Started early/experimentally (see `cli/`), out of order relative to Phases 2–3 — non-authoritative until the spec catches up.

21. 🚧 `ptc init` — `cli/` (Rust; non-interactive/flags-only first pass, see `cli/README.md`)
22. ⬜ `ptc import`
23. ⬜ `ptc inspect`
24. ⬜ `ptc scan`
25. ⬜ `ptc redact`
26. ⬜ `ptc encrypt`
27. ⬜ `ptc verify`
28. ⬜ `ptc unlock`
29. ⬜ `ptc deposit`

## Phase 5 — Real-world validation ⬜

Put the spec in front of archivists, preservation specialists, and estate lawyers, per `CONTRIBUTING.md`, and expect heavy revision as a result.

## Status

We are in **Phase 2**. Update this file's checkmarks as part of any PR that completes, drafts, or meaningfully advances an item.
