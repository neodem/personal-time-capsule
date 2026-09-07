# Integrity Model (draft)

Phase 2 (Specification) — [`ROADMAP.md`](./ROADMAP.md) item 11: everything beyond `capsule.yaml`'s `integrity.manifest_algorithm` field — fixity-check cadence, what a failed check triggers, and how a *sealed*, supposedly-frozen capsule accounts for the real things that keep happening to it for decades afterward. A machine-checkable schema for the one new file this adds lives at [`schema/preservation-log.schema.json`](./schema/preservation-log.schema.json); a real, filled-in (if sparse) example lives at [`examples/minimal-capsule/preservation-log.yaml`](./examples/minimal-capsule/preservation-log.yaml).

## What already exists

- `capsule.yaml`'s `integrity.manifest_algorithm: sha256` (see [`FORMAT.md`](./FORMAT.md)) — names the algorithm, nothing more.
- BagIt's `manifest-sha256.txt` (payload) and `tagmanifest-sha256.txt` (tag files), generated once at sealing (`LIFECYCLE.md` stage 5) — a checksum snapshot of the capsule at that moment.

Neither says how often anyone should check those checksums again, what to do if a check fails, or — the gap this document actually starts with — how a capsule accounts for the legitimate events (deposit, fixity checks, succession transfers, eventual release) that keep happening for potentially decades *after* sealing, when the whole point of sealing was that nothing changes anymore.

## The problem sealing creates for itself

[`METADATA.md`](./METADATA.md) (`ROADMAP.md` item 10) added `data/provenance/events.yaml` — an append-only log, but one that's checksummed as ordinary payload. That's fine for everything that happens *before* sealing (origination, imports, annotation, screening, sealing itself). It breaks down for everything that happens *after*: a fixity check six months into dormant preservation has nowhere honest to be recorded, because writing it into `events.yaml` would change a file `manifest-sha256.txt` already committed to, and regenerating the manifest to match would mean "sealed" no longer means "frozen" — it would mean "frozen until the next silent regeneration," which is a much weaker, much harder to verify promise, and one this project hasn't designed any versioning machinery to support.

## `preservation-log.yaml`

The resolution: a **second, deliberately unsealed log**, living at the bag root next to `capsule.yaml` — a tag file, but one **intentionally excluded from both `manifest-sha256.txt` and `tagmanifest-sha256.txt`**. This is the one file in a capsule that's expected to keep changing after sealing, and it's excluded from the fixity net precisely *because* it needs to change — including it would mean either the manifest never stabilizes, or every legitimate append looks identical to tampering.

This keeps the guarantee that matters absolutely intact: everything under `data/` and the three identity-defining tag files (`bagit.txt`, `bag-info.txt`, `capsule.yaml`) stay frozen from the moment of sealing, forever, exactly as `LIFECYCLE.md` stage 5 promises. `preservation-log.yaml` is where the capsule's *custodial* history — as opposed to its *creation* history — accumulates instead, in the same spirit as `data/provenance/events.yaml` but explicitly outside the part of the bag that "sealed" applies to.

It's **recommended, not required**, same as `data/provenance/events.yaml` — but any capsule that's actually been deposited somewhere should have one from the moment of deposit onward.

### Fields

Same shape as `data/provenance/events.yaml` (see `METADATA.md`), with two additional fields for the `reseal` case:

| Field | Required | Description |
|---|---|---|
| `date` | yes | ISO 8601 date, quoted. |
| `type` | yes | One of the event types below. |
| `agent` | yes | Free text: who or what performed the event. |
| `id` | no | Optional stable identifier. |
| `collection` | no | Matches a `collections[].name` in `capsule.yaml`, when relevant. |
| `outcome` | no | Free-text result — most relevant to `fixity_check`. |
| `note` | no | Anything else worth recording. |
| `manifest_checksum_before` | no | `reseal` only — see below. |
| `manifest_checksum_after` | no | `reseal` only — see below. |

### Event types

`encryption`, `deposit`, `dormant_preservation`, `release_trigger`, `unlock`, `access` — the post-seal tail of `capsule.yaml`'s `status.lifecycle_stage` enum, same convention `METADATA.md` established for the pre-seal types in `events.yaml`. `encryption` (`LIFECYCLE.md` stage 6) belongs here, not in `events.yaml`, precisely because it happens *after* sealing (stage 5) — see [`ENCRYPTION.md`](./ENCRYPTION.md) (`ROADMAP.md` item 13), which is what caught this originally being misplaced.

`fixity_check`, `succession_transfer`, `trustee_attestation` — the recurring sub-events `LIFECYCLE.md` stage 8 names.

