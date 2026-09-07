# Access Restriction Agreement (draft)

Phase 3 (Legal) — [`ROADMAP.md`](./ROADMAP.md) item 17.

> ⚠️ **This document, and the template it describes, are drafts prepared by non-lawyers as part of an open-source project. Neither has been reviewed or approved by any licensed attorney. Do not sign, rely on, or treat any version of this material as legally effective for an actual access restriction. This is explicitly a starting point for professional legal review — not a substitute for it.** See [`CONTRIBUTING.md`](./CONTRIBUTING.md).

The actual fill-in-the-blank template lives at [`templates/legal/access-restriction-agreement.md`](./templates/legal/access-restriction-agreement.md); a filled-in worked example for the fictional Jane Smith capsule lives at [`examples/minimal-capsule/legal/access-restriction-agreement.md`](./examples/minimal-capsule/legal/access-restriction-agreement.md).

## What this agreement does, and doesn't, cover

The founding conversation ([`research/origin/2026-08-20-initial-conversation.md`](./research/origin/2026-08-20-initial-conversation.md)) originally sketched "Access Restriction" and "Posthumous Release" as separate documents. They're merged into one here, because [`ACCESS.md`](./ACCESS.md) (`ROADMAP.md` item 14) already unified `immediate`, `fixed_date`, `after_death`, `after_death_plus_years`, `never`, and `custom` under one `release_policy` vocabulary — a legal document that split posthumous triggers out from the rest would be re-fragmenting something the technical spec deliberately consolidated.

This document legally formalizes the release terms already recorded in a capsule's own `capsule.yaml` — it's the binding obligation *not* to grant access before the recorded trigger, and to actually grant it once the trigger fires. It is deliberately **not**:

- **The deed of gift** (`ROADMAP.md` item 16, already drafted) — that transfers custody and a license; this agreement is a condition attached to how that license may be exercised.
- **The custodian agreement** (`ROADMAP.md` item 18, not yet drafted) — that covers the custodian's *ongoing operational* obligations (fixity-check cadence, replication) — this document is specifically about access *timing*, not general custodial duty.
- **Creator instructions** (`ROADMAP.md` item 20, not yet drafted) — personal guidance from the creator to their family/executor, distinct from a binding agreement with the custodian.

## Per-collection granularity, not one capsule-wide trigger

`ENCRYPTION.md` and `ACCESS.md` already established that release policy — and the encryption key enforcing it — varies per collection, not per capsule. This agreement's template reflects that directly with a release schedule listing each collection by name, its privacy zone, and its release policy, rather than assuming one trigger applies to everything. A capsule with an immediately-public photo collection and a 50-years-after-death email collection needs both terms recorded here, not just the capsule-wide default.

## Third-party privacy is an independent restriction, not just a courtesy

Per `PRINCIPLES.md` #6, `ACTORS.md`'s named subject role, and `PRIVACY.md` (`ROADMAP.md` item 12), a capsule's privacy zones and any per-item overrides in `privacy-overrides.yaml` are restrictions in their own right, independent of timing. An item marked `never-release`, or overridden to a stricter zone than its collection's default, stays restricted even after the collection's general release trigger fires. The template makes this explicit rather than letting a reader assume "the trigger fired" automatically means "everything in the collection is now available."

## Breach and remedy: reusing the succession chain, not inventing a new enforcer

This was a genuine, substantive judgment call — put to the user rather than decided silently, since the alternatives (naming a new, potentially decades-long-lived enforcement role; or specifying no mechanism at all and hoping general contract law is enough) are both real options with real drawbacks.

**The chosen approach: a documented breach — early release, or refusal to release once a trigger has fired — is treated as a third custodian-succession trigger**, added to [`CUSTODY.md`](./CUSTODY.md) (`ROADMAP.md` item 15) alongside explicit notice and the documented-unresponsiveness timeout it already defined. A custodian unwilling to honor the access terms it agreed to is exactly as unfit to keep holding the capsule as one that's collapsed financially, and the remedy — moving the capsule to the next custodian in the chain — doesn't depend on anyone still being around decades later to bring a lawsuit. Whoever currently holds standing (the executor while the estate is open, then successor custodians or trustees themselves afterward) can simply invoke it, per `CUSTODY.md`'s existing procedure.

Consistent with `INTEGRITY.md`'s existing "document the failure honestly, don't pretend nothing happened" precedent for failed fixity checks, a breach should be logged as its own `preservation-log.yaml` entry — reusing `succession_transfer`'s existing shape (`agent`, `outcome`, `note`) rather than inventing a new event type for it.

This doesn't foreclose a real attorney adding a more formal enforcement mechanism (a named party with standing to sue) on top — it just means the succession-chain remedy exists as a baseline that works even if no one is positioned to litigate.

## What's deliberately out of scope here

- **Jurisdiction-specific law** and **tax consequences** — same caveats as `DEED_OF_GIFT.md`.
- **What counts as sufficient documentation of a breach** — left as an open placeholder, same treatment `CUSTODY.md` already gives the unresponsiveness-timeout numbers; a real custodian agreement (item 18) needs to commit to specifics.
- **Unauthorized access that isn't the custodian's fault** (e.g., a security breach by an outside attacker) — `THREAT_MODEL.md` #4 territory technically; this agreement only covers the custodian's own contractual behavior, not third-party intrusion.

## Open questions

- **What if a successor custodian also breaches** — compounding `CUSTODY.md`'s already-flagged "exhausted chain" limitation with bad faith rather than mere collapse. Not solved here; flagged as a real, if hopefully rare, gap.
- **Whether a more formal, named enforcement mechanism should exist alongside the succession-chain remedy** — the option not chosen here, but not foreclosed either; a real attorney reviewing this may reasonably conclude both are needed.

## Status

First pass, written during Phase 3 (Legal), addressing [`ROADMAP.md`](./ROADMAP.md) item 17. Not usable — drafted by non-lawyers, pending review by a licensed attorney. See [`CONTRIBUTING.md`](./CONTRIBUTING.md).
