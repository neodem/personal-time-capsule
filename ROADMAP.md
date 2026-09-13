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

## Phase 2 — Specification ✅

7. ✅ Directory structure — `FORMAT.md`, `examples/minimal-capsule/`
8. ✅ `capsule.yaml` — `FORMAT.md`, `schema/capsule.schema.json`, `examples/capsule.yaml`, `examples/minimal-capsule/capsule.yaml`
9. ✅ Human-readable documentation — `CREATOR_GUIDE.md` (a plain-language guide for a non-technical creator, distinct from the technical spec)
10. ✅ Metadata model — `METADATA.md`, `schema/events.schema.json`, `examples/minimal-capsule/data/provenance/events.yaml` (PREMIS-aligned object/event/provenance fields — see `STANDARDS.md`)
11. ✅ Integrity model — `INTEGRITY.md`, `schema/preservation-log.schema.json`, `examples/minimal-capsule/preservation-log.yaml` (beyond `capsule.yaml`'s `manifest_algorithm` field — fixity-check cadence, what a failed check triggers)
12. ✅ Privacy model — `PRIVACY.md`, `schema/privacy-overrides.schema.json`, `examples/minimal-capsule/privacy-overrides.yaml` (per-item overrides when a collection-level zone isn't granular enough; sensitive-data-scanning categories)
13. ✅ Encryption model — `ENCRYPTION.md` (XChaCha20-Poly1305, SLIP-0039 key splitting, collection-level granularity; algorithm/scheme were deliberately left unstubbed in `capsule.yaml`, see `FORMAT.md`)
14. ✅ Access/release model — `ACCESS.md` (what actually satisfies each `release_policy.type` in `capsule.yaml`, e.g. what confirms an `after_death` trigger)
15. ✅ Custody/succession model — `CUSTODY.md` (what "successor activation" looks like procedurally)

## Phase 3 — Legal 🚧

Unlike Phase 2's model documents, "🚧" here specifically means *drafted by non-lawyers, not yet reviewed by a licensed attorney, not usable as-is* — a stronger caveat than Phase 2's "open to challenge." See each document's own warning banner. (Item 20, creator instructions, is the one exception — see its own entry below.)

16. 🚧 Deed of gift — `DEED_OF_GIFT.md`, `templates/legal/deed-of-gift.md`, `examples/minimal-capsule/legal/deed-of-gift.md` (drafted by non-lawyers; not usable until reviewed by a licensed attorney)
17. 🚧 Access restriction agreement — `ACCESS_RESTRICTION_AGREEMENT.md`, `templates/legal/access-restriction-agreement.md`, `examples/minimal-capsule/legal/access-restriction-agreement.md` (merges the founding conversation's "Access Restriction" and "Posthumous Release" sketches, since `ACCESS.md` already unified those triggers; breach is a new `CUSTODY.md` succession trigger)
18. 🚧 Custodian agreement — `CUSTODIAN_AGREEMENT.md`, `templates/legal/custodian-agreement.md`, `examples/minimal-capsule/legal/custodian-agreement.md` (commits the fixity-check cadence and unresponsiveness timeout `INTEGRITY.md`/`CUSTODY.md` left as recommendations, and what counts as a documented breach)
19. 🚧 Succession instructions — `SUCCESSION_INSTRUCTIONS.md`, `templates/legal/succession-instructions.md` + `certificate-of-succession.md` + `trustee-acceptance.md`, `examples/minimal-capsule/legal/succession-instructions.md` (gets every named successor custodian to pre-commit at deposit, rather than merely being named)
20. ✅ Creator instructions — `templates/legal/creator-instructions.md`, `examples/minimal-capsule/legal/creator-instructions.md` (a personal worksheet, not a binding agreement — no attorney-review caveat needed, since no institution's obligations depend on it; recommended to keep outside the sealed bag, with the creator's will, since unlike the other four documents it's meant to be updated over time)

## Phase 4 — Software 🚧

Started early/experimentally (see `cli/`) — the spec (Phases 2–3) has since fully caught up, so this is no longer "out of order," but the CLI itself is still an early reference implementation. Building one command at a time, same branch+PR+review pattern as Phases 2–3. See `cli/README.md`'s "Stability" section: the CLI's flags/output/exit-codes are this project's public integration contract, not its internal Rust types.

21. 🚧 `ptc init` — `cli/` (Rust; non-interactive/flags-only, now backed by a real `serde` capsule model, see `cli/README.md`)
22. 🚧 `ptc import` — `cli/` (LIFECYCLE.md stage 2; copies a file/directory into `data/<collection>/`, auto-registers the collection in `capsule.yaml` if new, and logs an `import` event in `data/provenance/events.yaml`; refuses once a capsule is past the pre-seal amend loop)
23. 🚧 `ptc inspect` — `cli/` (read-only summary: capsule identity/stage, `data/person/` presence, provenance event count, each collection's actual size and resolved privacy/release policy, undeclared `data/` directories, optional tag files, and manifest presence — see `cli/README.md`)
24. ⬜ `ptc scan`
25. ⬜ `ptc redact`
25a. 🚧 `ptc seal` — `cli/` (LIFECYCLE.md stage 5; no command existed for this stage until now, even though `verify`/`encrypt`/`deposit` all assume a manifest already exists — see `cli/README.md`)
26. ⬜ `ptc encrypt`
27. ⬜ `ptc verify`
28. ⬜ `ptc unlock`
29. ⬜ `ptc deposit`

## Phase 5 — Real-world validation ⬜

Put the spec in front of archivists, preservation specialists, and estate lawyers, per `CONTRIBUTING.md`, and expect heavy revision as a result.

## Status

Phase 2 (Specification) is complete. Phase 3 (Legal) now has a first draft of every item (16-20) — but unlike Phase 2, Phase 3 isn't "done" just because a draft exists: items 16-19 explicitly require review by a licensed attorney before they're usable by anyone (see each document's own warning banner), which hasn't happened yet. Update this file's checkmarks as part of any PR that completes, drafts, or meaningfully advances an item.
