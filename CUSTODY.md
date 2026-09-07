# Custody / Succession Model (draft)

Phase 2 (Specification) — [`ROADMAP.md`](./ROADMAP.md) item 15, the last Phase 2 item: what "successor activation" actually looks like procedurally, for both a collapsed custodian (`THREAT_MODEL.md` #3) and an unreachable key-holding trustee (`THREAT_MODEL.md` #7). No new file or schema — like `ACCESS.md`, this reuses `preservation-log.yaml`'s existing `succession_transfer` and `trustee_attestation` event types (`INTEGRITY.md`, item 11) rather than inventing new machinery.

## `capsule.yaml`'s `custody` section is a snapshot, not a live pointer

Same reasoning `METADATA.md` and `INTEGRITY.md` already established for `status.lifecycle_stage` and `events.yaml`: `custody.primary`/`custody.successors` is frozen at sealing and describes the creator's *original* plan, not who currently, actually holds the capsule. After any succession event, the capsule's real current custodian is whoever most recently appears as the receiving party in a `succession_transfer` entry in `preservation-log.yaml` — `capsule.yaml` itself is never rewritten to match, for the same reason it can't be: it's sealed.

## Custodian succession

### What triggers it

Either of two documented events is sufficient:

1. **Explicit notice** — the primary custodian formally notifies the succession chain of incapacity or closure. Unambiguous, immediate.
2. **A documented unresponsiveness timeout** — the primary fails to complete a defined number of consecutive scheduled check-ins (the same periodic confirmation `ACTORS.md`'s custodian role and `LIFECYCLE.md` stage 8 already call for). A real small institution is arguably more likely to just quietly run out of money than to issue a formal dissolution notice, so relying on explicit notice alone leaves a real gap.

The actual interval and missed-check-in threshold aren't fixed here — that's a number the Phase 3 custodian agreement (`ROADMAP.md` item 18) needs to actually commit to, with real institutions able to reason about what's operationally realistic. This document only establishes that *both* trigger types should be recognized, not what the specific numbers are.

### What activation and transfer actually involve

1. The successor obtains a verified replica of the capsule — ideally one it already holds, per the LOCKSS "lots of copies" practice (`STANDARDS.md`) a real deposit agreement should already be following; otherwise from another custodian in the chain, or from the creator/executor if still reachable.
2. The successor verifies `manifest-sha256.txt` and `tagmanifest-sha256.txt` against the copy it received (`THREAT_MODEL.md` #5) *before* treating the transfer as complete.
3. The successor logs a `succession_transfer` event in the transferred copy's own `preservation-log.yaml`: who the previous custodian was, which trigger applied, how the replica was verified, and when.
4. The successor becomes the de facto current custodian and takes over the ongoing responsibilities `INTEGRITY.md` already defines (fixity-check cadence, further succession monitoring) — no different from what the original primary was doing.

### If the whole named chain is exhausted

`THREAT_MODEL.md` #3 names this explicitly as a real risk, and this document doesn't solve it, because there isn't a mechanism that can guarantee a solution. The best available mitigation is the design already in place: a capsule that's self-describing enough for a genuinely new, previously undesignated institution to pick it up cold (`PRINCIPLES.md` #3, `THREAT_MODEL.md` #3's own mitigation direction), published and open enough (`PRINCIPLES.md` #10) that trustees or an executor motivated to find a new custodian aren't dependent on this project still existing to do it. That's a real mitigation, not a guarantee — `THREAT_MODEL.md`'s own "Explicit non-goals" section already says as much: no specific custodian's survival, including any in the original chain, is something this design can promise.

## Trustee succession

A fundamentally different problem from custodian succession. A custodian holds a *replica* — easy, in principle, to copy and hand to someone else. A trustee holds a *secret share* (`ENCRYPTION.md`), which can't simply be hand-copied to a new person without either reconstructing the underlying key (a real, if temporary, exposure) or using a more advanced scheme this project hasn't adopted (proactive secret sharing, share renewal without reconstruction) — out of scope for this project's stated scale.

**This can only happen while a live quorum still exists.** Shamir's Secret Sharing has no recovery path once the number of reachable trustees drops below the required threshold — that's a mathematical property of the scheme (`ENCRYPTION.md`), not a gap this document could close even if it tried. The only real defense is catching a trustee at risk *before* the quorum is actually lost, which is exactly what the recurring `trustee_attestation` sub-event (`LIFECYCLE.md` stage 8, `INTEGRITY.md`) exists for.

### When it happens

**Reactively — only when an actual `trustee_attestation` entry reports a real concern** (unreachable, resigning, reporting difficulty producing their share), not on a fixed rotation schedule. The alternative — re-splitting proactively on a calendar cadence regardless of any known problem — would catch a trustee who's quietly lost their share without realizing it, which a simple "are you still reachable" check can't. But every re-split requires momentarily reconstructing the full key, which is itself a real exposure (`THREAT_MODEL.md` #4, #9); recommending it on a fixed schedule means recommending more of those moments as routine practice, not free hygiene. This document recommends the reactive default and treats attestation actually working as the load-bearing assumption it is — an honest tradeoff, not a solved one.

### Procedure — and a distinction worth making precisely

1. A `trustee_attestation` entry documents a concern, while quorum is still intact.
2. The remaining trustees who still hold valid shares reconstruct the key.
3. **What happens next depends on *why* the trustee is leaving:**
   - **No-fault departure** (death, prolonged unreachability, voluntary resignation in good standing) — re-split the *same* key via SLIP-0039 with an adjusted roster. Cheaper: no payload re-encryption needed. The departed trustee's old share becomes irrelevant going forward (it reconstructs an instance of the same key, but there's no reason left for anyone to use it), though it isn't cryptographically *revoked* — see Open questions.
   - **For-cause removal** (suspected compromise, coercion, distrust) — re-splitting the *same* key isn't enough: the removed trustee's old share still reconstructs that same key regardless of whether they're excluded from the new roster. This case requires generating a **new** encryption key, re-encrypting the affected collection's payload with it, and only then splitting the new key via SLIP-0039 for the new roster. This is a `reseal`-class operation (`INTEGRITY.md`) — the payload's ciphertext changes — and should be logged as both a `reseal` entry (with the manifest checksums before/after) and a `succession_transfer` entry.
4. New shares are distributed to the (adjusted) roster; old shares should be destroyed by whoever held them, though this document can't enforce that any more than any other honesty-dependent step in this project can (see Open questions).
5. Log the whole operation — trigger, procedure used, new roster — as a `succession_transfer` entry in `preservation-log.yaml`.

### Worked example

```yaml
# preservation-log.yaml
- date: "2095-02-11"
  type: trustee_attestation
  agent: "Springfield Historical Society (custodian)"
  collection: email
  outcome: "Sam Smith (spouse, share 3 of 5) unreachable for 3 consecutive scheduled attestations"
  note: "Quorum (3-of-5) still intact — 4 of 5 trustees remain reachable"

- date: "2095-03-02"
  type: succession_transfer
  agent: "Priya Smith, Alex Smith, family estate attorney, Springfield Historical Society (4-of-5 quorum reconstructed key)"
  collection: email
  outcome: "No-fault departure (prolonged unreachability, not distrust) — same key re-split via SLIP-0039 with Sam Smith removed, no replacement added; quorum adjusted to 3-of-4"
  note: "Old share (index 3) not confirmed destroyed — see CUSTODY.md open questions"

- date: "2101-07-19"
  type: succession_transfer
  agent: "Riverside Public Library"
  outcome: "Primary custodian (Springfield Historical Society) missed 4 consecutive annual fixity-check confirmations; documented-unresponsiveness trigger per CUSTODY.md"
  note: "Replica verified against manifest-sha256.txt/tagmanifest-sha256.txt on receipt from the last custodian-held copy before transfer; Riverside Public Library is now the de facto primary custodian"
```

### `SLIP-0039`'s unused group structure

`ENCRYPTION.md` noted that SLIP-0039 supports more elaborate group-of-groups structures (e.g. "any 1 of 3 lawyers, OR 2 of 3 family members") that this project doesn't use — simple *k*-of-*n* has been sufficient for everything designed so far, including trustee succession above. Nothing here forecloses adopting it later if a real capsule's custody needs turn out to be more elaborate than one flat quorum.

## What's deliberately out of scope here

- **Specific timeout lengths and check-in intervals** for custodian unresponsiveness — a Phase 3 (`ROADMAP.md` item 18) number, not fixed here.
- **Legal authority** questions — whether a successor custodian can actually take custody without some formal legal process (e.g. if the original custodian's collapse involves bankruptcy proceedings with their own claims on assets). Phase 3 (`ROADMAP.md` items 16-19) territory entirely.
- **Automating any of this.** Detecting custodian unresponsiveness, orchestrating a trustee re-split — Phase 4 tooling, if it's ever built, not a specification question.
- **Pre-seal custody changes.** A creator changing their mind about the custody chain before sealing is just normal amend-loop editing of `capsule.yaml` (`LIFECYCLE.md`), not succession at all. This document only covers changes of hands *after* deposit.

## Open questions

- **Exhausted succession chains** (custodian or trustee) have no resolution here, only the mitigation-by-design described above. Real capsules and real institutional failures (Phase 5) are the only way to learn whether that's enough in practice.
- **Old trustee shares from a no-fault departure aren't cryptographically revoked**, only made irrelevant going forward — if the underlying key wasn't rotated, an old share technically still reconstructs it. This relies on the departed trustee actually destroying their old share, which this document can recommend but not enforce, similar to every other honesty-dependent step in this project's threat model.
- **Legal standing for a new, previously undesignated custodian** stepping in after an exhausted chain isn't addressed — Phase 3.

## Status

First pass, written during Phase 2 (Specification), addressing [`ROADMAP.md`](./ROADMAP.md) item 15 — the last Phase 2 item. Phase 2 (Specification) is now complete; Phase 3 (Legal) is next. Still open to challenge per [`CONTRIBUTING.md`](./CONTRIBUTING.md), especially the reactive-vs-proactive trustee rotation call and the for-cause/no-fault distinction, which haven't been reviewed by anyone with real institutional succession or cryptographic key-management experience.
