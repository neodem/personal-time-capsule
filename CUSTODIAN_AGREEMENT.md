# Custodian Agreement (draft)

Phase 3 (Legal) — [`ROADMAP.md`](./ROADMAP.md) item 18.

> ⚠️ **This document, and the template it describes, are drafts prepared by non-lawyers as part of an open-source project. Neither has been reviewed or approved by any licensed attorney. Do not sign, rely on, or treat any version of this material as legally effective for an actual custodial relationship. This is explicitly a starting point for professional legal review — not a substitute for it.** See [`CONTRIBUTING.md`](./CONTRIBUTING.md).

The actual fill-in-the-blank template lives at [`templates/legal/custodian-agreement.md`](./templates/legal/custodian-agreement.md); a filled-in worked example for the fictional Jane Smith capsule lives at [`examples/minimal-capsule/legal/custodian-agreement.md`](./examples/minimal-capsule/legal/custodian-agreement.md).

## What this agreement does, and why it exists

Three earlier documents each left a real number or threshold as a placeholder, deliberately, because naming it wasn't their job:

- `INTEGRITY.md` (item 11) **recommended** a fixity-check cadence ("at least annually") without making it enforceable.
- `CUSTODY.md` (item 15) left the custodian-unresponsiveness timeout as a number "the Phase 3 custodian agreement needs to actually commit to."
- `ACCESS_RESTRICTION_AGREEMENT.md` (item 17) left "what counts as sufficient documentation of a breach" open.

This document is where those recommendations become actual, binding obligations on a real custodian. It's the general **operational** relationship between creator and custodian — as distinct from the deed of gift (item 16, what's transferred), the access restriction agreement (item 17, when specific collections may be accessed), and succession instructions (item 19, the mechanics of a custodian handoff once triggered).

## The unresponsiveness timeout: escalating, not a single flat number

This was the one genuinely open design call in this document — put to the user rather than decided silently, since a timeout that's too short risks treating a merely slow, under-resourced institution as collapsed, while one that's too long risks leaving a capsule in limbo with a genuinely defunct custodian for years.

**Chosen: an escalating threshold.** One missed scheduled check-in triggers direct outreach from the successor chain or any reachable trustee — not an assumption of collapse, just a nudge. Only if a *second* consecutive check-in is also missed, with no response to that outreach, does the documented-unresponsiveness succession trigger (`CUSTODY.md`) actually fire. A custodian that responds at any point resets the clock. This gives a real institution genuine room (a year of silence alone doesn't end anything) while still bounding the worst case to roughly two years of undocumented silence before the chain activates — a meaningful improvement over waiting indefinitely, without manufacturing false urgency over an institution that's just slow.

## Fixity-check cadence, made binding

`INTEGRITY.md`'s recommendation becomes this agreement's actual term: **not less than once per calendar year**, plus after any custody transfer and before/after any physical media migration — the same events `INTEGRITY.md` already named. On a failed check, Custodian's obligations follow `INTEGRITY.md`'s recommended response directly: quarantine, notify, attempt restoration from another custodian's replica, and log the outcome either way, including "unrecoverable."

## Trustee attestation coordination

Nothing in `ACTORS.md` or `CUSTODY.md` assigns anyone the job of actually *prompting* trustees for their periodic attestation — it's described as something that happens, not something someone is responsible for making happen. This agreement gives that job to Custodian (who, practically, is the party most likely to still exist and be paying attention decades in), on the same annual cadence as fixity checks, for simplicity — one calendar rhythm to track rather than two.

## What counts as a documented breach

Consistent with `ACCESS.md`'s existing two-source standard for confirming a creator's death, this agreement applies the same pattern to documenting a custodian's breach of the Access Restriction Agreement: a named party asserting the breach occurred, plus a specific, corroborating description of what happened (a date, an action, a communication) — not a bare accusation. This isn't a new pattern invented for this document; it's the same "documented, not just asserted" standard `ACCESS.md` already established, applied to a different kind of claim.

## What's deliberately out of scope here

- **Funding and cost.** This document doesn't address who pays for decades of storage, staff time, or replication — a real and important gap for an institution expected to hold something for 50-100 years, but genuinely outside what a non-lawyer, non-archivist drafting a template can respons­ibly guess at. Flagged, not solved — see Open questions.
- **Jurisdiction-specific law and tax consequences** — same caveats as `DEED_OF_GIFT.md` and `ACCESS_RESTRICTION_AGREEMENT.md`.
- **The mechanics of a triggered succession itself** (verifying and transferring the replica) — `CUSTODY.md` already specifies that procedurally; `ROADMAP.md` item 19 (succession instructions) is where it gets a legal document of its own.

## Open questions

- **Funding sustainability isn't addressed at all.** A small, real institution (the kind this project explicitly wants to serve, per `PRINCIPLES.md` #1) may not have a 50-100-year funding plan for anything, regardless of what this agreement says. Whether this project should eventually recommend an accompanying endowment gift, a fee schedule, or something else is a real question this document doesn't attempt to answer.
- **The escalating timeout assumes someone is paying attention to notice a missed check-in and initiate outreach at all** — if the whole succession chain has gone quiet simultaneously (a broader failure than any one custodian), this mechanism doesn't help. Related to `CUSTODY.md`'s already-flagged "exhausted chain" limitation.
- **What happens to trustee-attestation coordination if Custodian itself is the party that's unresponsive** — presumably a successor custodian or the trustees themselves pick this up, but that hand-off isn't specified here.

## Status

First pass, written during Phase 3 (Legal), addressing [`ROADMAP.md`](./ROADMAP.md) item 18. Not usable — drafted by non-lawyers, pending review by a licensed attorney. See [`CONTRIBUTING.md`](./CONTRIBUTING.md).
