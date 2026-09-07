# Access / Release Model (draft)

Phase 2 (Specification) — [`ROADMAP.md`](./ROADMAP.md) item 14: what actually satisfies each `release_policy.type` in `capsule.yaml` — e.g. what confirms an `after_death` trigger. No new file or schema; this document gives real semantic content to vocabulary `capsule.yaml` (item 8), `preservation-log.yaml`'s `release_trigger`/`unlock`/`access` event types (`INTEGRITY.md`, item 11), and the trustee quorum (`ENCRYPTION.md`, item 13) already fixed.

## What already existed

`release_policy.type` has six values (`immediate`, `fixed_date`, `after_death`, `after_death_plus_years`, `never`, `custom`), and `LIFECYCLE.md` names a **release trigger** stage (9) — "the condition specified in the release policy is met" — distinct from **unlock** (stage 10), where trustees actually reconstruct the key. `FORMAT.md` flagged, since item 8, that nothing had yet defined what satisfies each type. This document does.

## `immediate`

Trivially satisfied — there's no trigger to confirm. Per `ENCRYPTION.md`, `immediate`-release content is never encrypted in the first place, so "release" already happened at deposit, or even before. No `release_trigger` event is needed for this type.

## `fixed_date`

Satisfied the instant the current date reaches the recorded `date`. Self-evident and confirmable by literally anyone with a calendar — no attestation, no evidence, no named confirming party required. A `release_trigger` event is still worth logging (`agent` can just be `"ptc CLI v0.1"` or similar — software checking a date needs no human judgment), purely so the capsule's own history stays complete, not because there's anything to dispute.

## `after_death` and `after_death_plus_years`

The genuinely hard case, and where this document does its real work.

**Confirmation reuses the existing trustee quorum (`ENCRYPTION.md`) rather than inventing a separate mechanism.** The executor/estate representative (`ACTORS.md`) is the expected first mover: they confirm the creator's death and notify the custodian and trustees, out-of-band (letter, phone, whatever the Phase 3 custodian agreement specifies). But **their word alone doesn't unlock anything.** Each trustee is expected to independently satisfy themselves that the release condition is genuinely met before contributing their SLIP-0039 share — the same way they'd be expected to notice a coerced or fraudulent request under any other circumstance. This means fooling the release trigger requires fooling the same quorum majority already required to reconstruct the key at all, not just one executor — a meaningfully stronger defense against `THREAT_MODEL.md` #6 (coercion) than treating one logged claim as sufficient. The cost: trustees need enough context from the executor to actually judge the claim themselves, not just permission to proceed.

**Recommended (not mandated) documentation standard:** a trustworthy `release_trigger` entry in `preservation-log.yaml` should name a confirming party *and* cite at least one independent corroborating source — a death certificate reference, a published obituary, a second family member's confirmation. This is guidance for what the log entry should contain, not a new schema field this spec enforces; nothing here can verify a death actually occurred. Formalizing this into an actual requirement (what a custodian is contractually obligated to demand before honoring a trigger) is Phase 3's job (`ROADMAP.md` item 18, custodian agreement) — this document sets the bar a good implementation should clear, not one it can guarantee.

**`after_death_plus_years` needs one more piece of information:** the confirmed *date* of death, not just the fact of it, since the real release date is `(confirmed death date) + years`. The `release_trigger` entry should record that date explicitly (in `note`, e.g. `"Death confirmed 2071-03-14; release date 2121-03-14"`) so the arithmetic has something concrete to anchor to.

### Worked example

```yaml
# preservation-log.yaml
- date: "2071-04-02"
  type: release_trigger
  agent: "John Doe, executor (ACTORS.md)"
  collection: email
  outcome: "Death confirmed"
  note: "Death occurred 2071-03-14. Corroborated by death certificate (county records, ref. #4471-2071) and published obituary (Springfield Gazette, 2071-03-18). release_policy: after_death_plus_years, 50 -> release date 2121-03-14."

- date: "2071-04-05"
  type: trustee_attestation
  agent: "Priya Smith (daughter), share 1 of 3"
  outcome: "Independently confirmed death via death certificate; will contribute share when quorum is assembled at the actual release date (2121-03-14)"

- date: "2121-03-14"
  type: unlock
  agent: "Priya Smith (daughter), Alex Smith (son) — 2 of 3 quorum reached"
  outcome: "Key reconstructed; email collection decrypted"
```

## `never`

No trigger ever fires — procedurally or cryptographically. Matches `ENCRYPTION.md`'s per-collection key design directly: a `never`-release collection's key is simply never reconstructed, because trustees never have occasion to contribute their shares toward it. There's nothing to confirm and nothing to log, by design.

If a creator later wants to reverse a sealed, deposited capsule's `never` policy — a genuine change of heart, decades on — that's an exceptional case this document doesn't resolve. It would intersect with `INTEGRITY.md`'s `reseal` mechanism and would likely require an entirely new key and trustee set for that collection (the old one was deliberately never assembled), which is real, unbuilt Phase 3/4 territory. Flagged in Open questions, not designed here.

## `custom`

Deliberately unformalized. The creator's free-text `description` *is* the policy — this document can't define generic semantics for a field that exists specifically because the built-in types didn't cover some creator's actual intent. What doesn't change: whichever trigger a `custom` policy names, satisfying it should still produce a `release_trigger` log entry, for the same reason every other type gets one — a capsule has to stay self-describing enough for a custodian who's never seen it before (`THREAT_MODEL.md` #3), and an undocumented `custom` release is exactly the kind of gap that breaks that.

## What's deliberately out of scope here

- **Legal recognition of death** — jurisdictional variation, disputed deaths, missing-persons declarations after years of absence, and everything else that makes "confirm someone died" a real legal question rather than a data-entry problem. Entirely Phase 3 (`ROADMAP.md` items 16-19) territory; this document only describes what the *spec-level record* should contain, not how to resolve a legal dispute about the underlying facts.
- **A formal appeals or dispute process** if trustees disagree among themselves about whether a trigger is genuinely met. Not designed here — see Open questions.
- **Changing a sealed release policy after deposit.** Touched on under `never`, above, but applies generally: this document assumes `release_policy` is fixed by sealing, per the normal amend-loop-then-seal flow `LIFECYCLE.md` already describes.
- **Automated trigger detection** (e.g., tooling that checks obituary databases or public records on a schedule). A Phase 4 tooling question, if it's ever built at all, not a specification question.

## Open questions

- **No minimum waiting or dispute period is defined between confirmation and unlock.** A real implementation might want a safety margin (e.g., 30 days) between a logged `release_trigger` and trustees actually being permitted to unlock, to give time for a premature or fraudulent claim to be caught. Not designed here — flagged for Phase 3/4 to consider.
- **No resolution process if trustees disagree** about whether a trigger is actually met. Realistically only surfaces with real capsules and real disputes (Phase 5); premature to design a resolution process without a real case to design it against.
- **Reversing a sealed `never` (or any other) policy** isn't designed — see the `never` section above.

## Status

First pass, written during Phase 2 (Specification), addressing [`ROADMAP.md`](./ROADMAP.md) item 14. This is the document on this project most dependent on Phase 3 (Legal) work that doesn't exist yet — expect it to need real revision once an actual estate lawyer looks at it, per [`CONTRIBUTING.md`](./CONTRIBUTING.md).
