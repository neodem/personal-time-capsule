# Capsule Format (draft)

Phase 2 (Specification) — see [`ROADMAP.md`](./ROADMAP.md) items 7, 8, and 8a — covering the on-disk directory structure and the `capsule.yaml` file that acts as a capsule's "constitution." Structurally settled and validated against a real example bag (see Status below), but still open to challenge per [`CONTRIBUTING.md`](./CONTRIBUTING.md).

A heavily annotated reference example lives at [`examples/capsule.yaml`](./examples/capsule.yaml), and a small, complete, working example bag — real payload files, real checksums, a `capsule.yaml` instance validated against the schema below — lives at [`examples/minimal-capsule/`](./examples/minimal-capsule/). A machine-checkable schema lives at [`schema/capsule.schema.json`](./schema/capsule.schema.json).

## Directory structure

A capsule is a [BagIt](https://www.rfc-editor.org/rfc/rfc8493) bag (see [`STANDARDS.md`](./STANDARDS.md)), with Personal Time Capsule–specific additions layered on top of BagIt's required layout rather than replacing it:

```
<capsule-root>/
├── bagit.txt                 # BagIt required: version + tag-file encoding
├── bag-info.txt               # BagIt standard: bagging date, size, etc.
├── manifest-sha256.txt        # BagIt required: checksums of everything under data/
├── tagmanifest-sha256.txt     # checksums of the tag files below (capsule.yaml, bag-info.txt, ...)
├── capsule.yaml                # this capsule's constitution — see below
├── privacy-overrides.yaml      # per-item privacy exceptions — see PRIVACY.md. Frozen at sealing, like capsule.yaml
├── preservation-log.yaml       # post-seal event log — see INTEGRITY.md. NOT in either manifest, by design
├── legal/                      # deed of gift, access-restriction agreement, etc. (Phase 3)
│   ├── deed-of-gift.md          # see DEED_OF_GIFT.md — checksummed in tagmanifest, frozen at sealing
│   ├── access-restriction-agreement.md  # see ACCESS_RESTRICTION_AGREEMENT.md — same treatment
│   ├── custodian-agreement.md   # see CUSTODIAN_AGREEMENT.md — same treatment
│   ├── succession-instructions.md  # see SUCCESSION_INSTRUCTIONS.md — same treatment. NOTE: a
│   │                                #   later Certificate of Succession is NOT added here — see below
│   └── creator-instructions.md  # OPTIONAL — see templates/legal/creator-instructions.md.
│                                #   Not a legally binding agreement; a personal worksheet the
│                                #   creator is encouraged to keep updated OUTSIDE the bag instead
└── data/                       # BagIt payload — the actual archival content
    ├── person/                  # human-context layer
    │   ├── biography.md
    │   ├── timeline.md
    │   ├── people.yaml
    │   └── places.yaml
    ├── provenance/               # event/provenance log — see METADATA.md
    │   └── events.yaml           # recommended, not required
    ├── photos/
    ├── email/
    ├── writing/
    ├── audio/
    ├── video/
    └── ...                      # any other creator-defined collection
```

Two decisions worth calling out explicitly:

- **`capsule.yaml` is a tag file, not payload.** It lives at the bag root, alongside `bagit.txt`, and is checksummed via `tagmanifest-sha256.txt` rather than `manifest-sha256.txt`. It describes and governs the capsule; it isn't itself part of the historical record.
- **`privacy-overrides.yaml` is a tag file too, checksummed and frozen at sealing just like `capsule.yaml`.** Privacy screening (`LIFECYCLE.md` stage 4) happens before sealing, so — unlike `preservation-log.yaml` below — there's no ongoing-mutation problem to solve here. See [`PRIVACY.md`](./PRIVACY.md) (`ROADMAP.md` item 12) for what it records and why it's sparse by design.
- **The human-context layer (`data/person/`) *is* payload.** Biography, timeline, people, and places are treated as part of the archival content itself (Principle #6 — the annotations that turn a folder of files into a historical record are as much "the record" as the photos and email are), not as packaging metadata about it. They're checksummed in `manifest-sha256.txt` alongside everything else under `data/`.
- **The event/provenance log (`data/provenance/`) is also payload**, for the same reason — see [`METADATA.md`](./METADATA.md) (`ROADMAP.md` item 10) for what it records and why.
- **`preservation-log.yaml` is a tag file too, but a deliberately unprotected one** — the one file in a capsule that's expected to keep changing after sealing, and the only one intentionally excluded from both `manifest-sha256.txt` and `tagmanifest-sha256.txt`. See [`INTEGRITY.md`](./INTEGRITY.md) (`ROADMAP.md` item 11) for why sealing needed an escape hatch like this at all.
- **`legal/`'s contents are checksummed in `tagmanifest-sha256.txt` and frozen at sealing.** A signed deed of gift or other legal instrument should be tamper-evident the moment it's executed — normally alongside the `custody` decision it formalizes, not something that needs to change afterward. See [`DEED_OF_GIFT.md`](./DEED_OF_GIFT.md) (`ROADMAP.md` item 16).
- **A later Certificate of Succession is deliberately *not* added to `legal/`.** It's executed only if and when an actual succession event occurs, potentially decades after sealing — by which point `legal/` is long since frozen and can't accept new files. It's retained by the assuming custodian as its own record and referenced (not embedded) via a `succession_transfer` entry in `preservation-log.yaml`, the same pattern `ENCRYPTION.md` already uses for trustee share material living outside the bag. See [`SUCCESSION_INSTRUCTIONS.md`](./SUCCESSION_INSTRUCTIONS.md) (`ROADMAP.md` item 19).
- **Creator instructions (`ROADMAP.md` item 20) is optional and, unlike the other four `legal/` documents, not a binding agreement between parties at all** — no institution signs it, so it carries none of the "reviewed by an attorney before use" weight the others do. It's a personal worksheet the creator is actually encouraged to keep *outside* the bag, with their will, since (unlike the sealed contents) it's meant to be updated over the years — a copy placed in `legal/` is frozen at sealing like everything else there, but isn't the recommended primary copy. See `templates/legal/creator-instructions.md`.

Collection directory names under `data/` (`photos/`, `email/`, `writing/`, ...) are creator-defined — nothing in the format requires a fixed taxonomy. `capsule.yaml`'s `collections` list is what actually names and describes them.

### Tag files

`bagit.txt` is fixed, two-line, required-by-RFC-8493 content — every capsule's is identical:

```
BagIt-Version: 1.0
Tag-File-Character-Encoding: UTF-8
```

`bag-info.txt` is a standard BagIt tag file of `Key: Value` metadata lines. This project's profile recommends:

- `Bagging-Date` — when the bag was assembled (Lifecycle stage 5, Sealing)
- `Payload-Oxum` — `<total payload bytes>.<total payload file count>`, BagIt's standard sanity-check field
- `External-Identifier` — should match `capsule.yaml`'s `capsule.id`, so a bag can be cross-checked against its own constitution without parsing YAML
- `Bag-Software-Agent`, `External-Description` — recommended but not required

Fields like `Source-Organization` or `Contact-Name`/`Contact-Email` are standard BagIt fields but intentionally **not** required here — per Principle #2, the creator controls what identifying information appears at all, same as `capsule.yaml`'s `creator` block.

See [`examples/minimal-capsule/bag-info.txt`](./examples/minimal-capsule/bag-info.txt) for a real, filled-in example, and [`examples/minimal-capsule/manifest-sha256.txt`](./examples/minimal-capsule/manifest-sha256.txt) / [`tagmanifest-sha256.txt`](./examples/minimal-capsule/tagmanifest-sha256.txt) for real computed checksums.

## `capsule.yaml`

A machine-checkable schema for this file lives at [`schema/capsule.schema.json`](./schema/capsule.schema.json) (JSON Schema draft 2020-12 — validates the YAML once parsed). One gotcha worth calling out: **date fields (`capsule.created`, `access.default_release_policy.date`) must be written as quoted strings** (`created: "2026-08-30"`), not bare YAML date literals (`created: 2026-08-30`). Unquoted, YAML parsers auto-convert these to native date objects rather than strings, which then fail JSON Schema's `string`/`format: date` check — this bit the example capsules during drafting and would bite any future `ptc verify` implementation the same way.

### `capsule`

Identity of the capsule itself: `spec_version` (`PTC-0.1` for this draft), a stable `id`, the `created` date (Lifecycle stage 1 — Origination), and `creator` info. Per Principle #2, the creator controls what identifying information appears here at all — none of these sub-fields beyond `spec_version`, `id`, and `created` should be treated as required.

### `purpose`

A free-text description of what this particular capsule is. Optional but encouraged — this is a natural place for a creator's own framing of why they built it.

### `series`

Optional — most capsules are a creator's only one and omit this entirely. For an archive that instead spans multiple sequential capsules over a lifetime (`LIFECYCLE.md`'s "Multiple capsules over a lifetime"), `continues` names the immediately preceding capsule's own `capsule.id`, and `note` is free text on the relationship — what period this capsule covers, where the earlier one was deposited, anything worth a future reader knowing.

Deliberately **one-directional**: a capsule can only reference an earlier, already-sealed capsule's known `id` — never a `continued_by` pointing at a later one. An earlier capsule's `capsule.yaml` is frozen forever once sealed (`INTEGRITY.md`), with no mechanism to retroactively add a forward-pointing reference to it even if a creator wanted to. A series is walked backward from the newest capsule, one link at a time, not forward from an index the first capsule couldn't have known to include.

Deliberately **not a merge mechanism**. This field only links two independently-sealed, independently-verifiable bags for a reader's benefit; it doesn't combine them into one. Actually merging capsules would mean reconciling collection names that might collide, two separate custody/succession chains, and encryption keys and quorums that were split independently — real problems this field doesn't attempt to solve. If that ever becomes a genuine need, it's a separate, much larger design question.

### `collections`

Named groupings of payload content under `data/`. Each collection has a `name` and a `path`, and *optionally* a `privacy_zone` and `release_policy` that override the capsule-wide defaults declared in `access` below. A collection that omits either field simply inherits the default — this keeps `capsule.yaml` a bounded, human-readable file rather than an exhaustive per-item ledger. Finer-than-collection overrides (a single restricted item in an otherwise-public collection) belong in `privacy-overrides.yaml`, not here — see [`PRIVACY.md`](./PRIVACY.md) (`ROADMAP.md` item 12).

### `access`

`default_privacy_zone` (one of `public`, `restricted`, `never-release`, per `PRINCIPLES.md` #7 / `THREAT_MODEL.md` #8 / `LIFECYCLE.md` stage 4) and `default_release_policy`, which every collection inherits unless it overrides. Release policy `type` is one of: `immediate`, `fixed_date`, `after_death`, `after_death_plus_years`, `never`, or `custom` — matching the options `README.md` already commits to. What actually satisfies each type (e.g. what confirms an `after_death` trigger) is [`ACCESS.md`](./ACCESS.md) (`ROADMAP.md` item 14).

### `custody`

`primary` custodian plus a `successors` list — the fallback chain from `THREAT_MODEL.md` #3 and `ACTORS.md` (successor custodian), so a capsule isn't orphaned if its first custodian collapses. Left as placeholders (`TBD`) in the example since no real capsule has a custodian yet. This section is a frozen snapshot of the creator's original plan, not a live pointer — once succession actually happens, the real current custodian is tracked in `preservation-log.yaml`, not by rewriting this section. See [`CUSTODY.md`](./CUSTODY.md) (`ROADMAP.md` item 15) for what triggers succession and what it involves procedurally.

### `encryption`

`status` (`planned` / `encrypted` / `none`), `algorithm` (locked to `xchacha20-poly1305`), and `key_custody` (a `quorum` scheme, `slip-0039` as the `splitting_scheme`, and a `trustees` list of bookkeeping-only identity records — never the actual share material) — see [`ENCRYPTION.md`](./ENCRYPTION.md) (`ROADMAP.md` item 13) for why these were chosen and what they don't yet solve. A `collections[]` entry can declare its own full `encryption` override, same convention as `privacy_zone`/`release_policy` — generally necessary whenever a collection has its own `release_policy`, so unlocking one collection doesn't imply unlocking another that happens to share a key.

### `integrity`

`manifest_algorithm: sha256`, matching BagIt's `manifest-sha256.txt` and `STANDARDS.md`'s SHA-256 entry. Not expected to change, but named explicitly so tooling doesn't hardcode an assumption. Fixity-check cadence, what a failed check should trigger, and how post-seal events (fixity checks, deposit, succession transfers) get recorded without reopening the sealed manifest are [`INTEGRITY.md`](./INTEGRITY.md)'s job (`ROADMAP.md` item 11), not this field's.

### `status`

`lifecycle_stage` — which `LIFECYCLE.md` stage this specific capsule instance currently sits in (`origination`, `import`, `annotation`, `screening`, `sealing`, `encryption`, `deposit`, `dormant_preservation`, `release_trigger`, `unlock`, `access`). Lets tooling (and a human inspecting the file) know where a given capsule is in its life without inferring it from side effects.

## Open questions

- ~~The full **privacy model** (item 12) needs to define how per-item overrides work when a collection-level zone isn't granular enough~~ Resolved — see [`PRIVACY.md`](./PRIVACY.md) and `privacy-overrides.yaml`.
- ~~The full **access/release model** (item 14) needs to formalize what each `release_policy.type` actually requires~~ Resolved — see [`ACCESS.md`](./ACCESS.md): trustee quorum doubles as trigger confirmation for `after_death`/`after_death_plus_years`, recommending a two-source evidence minimum.
- ~~The full **custody/succession model** (item 15) needs to define what "successor activation" actually looks like procedurally~~ Resolved — see [`CUSTODY.md`](./CUSTODY.md): explicit-notice-or-unresponsiveness-timeout for custodians, reactive-only re-splitting for trustees, and the honest limit that an exhausted chain has no guaranteed resolution.
- ~~The **encryption model** (item 13) is intentionally undecided here~~ Resolved — see [`ENCRYPTION.md`](./ENCRYPTION.md): `xchacha20-poly1305` and `slip-0039`, with collection-level key granularity.
- **Whether a series should ever support more than a simple backward chain** (item 8a) — e.g. a capsule referencing more than one predecessor, or some notion of branching. Deferred rather than designed under speculation; revisit only if a real multi-capsule creator's needs turn out to actually require it.

## Status

Structurally complete for [`ROADMAP.md`](./ROADMAP.md) items 7 (directory structure) and 8 (`capsule.yaml`): a real example bag ([`examples/minimal-capsule/`](./examples/minimal-capsule/)) exists with computed checksums that pass verification, and its `capsule.yaml` validates against [`schema/capsule.schema.json`](./schema/capsule.schema.json). Every item the "Open questions" section above once listed is now resolved by a dedicated model document (`METADATA.md`, `INTEGRITY.md`, `PRIVACY.md`, `ENCRYPTION.md`, `ACCESS.md`, `CUSTODY.md`) — Phase 2 (Specification) is complete as of item 15. Item 8a (capsule series) was added afterward, prompted by a design conversation about whether a creator should make one capsule or several over a lifetime — a first draft, not yet validated against a real multi-capsule example the way items 7-8 are. Everything here should still be considered open to challenge per `CONTRIBUTING.md` — especially by anyone with real BagIt-profile or archival-metadata experience.
