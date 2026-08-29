# Capsule Format (draft)

This is the first pass at Phase 2 (Specification) — see [`ROADMAP.md`](./ROADMAP.md) items 7 and 8 — covering the on-disk directory structure and the `capsule.yaml` file that acts as a capsule's "constitution." It's deliberately a first draft — see [`CONTRIBUTING.md`](./CONTRIBUTING.md) if something here should be challenged.

A worked example lives at [`examples/capsule.yaml`](./examples/capsule.yaml).

## Directory structure

A capsule is a [BagIt](https://www.rfc-editor.org/rfc/rfc8493) bag (see [`STANDARDS.md`](./STANDARDS.md)), with Personal Time Capsule–specific additions layered on top of BagIt's required layout rather than replacing it:

```
<capsule-root>/
├── bagit.txt                 # BagIt required: version + tag-file encoding
├── bag-info.txt               # BagIt standard: baging date, size, etc.
├── manifest-sha256.txt        # BagIt required: checksums of everything under data/
├── tagmanifest-sha256.txt     # checksums of the tag files below (capsule.yaml, bag-info.txt, ...)
├── capsule.yaml                # this capsule's constitution — see below
├── legal/                      # deed of gift, access-restriction agreement, etc. (Phase 3 templates)
│   └── ...
└── data/                       # BagIt payload — the actual archival content
    ├── person/                  # human-context layer
    │   ├── biography.md
    │   ├── timeline.md
    │   ├── people.yaml
    │   └── places.yaml
    ├── photos/
    ├── email/
    ├── writing/
    ├── audio/
    ├── video/
    └── ...                      # any other creator-defined collection
```

Two decisions worth calling out explicitly:

- **`capsule.yaml` is a tag file, not payload.** It lives at the bag root, alongside `bagit.txt`, and is checksummed via `tagmanifest-sha256.txt` rather than `manifest-sha256.txt`. It describes and governs the capsule; it isn't itself part of the historical record.
- **The human-context layer (`data/person/`) *is* payload.** Biography, timeline, people, and places are treated as part of the archival content itself (Principle #6 — the annotations that turn a folder of files into a historical record are as much "the record" as the photos and email are), not as packaging metadata about it. They're checksummed in `manifest-sha256.txt` alongside everything else under `data/`.

Collection directory names under `data/` (`photos/`, `email/`, `writing/`, ...) are creator-defined — nothing in the format requires a fixed taxonomy. `capsule.yaml`'s `collections` list is what actually names and describes them.

## `capsule.yaml`

### `capsule`

Identity of the capsule itself: `spec_version` (`PTC-0.1` for this draft), a stable `id`, the `created` date (Lifecycle stage 1 — Origination), and `creator` info. Per Principle #2, the creator controls what identifying information appears here at all — none of these sub-fields beyond `spec_version`, `id`, and `created` should be treated as required.

### `purpose`

A free-text description of what this particular capsule is. Optional but encouraged — this is a natural place for a creator's own framing of why they built it.

### `collections`

Named groupings of payload content under `data/`. Each collection has a `name` and a `path`, and *optionally* a `privacy_zone` and `release_policy` that override the capsule-wide defaults declared in `access` below. A collection that omits either field simply inherits the default — this keeps `capsule.yaml` a bounded, human-readable file rather than an exhaustive per-item ledger (per-item detail, if a capsule ever needs it, belongs in a separate per-collection manifest, not here).

### `access`

`default_privacy_zone` (one of `public`, `restricted`, `never-release`, per `PRINCIPLES.md` #7 / `THREAT_MODEL.md` #8 / `LIFECYCLE.md` stage 4) and `default_release_policy`, which every collection inherits unless it overrides. Release policy `type` is one of: `immediate`, `fixed_date`, `after_death`, `after_death_plus_years`, `never`, or `custom` — matching the options `README.md` already commits to. The full semantics of each policy type are [`ROADMAP.md`](./ROADMAP.md) item 14 (access/release model) and aren't fully specified yet; this schema just fixes the vocabulary.

### `custody`

`primary` custodian plus a `successors` list — the fallback chain from `THREAT_MODEL.md` #3 and `ACTORS.md` (successor custodian), so a capsule isn't orphaned if its first custodian collapses. Left as placeholders (`TBD`) in the example since no real capsule has a custodian yet.

### `encryption`

**Deliberately a stub.** `status` (`planned` / `encrypted` / `none`) and the *shape* of key custody (a `quorum` scheme with `required`-of-`of` trustees) are declared so tooling has something stable to build against, but no algorithm or key-splitting scheme is named yet — that's [`ROADMAP.md`](./ROADMAP.md) item 13 (encryption model), a separate, dedicated design discussion, not something to back into via this file.

### `integrity`

`manifest_algorithm: sha256`, matching BagIt's `manifest-sha256.txt` and `STANDARDS.md`'s SHA-256 entry. Not expected to change, but named explicitly so tooling doesn't hardcode an assumption.

### `status`

`lifecycle_stage` — which `LIFECYCLE.md` stage this specific capsule instance currently sits in (`origination`, `import`, `annotation`, `screening`, `sealing`, `encryption`, `deposit`, `dormant_preservation`, `release_trigger`, `unlock`, `access`). Lets tooling (and a human inspecting the file) know where a given capsule is in its life without inferring it from side effects.

## Open questions

- The full **privacy model** ([`ROADMAP.md`](./ROADMAP.md) item 12) needs to define how per-item overrides work when a collection-level zone isn't granular enough (e.g. one restricted email in an otherwise-public collection).
- The full **access/release model** (item 14) needs to formalize what each `release_policy.type` actually requires (e.g. what confirms an `after_death` trigger — see `ACTORS.md`, executor/estate representative).
- The full **custody/succession model** (item 15) needs to define what "successor activation" actually looks like procedurally, beyond the chain being declared.
- The **encryption model** (item 13) is intentionally undecided here; `encryption.key_custody.scheme: quorum` assumes *a* threshold scheme exists but doesn't commit to Shamir's Secret Sharing or any other specific mechanism.

## Status

First pass, written during Phase 2 (Specification), covering [`ROADMAP.md`](./ROADMAP.md) items 7 (directory structure) and 8 (`capsule.yaml`). Everything here should be considered a draft open to challenge per `CONTRIBUTING.md` — especially by anyone with real BagIt-profile or archival-metadata experience.
