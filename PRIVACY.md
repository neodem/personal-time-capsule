# Privacy Model (draft)

Phase 2 (Specification) — [`ROADMAP.md`](./ROADMAP.md) item 12: per-item privacy overrides when a collection-level zone isn't granular enough, and a starter vocabulary for sensitive-data-scanning categories. Implements [`PRINCIPLES.md`](./PRINCIPLES.md) #7 (privacy is fundamental, not an afterthought) and responds to [`THREAT_MODEL.md`](./THREAT_MODEL.md) #8 (privacy harm to third parties). A machine-checkable schema for the one new file this adds lives at [`schema/privacy-overrides.schema.json`](./schema/privacy-overrides.schema.json); a real (if sparse) example lives at [`examples/minimal-capsule/privacy-overrides.yaml`](./examples/minimal-capsule/privacy-overrides.yaml).

## What already exists

`capsule.yaml`'s `access.default_privacy_zone` sets a capsule-wide default (`public` / `restricted` / `never-release`, per `PRINCIPLES.md` #7), and each named `collections[]` entry can override it — see [`FORMAT.md`](./FORMAT.md). That covers "this whole email collection is restricted" but not "this *one* email in an otherwise-public collection needs to stay hidden" — exactly the gap `FORMAT.md`'s own "Open questions" section flagged when item 8 was written.

## `privacy-overrides.yaml`

A new root-level tag file, alongside `capsule.yaml` — **one consolidated list for the whole capsule**, not scattered per-collection sidecar files, so a creator (or a future auditor) can see every exception to the default in one place rather than having to check every collection directory individually. Each entry names one file, relative to the bag root, and what's different about it.

