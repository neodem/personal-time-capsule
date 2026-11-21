# Lifecycle

The stages a capsule passes through, from a creator's first decision to make one, through however many decades of dormant preservation, to eventual access. This is meant to describe the *shape* of the process for Phase 2 (Specification) to formalize, not to lock in specific file formats or commands — see `README.md` for the eventual CLI (`init`, `import`, `inspect`, `scan`, `redact`, `encrypt`, `verify`, `unlock`, `deposit`), which will roughly map onto these stages.

Roles referenced below are defined in [`ACTORS.md`](./ACTORS.md).

```
 ORIGINATION → IMPORT → ANNOTATION → SCREENING → SEALING → ENCRYPTION
      ▲            ▲         ▲                                  │
      └────────────┴─────────┘  (amend loop, while unsealed)     ▼
                                                              DEPOSIT
                                                                  │
                                                                  ▼
                                                   DORMANT PRESERVATION ◄──┐
                                                          │                │
                                              (succession event) ──────────┘
                                                          │
                                                  RELEASE TRIGGER
                                                          │
                                                          ▼
                                                       UNLOCK
                                                          │
                                                          ▼
                                                       ACCESS
```

## 1. Origination

The creator decides to start a capsule. No files have moved yet — this is the decision to begin, and (informally) the first thinking about scope: what parts of a life, which accounts, which formats, roughly how far back.

- **Actor:** creator (or assistant, on their behalf).
- **Exit condition:** creator has something to import — even one account export or one folder is enough to start.

## 2. Import

Raw records — email exports, photo libraries, documents, audio, video — are brought into the capsule's directory structure in their native, standard formats (Principle #3). Nothing is transformed or judged yet; this stage is mechanical.

- **Actor:** creator/assistant.
- **Notable:** Principle #6 (preserve the mess) applies from this stage forward — import should never require sorting, renaming, or cleanup as a precondition.
- **Exit condition:** the records the creator wants in this capsule (for now — see the amend loop below) are present in the working structure.

## 3. Annotation

The human-context layer goes on: biography, timeline, who's who, what places and events mattered, why a given batch of records is there. This is what turns a folder of files into a legible historical record (per `VISION.md`).