`reseal` — the rare, exceptional case where a capsule's payload manifest is legitimately regenerated after sealing (e.g. a real error discovered in the original bagging, or a later, deliberate multi-part release where a privacy-zoned collection is resealed on its own schedule — the latter likely intersects with `ROADMAP.md` item 14, access/release model). A `reseal` entry must record `manifest_checksum_before` and `manifest_checksum_after`, plus a `note` explaining why. A verifier who finds `manifest-sha256.txt` doesn't match what they last recorded should treat that as a serious red flag *unless* a corresponding `reseal` entry accounts for it — see Open questions, below, for the honest limits of that defense.

### Worked example

```yaml
# preservation-log.yaml
- date: "2026-09-15"
  type: encryption
  agent: "Jane Smith (creator); ptc encrypt v0.1"
  collection: email
  note: "XChaCha20-Poly1305, 3-of-5 SLIP-0039 quorum — see ENCRYPTION.md"

- date: "2026-10-01"
  type: deposit
  agent: "Springfield Historical Society"
  outcome: "Manifest verified on receipt; accepted per deposit agreement"

- date: "2027-10-01"
  type: fixity_check
  agent: "Springfield Historical Society"
  outcome: "pass"

- date: "2028-10-03"
  type: fixity_check
  agent: "Springfield Historical Society"
  outcome: "checksum mismatch: data/photos/1998-07-04-lakehouse.jpg"
  note: "Restored from successor custodian's replica 2028-10-05; see next entry"

- date: "2028-10-05"
  type: fixity_check
  agent: "Springfield Historical Society"
  outcome: "pass"
  note: "Re-verified after restoration from Riverside Public Library's replica"
```

## Fixity-check cadence

Recommended, not mandated: **at least annually** during dormant preservation, plus immediately after any custody or succession transfer, and before/after any physical media migration. This document sets the expectation; an actual enforceable commitment belongs in the Phase 3 custodian agreement (`ROADMAP.md` item 18), once real custodian relationships exist to know what's realistic. Per Principle #9 (no brittle promises), this spec doesn't itself obligate any custodian to a schedule it has no way to enforce.

## What a failed fixity check should trigger

A recommended default, again not a hard mandate the spec can enforce on its own:

1. **Quarantine, don't delete or overwrite.** A failed check means "this copy no longer matches its recorded checksum," not "this copy is worthless" — don't destroy potential forensic or partial-recovery value.
2. **Notify** the creator (if reachable), key-holding trustees, and the rest of the custodian succession chain.
3. **Attempt restoration** from another custodian's independently verified replica, per the LOCKSS philosophy (`STANDARDS.md`, `THREAT_MODEL.md` #1) — this is exactly the scenario multiple independent copies exist for.
4. **Log the outcome either way** in `preservation-log.yaml` — including "unrecoverable" if restoration fails. Silent loss is worse than documented loss; a future researcher is better served by an honest gap than by a capsule that quietly pretends nothing happened.

As with cadence, the real obligations (who must do what, by when) are Phase 3 (`ROADMAP.md` item 18) territory — this document says what a *good* response looks like, not what any given custodian is contractually bound to.

## Open questions

- **`preservation-log.yaml` is not itself tamper-evident.** It's deliberately outside the fixity net (that's the whole point), which means it's also, honestly, editable by anyone with write access to the capsule's storage — including a custodian trying to cover up a failed check or an unauthorized access. This document doesn't solve that; it documents the record, it doesn't cryptographically prove the record wasn't altered. A future revision could hash-chain entries (each entry committing to the previous one's hash) to at least make silent edits to *past* entries detectable — judged not worth the added complexity for a personal-scale v0.1, but flagged here for real reconsideration, especially given `THREAT_MODEL.md` #9 (insider threat).
- **Who's authorized to append isn't defined yet.** Any custodian in the chain? Only the current primary? Does a newly activated successor custodian's first entry need to establish its own provenance somehow? This overlaps with `ROADMAP.md` item 15 (custody/succession model) and is deferred to it.
- **Cadence and failed-check response are recommendations here, not enforced mechanisms.** `ROADMAP.md` item 18 (custodian agreement) is where an actual commitment gets made; this document exists so that agreement has something concrete to point at instead of starting from nothing.

## Status

First pass, written during Phase 2 (Specification), addressing [`ROADMAP.md`](./ROADMAP.md) item 11. The `preservation-log.yaml` / `data/provenance/events.yaml` split is a direct response to a gap `METADATA.md` flagged in its own "Open questions" section — still open to challenge per [`CONTRIBUTING.md`](./CONTRIBUTING.md), particularly whether a second file is the right resolution versus something simpler.