It's **sparse by design**: only items that *differ* from their collection's (or the capsule's default) zone need an entry. A 4,000-message email export with 12 flagged messages needs 12 entries, not 4,000 — the same "don't force exhaustive per-item cataloging" reasoning `METADATA.md` already applied to the event log.

**It's a tag file, checksummed in `tagmanifest-sha256.txt`, frozen at sealing — same as `capsule.yaml`.** Privacy screening (`LIFECYCLE.md` stage 4) happens *before* sealing (stage 5) as part of the normal, pre-seal workflow, so there's no ongoing-mutation problem here the way there was for post-seal events (`INTEGRITY.md`, item 11). If a real privacy issue is discovered *after* sealing — someone finds an unredacted SSN in an already-sealed, already-deposited capsule — that's an exceptional correction, not routine editing, and it should go through the same `reseal` event `INTEGRITY.md` already defined for exactly this kind of rare, after-the-fact fix: log it in `preservation-log.yaml` with the manifest checksums before and after, and why.

It's **recommended, not required** — a capsule where screening surfaced zero exceptions doesn't need the file at all, same as `data/provenance/events.yaml` and `preservation-log.yaml`.

### Fields

| Field | Required | Description |
|---|---|---|
| `path` | yes | Exact path to one payload file, relative to the bag root (e.g. `"data/email/2011-03-02-medical-bill.eml"`). No wildcards or directory-level matching in this version — see Open questions. |
| `privacy_zone` | no | Overrides this item's zone. Same vocabulary as `capsule.yaml`: `public` / `restricted` / `never-release`. |
| `release_policy` | no | Overrides this item's release timing independently of `privacy_zone` — same shape as `capsule.yaml`'s release policy. Lets one item release later (or earlier) than the rest of its collection without also changing whether it's public/restricted/never. |
| `category` | no | Free text — see "Sensitive-data-scanning categories," below. |
| `reason` | no | Free-text explanation. |
| `flagged_by` | no | Free text: who or what identified the need for this override (`"Jane Smith (creator)"`, `"ptc scan v0.1 (creator-confirmed)"`) — same convention as `data/provenance/events.yaml`'s `agent` field. |

An entry can set `privacy_zone`, `release_policy`, or both — a photo that should simply release sooner than its collection doesn't need a zone change to justify a release-timing override.

### Worked example

```yaml
# privacy-overrides.yaml
- path: data/email/2011-03-02-medical-bill.eml
  privacy_zone: never-release
  category: medical
  reason: "Contains provider name and a diagnosis code"
  flagged_by: "Jane Smith (creator)"

- path: data/email/2014-06-19-friend-cosign.eml
  privacy_zone: restricted
  category: third-party-privacy
  reason: "A friend's SSN, shared for a loan cosign; redact before release"
  flagged_by: "ptc scan v0.1 (creator-confirmed)"

- path: data/photos/2020-family-reunion-banner.jpg
  release_policy:
    type: immediate
  reason: "Already public elsewhere; no reason to hold back with the rest of the photos collection"
  flagged_by: "Jane Smith (creator)"
```

## Sensitive-data-scanning categories

An **open, recommended vocabulary**, not a closed enum — `category` in the schema accepts any string. This matches the precedent `capsule.schema.json` already set with `encryption.key_custody.scheme`: the real taxonomy isn't settled by field experience yet (no `ptc scan` exists — that's `ROADMAP.md` item 24, and Phase 5's real-world validation is where a taxonomy like this actually gets tested), so this document recommends a starting point rather than locking one in prematurely.

Recommended starter categories, drawn directly from `THREAT_MODEL.md` #8:

- **`financial`** — account numbers, routing numbers, transaction details
- **`credentials`** — passwords, API keys, security question answers left in files
- **`medical`** — diagnoses, provider names, prescriptions, health records
- **`government-id`** — SSNs, passport/license numbers, other identity numbers
- **`third-party-privacy`** — anything identifying a person other than the creator who never consented to being archived (`ACTORS.md`'s named subject/third party role — this is the category that exists specifically because of them)
- **`other`** — anything sensitive that doesn't fit the above; `reason` should carry the actual explanation

Creators and future tooling are free to use other category strings; these six just give `ptc scan` (once it exists) and creators doing this by hand a shared starting vocabulary instead of each capsule inventing its own from scratch.

## What's deliberately out of scope here

- **Redaction mechanics.** *Whether* an item is redacted, and how a redacted copy relates to its original (kept alongside in a stricter zone? overwritten entirely?), is `LIFECYCLE.md` stage 4's job to describe and `ROADMAP.md` item 25 (`ptc redact`) to implement — this document only covers *classifying* an item's privacy zone, not modifying its content.
- **Automated scanning itself.** What patterns or techniques actually detect an SSN or a credential in the wild is `ROADMAP.md` item 24 (`ptc scan`) — a Phase 4 tooling concern. This document only defines where a scan's *findings*, once a creator confirms them, get recorded.
- **Wildcard or directory-level path matching.** Every override names one exact file. Real capsules may eventually want "everything under `data/email/2003/`" without listing each message — deferred rather than designed under speculation; see Open questions.

## Open questions

- **No pattern matching yet.** Exact-path-only entries could get unwieldy for a creator who wants to restrict, say, an entire year of correspondence at once, short of making it its own `collections[]` entry. Worth revisiting once real capsules (Phase 5) show how common that actually is.
- **Nothing validates that `path` actually exists or belongs to a declared collection.** Same limitation `METADATA.md` already flagged for `data/provenance/events.yaml`'s `collection` field — acceptable for a hand-written file today, worth a `ptc verify` structural check later (Phase 4).
- **Precedence when a collection-level and item-level policy conflict isn't formally stated**, though the intent above is that item-level always wins when present — should be made explicit and testable once `ROADMAP.md` item 14 (access/release model) defines how `release_policy` is actually evaluated end-to-end.

## Status

First pass, written during Phase 2 (Specification), addressing [`ROADMAP.md`](./ROADMAP.md) item 12. Still open to challenge per [`CONTRIBUTING.md`](./CONTRIBUTING.md) — particularly the decision to keep this sparse and exact-path-only rather than supporting patterns from the start.