- **Actor:** creator/assistant.
- **Notable:** additive only — annotation never edits or replaces the underlying raw files (Principle #6).
- **Exit condition:** as much or as little annotation as the creator wants; this stage has no required minimum.

## 4. Screening

Sensitive-data scanning runs across the imported material; the creator reviews findings and assigns privacy zones (public / restricted / never-release) at whatever granularity the spec ends up supporting (per-item, per-collection). Redaction happens here if the creator chooses it.

- **Actor:** creator/assistant, using scanning/redaction tooling; the tooling surfaces candidates, the creator decides (Principle #2 — the creator, not the software, has final say).
- **Notable:** this stage exists specifically to protect third parties who never consented to being archived (`ACTORS.md` — named subject) as well as the creator's own sensitive material.
- **Exit condition:** every item has an assigned privacy zone, explicit or default.

## 5. Sealing

Cryptographic integrity manifests (checksums, per BagIt) are generated over the capsule's contents. From this point, any modification is detectable.

- **Actor:** tooling, triggered by the creator.
- **Exit condition:** a manifest exists and validates against current contents.

## 6. Encryption & policy binding

The creator chooses (or confirms) a release policy — immediate, a fixed date, "after death," "after death + N years," possibly different policies per collection (per `README.md`). The capsule (or the relevant privacy-zoned portions of it) is encrypted, and keys or key-shares are distributed to the chosen trustee(s), per whatever quorum scheme the creator selected.

- **Actors:** creator (decision), key-holding trustee(s) (receipt of key material).
- **Notable:** this is where Principle #8 (preservation and access are separate) becomes concrete — a custodian receiving the capsule after this point can hold and verify it without necessarily being able to read it.
- **Exit condition:** capsule is encrypted per its stated policy; trustees have confirmed receipt of their key material.

### Amend loop

Stages 1–4 (and arguably 5) aren't strictly linear. A creator can return to import, annotation, or screening at any time *before* sealing — adding new material years later, revising annotations, re-screening. Sealing is the natural point where "still being built" becomes "ready to deposit," but nothing requires a creator to seal immediately, and a capsule can stay in this loop indefinitely, or even permanently, if a creator only ever wants a personal archive and never deposits it anywhere.

## 7. Deposit

The sealed, encrypted capsule is transferred to a custodian institution (or several, for redundancy per LOCKSS philosophy). The custodian verifies the manifest on receipt and confirms acceptance.

- **Actors:** creator/assistant → custodian institution.
- **Notable:** at real-world scale (`FORMAT.md`'s "Scale" section puts a thorough personal archive at low hundreds of GB to low single-digit TB) this is usually a physical-media transfer, not a network upload — see `CUSTODY.md`'s "Initial deposit" section for the actual mechanics.
- **Exit condition:** custodian has an independently verified copy and has agreed (per whatever Phase 3 deposit agreement applies) to hold it.

## 8. Dormant preservation

The capsule sits with its custodian(s), held and periodically fixity-checked (integrity re-verified against the manifest) without being read. This is the longest stage by far — potentially decades.

- **Actors:** custodian institution(s); auditor/verifier role can run integrity checks at any point without needing access.
- **Ongoing sub-events, not full stage transitions:**
  - **Succession event** (`THREAT_MODEL.md` #3): if a custodian can no longer fulfill its role, the capsule (or a verified replica) transfers to the next custodian in the chain, and dormant preservation continues there. This can happen multiple times over a capsule's life and doesn't affect the release policy. See `CUSTODY.md` for what triggers this and what activation involves procedurally.
  - **Trustee attestation**: key-holding trustees periodically confirm they can still produce their key/share when needed. A trustee becoming unreachable is a risk to flag (`THREAT_MODEL.md` #7), potentially triggering a trustee-succession process analogous to custodian succession — see `CUSTODY.md`, including why this can only happen while a live quorum still exists.

## 9. Release trigger

The condition specified in the release policy is met — a calendar date arrives, a death is confirmed (per whatever process the executor/estate representative role formalizes), or another documented condition fires. See `ACCESS.md` for what actually satisfies each `release_policy.type`.

- **Actors:** whoever the policy names as responsible for confirming the trigger (custodian, executor, trustees themselves).
- **Exit condition:** trigger is confirmed and documented; unlock can proceed.

## 10. Unlock

Trustees provide their keys or key-shares; if a quorum scheme is in place, enough of them cooperate to reconstruct decryption capability. The capsule (or the relevant privacy-zoned portion of it, if release is staged) is decrypted.

- **Actors:** key-holding trustee(s).
- **Notable:** a quorum requirement here is the main technical defense against both a single coerced trustee and a single trustee becoming permanently unreachable (`THREAT_MODEL.md` #6, #7).
- **Exit condition:** decrypted content is available to whichever custodian or party the policy designates as the release point.

## 11. Access

The beneficiary — named individuals, an institution, or the public, per the creator's original choice — can now read the capsule: standard-format files plus the human-context layer, openable with nothing more than a filesystem, ordinary non-proprietary software, and the published specification (Principle #3).

- **Actors:** beneficiary/recipient; eventually, future researcher/historian.
- **Notable:** access doesn't have to be all-or-nothing if the creator set different policies per collection (`README.md`) — a "never-release" privacy zone, for instance, may never reach this stage even after everything else does.
- **Exit condition:** none — this is the terminal, intended-to-be-permanent state. The capsule continues to sit with its custodian(s) under ordinary preservation practice indefinitely.

## Multiple capsules over a lifetime

Nothing above requires a creator to have exactly one capsule, ever. The amend loop (stages 1–4) lets a single capsule grow indefinitely before sealing, but sealing and deposit are what actually get material to safety — LOCKSS-style redundancy across custodians (`THREAT_MODEL.md` #1) doesn't start until then. Waiting to seal until an archive feels "finished" risks holding everything hostage to that feeling for years or decades, which is exactly the kind of loss `THREAT_MODEL.md` already worries about.

A creator can instead seal and deposit a capsule, then originate a new one later that picks up where it left off — `capsule.yaml`'s optional `series.continues` field (`FORMAT.md`, `ROADMAP.md` item 8a) names the earlier capsule's own `id`, so a custodian or future researcher can walk the chain. This is deliberately one-directional: a capsule can only reference an earlier, already-sealed capsule's known `id`, never a later one, since the earlier capsule's `capsule.yaml` is frozen forever once sealed (`INTEGRITY.md`) — there's no mechanism to retroactively add a forward-pointing reference to it. A series is read backward from the newest capsule, not forward from an index the first one couldn't have known to include.

This deliberately doesn't define a mechanism for merging multiple capsules into one — see `FORMAT.md`'s `series` section for why that's a much harder problem (reconciling collection names, custody chains, and independently-split encryption keys across two independently-sealed bags) than simply linking them for a reader's benefit.

## Status

First pass, written during Phase 1 (Definition) alongside `THREAT_MODEL.md` and `ACTORS.md`. The amend loop, succession events, and multi-collection/staged release are the parts most likely to need real structure once Phase 2 (Specification) defines `capsule.yaml` and the metadata model.
