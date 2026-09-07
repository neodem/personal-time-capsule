# Metadata Model (draft)

Phase 2 (Specification) — [`ROADMAP.md`](./ROADMAP.md) item 10: PREMIS-aligned object/event/provenance fields, per [`STANDARDS.md`](./STANDARDS.md)'s PREMIS entry. This document defines the one new file this adds to a capsule (`data/provenance/events.yaml`) and what it's for. A machine-checkable schema lives at [`schema/events.schema.json`](./schema/events.schema.json); a real, filled-in example lives at [`examples/minimal-capsule/data/provenance/events.yaml`](./examples/minimal-capsule/data/provenance/events.yaml).

## Why this exists

Two kinds of metadata already exist elsewhere in a capsule, and neither one records *history*:

- `capsule.yaml`'s `status.lifecycle_stage` (see [`FORMAT.md`](./FORMAT.md)) is a single current-stage snapshot — it says where a capsule is *right now*, not how it got there.
- BagIt's `manifest-sha256.txt` proves *what* the payload is at sealing time — it says nothing about *how* it was assembled or by whom.

PREMIS's core contribution is the idea that an archival object's trustworthiness depends on its documented history as much as its content: what happened to it, in what order, by whom. That's what's missing, and what `data/provenance/events.yaml` is for. It directly serves:

- **`THREAT_MODEL.md` #3** (institutional collapse) — "capsule metadata is self-describing enough that a *new*, previously uninvolved institution can pick it up cold." An event log is exactly the self-description a successor custodian needs.
- **`THREAT_MODEL.md` #5** (unauthorized modification or forgery) and **#9** (insider threat) — a documented history makes it easier to notice something that doesn't add up, even though the log itself can't be cryptographically tamper-proof on its own (see Open questions, below).
- **`ACTORS.md`**'s auditor/verifier role — someone checking a capsule's integrity benefits from knowing its assembly history, not just its current checksums.

## Scope decision: event/provenance log, not per-file object records

Full PREMIS practice typically includes a metadata record for every digital object (format, technical characteristics, a provenance link). This document deliberately does **not** require that. Two reasons:

1. **Principle #6 (preserve the mess) and `FORMAT.md`'s precedent that `capsule.yaml` stays a bounded, readable file rather than an exhaustive per-item ledger.** A capsule can hold tens of thousands of photos; a hand- or tool-maintained record per file would work against both.
2. **It's largely redundant with what already exists.** Every payload file already has a path and a SHA-256 checksum in `manifest-sha256.txt`. Format identification (what a file *is*) is recoverable from the file itself at any time by any future tool — it doesn't need to be pre-recorded to survive.

So the model here operates at **event** and **batch/collection** granularity, not per-file:

- **Events** — things that happened to the capsule as a whole, or to one named collection: origination, an import batch, a screening pass, sealing, a fixity check, a succession transfer.
- **Provenance** — for import events specifically, *where a batch of material came from* (e.g. "Gmail Takeout export, jane@example.org, exported 2026-08-01"), attached to that event rather than to each file it brought in.

If a real future need for per-file records emerges (Phase 4 tooling, or real-world validation in Phase 5), it can be added later as an additional, separate file — nothing here forecloses that; it just isn't required now.

## `data/provenance/events.yaml`

An append-only, ordered list of events. Lives under `data/`, alongside `data/person/` — **payload, not a tag file** (checksummed in `manifest-sha256.txt`, not `tagmanifest-sha256.txt`). This follows the same reasoning `FORMAT.md` already applied to `data/person/`: the history of how an archive was assembled is itself historically useful to a future reader, not mere packaging metadata about the archive. It's kept as its own directory rather than folded into `data/person/`, because it's the record of the *capsule's* assembly, not the *creator's* biography — those are different stories that happen to share a bag.

It's **recommended, not required**. A capsule with no events file is still a structurally valid capsule (consistent with `LIFECYCLE.md` stage 3's "no required minimum" for annotation) — but every capsule this project's own tooling produces should write to it from the first `ptc init` onward, since a log that starts on day one is far more useful than one bolted on retroactively.

### Fields

Each entry is an object:

