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
8a. 🚧 Capsule series — `FORMAT.md`, `schema/capsule.schema.json`, `LIFECYCLE.md`, `CREATOR_GUIDE.md` (optional `series.continues` field lets a new capsule reference an earlier, already-sealed one's `id`, for a creator whose archive spans multiple sequential capsules over a lifetime rather than one perpetually-unsealed archive; deliberately one-directional — a capsule can't reference a *later* one, since an earlier capsule's `capsule.yaml` is frozen forever once sealed — and deliberately no merge mechanism; see `FORMAT.md`)
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
24. 🚧 `ptc scan` — `cli/` (LIFECYCLE.md stage 4; report-only regex scan of `data/` for `PRIVACY.md`'s `financial`/`credentials`/`government-id` categories — SSNs, Luhn-checked credit card numbers, AWS keys, private key headers, credential assignments — with masked output and no writes; `medical`/`third-party-privacy` need semantic understanding no regex can provide, so aren't attempted — see `cli/README.md`)
25. 🚧 `ptc redact` — `cli/` (LIFECYCLE.md stage 4; targets one creator-named file, dry-run by default, `--apply` to actually rewrite; auto-redacts SSNs/credit card numbers/AWS keys/credential values in place, flags a private key header for manual review rather than falsely claiming it's handled; answers `PRIVACY.md`'s open question on redaction mechanics by overwriting in place with no automatic backup — see `cli/README.md`)
25a. 🚧 `ptc seal` — `cli/` (LIFECYCLE.md stage 5; no command existed for this stage until now, even though `verify`/`encrypt`/`deposit` all assume a manifest already exists — see `cli/README.md`)
26. 🚧 `ptc encrypt` — `cli/` (LIFECYCLE.md stage 6; XChaCha20-Poly1305 payload encryption only, one key per unit (named collection, or `data/person`/`data/provenance` under the capsule default), dry-run by default; built around a `Cipher` trait so a future algorithm change is a new impl, not a rewrite; never writes to `capsule.yaml` — logs to `preservation-log.yaml` instead, since `capsule.yaml` is frozen forever once sealed; SLIP-0039 key-splitting is NOT implemented — no solid Rust crate exists, so `--apply` issues each unit's raw key directly to the creator — see `cli/README.md` and `ENCRYPTION.md`)
27. 🚧 `ptc verify` — `cli/` (read-only: recomputes checksums against `manifest-sha256.txt`/`tagmanifest-sha256.txt`; treats an encrypted (`.enc`) file as informational, not a failure, per `ENCRYPTION.md`; flags any file under `data/` the manifest never recorded; does not yet do full JSON Schema conditional validation — see `cli/README.md`)
28. 🚧 `ptc unlock` — `cli/` (LIFECYCLE.md stage 10; the inverse of `ptc encrypt` -- takes a unit's raw key directly (no SLIP-0039 quorum reconstruction, since none was ever split), dry-run by default validating the key against every `.enc` file before `--apply` writes anything; logs to `preservation-log.yaml`, same reasoning as `ptc encrypt` — see `cli/README.md`)
29. 🚧 `ptc deposit` — `cli/` (LIFECYCLE.md stage 7; always verifies first and refuses to proceed if that fails; optional `--archive` serializes into one `.tar.gz` purely as a transport convenience (BagIt's own "serialization" concept, RFC 8493) -- the capsule's real form stays a directory; dry-run by default; `--apply` logs a `deposit` event to `preservation-log.yaml` naming the custodian; does not invent a transfer protocol -- see `cli/README.md`)

## Phase 5 — Real-world validation ⬜

Put the spec in front of archivists, preservation specialists, and estate lawyers, per `CONTRIBUTING.md`, and expect heavy revision as a result.

Early outreach already surfaced one idea worth tracking rather than acting on yet: `ptc`'s own UX is deliberately bare-bones (Phase 4's CLI is a reference implementation, not a consumer tool), and the realistic path to something a non-technical person would actually use may not be making `ptc` itself friendlier so much as *other* tools building on top of it entirely — a Gmail plugin that creates a capsule from an export, an Apple Photos plugin, a phone app that does the packaging for someone who'd never touch a command line. This is exactly what `cli/README.md`'s "Stability" section already set up for: the CLI's flags/output/exit-codes are the versioned public contract a future tool in any language builds on, not an internal implementation detail. Noted here as a real, externally-suggested direction, not a commitment to build any of it.

## Status

Phase 2 (Specification) is complete except item 8a (capsule series), added after the fact and still a first draft. Phase 3 (Legal) now has a first draft of every item (16-20) — but unlike Phase 2, Phase 3 isn't "done" just because a draft exists: items 16-19 explicitly require review by a licensed attorney before they're usable by anyone (see each document's own warning banner), which hasn't happened yet. Phase 4 (Software) now has a first pass of every reference CLI command (items 21-29, plus 25a) — none of it should be treated as production-grade custody tooling yet (see each command's own "Known limitations" notes in `cli/README.md`), and real cryptographic key-splitting (SLIP-0039) remains an open, documented gap in `ptc encrypt`/`ptc unlock` pending a usable Rust crate. Phase 5 (Real-world validation) has started — the repo is public and early outreach is underway; see this phase's own note above for the first concrete idea that's come back from it. Update this file's checkmarks as part of any PR that completes, drafts, or meaningfully advances an item.
