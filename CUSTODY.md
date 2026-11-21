# Custody / Succession Model (draft)

Phase 2 (Specification) — [`ROADMAP.md`](./ROADMAP.md) item 15, the last Phase 2 item: what "successor activation" actually looks like procedurally, for both a collapsed custodian (`THREAT_MODEL.md` #3) and an unreachable key-holding trustee (`THREAT_MODEL.md` #7). No new file or schema — like `ACCESS.md`, this reuses `preservation-log.yaml`'s existing `succession_transfer` and `trustee_attestation` event types (`INTEGRITY.md`, item 11) rather than inventing new machinery.

## `capsule.yaml`'s `custody` section is a snapshot, not a live pointer

Same reasoning `METADATA.md` and `INTEGRITY.md` already established for `status.lifecycle_stage` and `events.yaml`: `custody.primary`/`custody.successors` is frozen at sealing and describes the creator's *original* plan, not who currently, actually holds the capsule. After any succession event, the capsule's real current custodian is whoever most recently appears as the receiving party in a `succession_transfer` entry in `preservation-log.yaml` — `capsule.yaml` itself is never rewritten to match, for the same reason it can't be: it's sealed.

## Initial deposit: creator to primary custodian

A different event from everything below it — this happens once, not repeatedly, and there's no prior custodian to verify against, only the creator's own sealed copy. `LIFECYCLE.md` stage 7 names this stage but doesn't work out the mechanics; this is where that actually happens.

**Physical media, not network transfer, for anything beyond a modest capsule.** `FORMAT.md`'s "Scale" section sizes a genuinely complete personal archive at low hundreds of GB to low single-digit TB, dominated by photos and video. At that size, uploading over a typical residential connection (upload speed is usually far slower than download) can take days of continuous transfer — shipping a hard drive is often both faster and more reliable. This isn't a workaround; it's standard practice in the archives field already, and several institutions explicitly accept physical media as a donation format for exactly this reason.

The sequence:

1. **Seal, then encrypt if applicable** (`LIFECYCLE.md` stages 5-6) before anything leaves the creator's hands.
2. **Copy the sealed capsule onto the drive as a plain directory.** Skip `.tar.gz` compression for a media-heavy capsule — gzip gains essentially nothing on already-compressed JPEGs, video, or `.enc` ciphertext, and just costs time. Compression is only worth it for a genuinely text/email-dominated capsule.
3. **Run `ptc verify` before the drive ships.** Confirms the copy is good before anything can go wrong in transit — bit rot on the drive, physical damage, a dropped file during copying.
4. **Key material travels separately from the data, if encryption is in use.** Never ship the same drive, or the same package, carrying both ciphertext and a trustee's key share — that defeats the point of splitting the key at all. Keys/shares go to trustees through whatever channel was already agreed, independent of how the data itself travels.
5. **The custodian runs `ptc verify` again on arrival**, independent of how the copy physically got there. This is the actual point of the manifest: it lets the custodian confirm nothing was corrupted or tampered with in transit, without having to trust the courier, the drive, or the creator's own claim that nothing changed.
6. **Only after verification passes does custody formally transfer** — logged as a `deposit` entry in `preservation-log.yaml` (`INTEGRITY.md`; `ptc deposit` already does this), not a `succession_transfer` — that type is reserved for a *later* change of hands, covered below. Only once this is logged does the custodian's own internal storage strategy (their cloud, tape, on-prem replication) become their business, not the creator's.

**A single drive shipped once is a real, if temporary, single point of failure** — lost in transit, damaged, or destroyed before the custodian has made their own copies. Keeping the creator's own copy until verification is confirmed is cheap insurance, and doesn't substitute for the custodian actually executing LOCKSS-style replication (`THREAT_MODEL.md` #1) promptly once they do have a verified copy.

## Custodian succession

### What triggers it

Any of three documented events is sufficient:

1. **Explicit notice** — the primary custodian formally notifies the succession chain of incapacity or closure. Unambiguous, immediate.
2. **A documented unresponsiveness timeout** — the primary fails to complete a defined number of consecutive scheduled check-ins (the same periodic confirmation `ACTORS.md`'s custodian role and `LIFECYCLE.md` stage 8 already call for). A real small institution is arguably more likely to just quietly run out of money than to issue a formal dissolution notice, so relying on explicit notice alone leaves a real gap.
3. **A documented breach of the Access Restriction Agreement** (`ROADMAP.md` item 17) — the custodian releases content early, or refuses to release it once its recorded trigger has actually fired. Added once `ACCESS_RESTRICTION_AGREEMENT.md` was written: a custodian that won't honor the access terms it agreed to is exactly as unfit to continue holding the capsule as one that's gone bankrupt, and treating a breach as a succession trigger means the remedy doesn't depend on anyone still being around decades later to file a lawsuit — whoever currently has standing (the executor while the estate is open, then successor custodians or trustees themselves afterward) can simply invoke it.

The actual interval and missed-check-in threshold for trigger 2 are committed in [`CUSTODIAN_AGREEMENT.md`](./CUSTODIAN_AGREEMENT.md) (`ROADMAP.md` item 18): one missed scheduled check-in triggers direct outreach, and a second consecutive miss with no response to that outreach triggers succession. What counts as sufficient documentation for trigger 3 is `ACCESS_RESTRICTION_AGREEMENT.md` (item 17) territory. This document only establishes that all three trigger types should be recognized — the specific numbers live in the agreements that actually bind a real custodian.

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
- **How many physical copies a creator should make before initial deposit, and whether each custodian in the chain should receive its own shipment versus relying on the primary to replicate onward**, isn't addressed. Neither is who bears the cost or liability if a shipped drive is lost or damaged in transit.

## Status

First pass, written during Phase 2 (Specification), addressing [`ROADMAP.md`](./ROADMAP.md) item 15 — the last Phase 2 item. Amended during Phase 3 to add breach of the Access Restriction Agreement as a third custodian-succession trigger (`ACCESS_RESTRICTION_AGREEMENT.md`, item 17). Amended again after a Phase 5 conversation to add the "Initial deposit" section above — the creator-to-primary-custodian transfer this document had never actually worked out, despite covering every later succession transfer in detail. Still open to challenge per [`CONTRIBUTING.md`](./CONTRIBUTING.md), especially the reactive-vs-proactive trustee rotation call and the for-cause/no-fault distinction, which haven't been reviewed by anyone with real institutional succession or cryptographic key-management experience.