| Field | Required | Description |
|---|---|---|
| `date` | yes | ISO 8601 date the event occurred, quoted (same YAML gotcha as `capsule.yaml` — see `FORMAT.md`). |
| `type` | yes | One of the event types below. |
| `agent` | yes | Free text: who or what performed the event — a person plus their [`ACTORS.md`](./ACTORS.md) role (`"Jane Smith (creator)"`), or software plus version (`"ptc CLI v0.1"`). |
| `id` | no | A stable identifier for this event (`"evt-2026-08-30-import-email"`), in case something needs to reference it later. |
| `collection` | no | When the event concerns one named collection rather than the whole capsule, matches a `collections[].name` in `capsule.yaml`. |
| `source` | no | Free-text provenance — primarily for `import` events: where did this batch come from. |
| `outcome` | no | Free-text result, most relevant to `fixity_check` (`"pass"`, `"3 files failed checksum"`). |
| `note` | no | Anything else worth recording. |

### Event types

The first 11 match `capsule.yaml`'s `status.lifecycle_stage` enum one-for-one — the same vocabulary describing "where a capsule is" also describes "what happened," which is deliberate:

`origination`, `import`, `annotation`, `screening`, `sealing`, `encryption`, `deposit`, `dormant_preservation`, `release_trigger`, `unlock`, `access`

Three more cover the recurring sub-events `LIFECYCLE.md` stage 8 names, which aren't stage transitions themselves:

`fixity_check`, `succession_transfer`, `trustee_attestation`

A capsule can log the same type more than once — e.g. multiple `import` events over years, as the amend loop (`LIFECYCLE.md`) is used, or repeated `fixity_check` events throughout dormant preservation. `capsule.yaml`'s `status.lifecycle_stage` should be updated to match whenever a new stage-level event (the first 11 types) is logged; the log is the history, the status field is the current position.

### Worked example

```yaml
# data/provenance/events.yaml
- date: "2026-08-29"
  type: origination
  agent: "Jane Smith (creator)"

- date: "2026-08-30"
  type: import
  agent: "Jane Smith (creator)"
  collection: email
  source: "Gmail Takeout export, jane@example.org"
  note: "~4,200 messages, 2003–2026"

- date: "2026-09-05"
  type: screening
  agent: "Jane Smith (creator); ptc scan v0.1"
  collection: email
  outcome: "12 items flagged and set to restricted"

- date: "2026-09-10"
  type: sealing
  agent: "ptc CLI v0.1"
```

## What's deliberately out of scope here

- **Per-file (PREMIS Object) format/technical metadata.** See "Scope decision," above — deferred to tooling computing it on demand from the payload itself, not pre-recorded.
- **PREMIS Rights entities.** Access and privacy are governed by `capsule.yaml`'s `access` model and the eventual Phase 3 legal documents, not duplicated here.
- **A structured Agent registry.** `agent` is free text for now. If the encryption or custody/succession models (`ROADMAP.md` items 13, 15) end up needing real trustee/agent identity records, that can extend this file later — every field beyond `date`/`type`/`agent` is additive, and nothing here should have to change shape to accommodate it.

## Open questions

- **This file isn't self-protecting after sealing.** Sealing fixes `manifest-sha256.txt` against the payload as of that moment, but a capsule keeps generating legitimate events afterward (deposit, fixity checks, eventually succession or release). Whether/how the manifest gets re-issued to cover later, legitimate appends — versus treating any post-seal change as suspect by default — is really the integrity model's problem (`ROADMAP.md` item 11), not resolved here.
- **Fixity-check cadence** (how often, triggered by whom) is exactly what item 11 needs to define; this document just gives fixity checks a place to be logged once that cadence exists.
- **Collection cross-references aren't enforced.** `collection` is meant to match a name in `capsule.yaml`'s `collections` list, but nothing (yet) validates that they stay in sync — acceptable for a hand-written file today, worth revisiting if `ptc verify` (Phase 4) starts checking capsules structurally.

## Status

First pass, written during Phase 2 (Specification), addressing [`ROADMAP.md`](./ROADMAP.md) item 10. Still open to challenge per [`CONTRIBUTING.md`](./CONTRIBUTING.md) — especially the granularity decision (event/batch-level vs. full per-file PREMIS records), which was a judgment call, not a settled archival consensus.
