# Succession Instructions (draft)

Phase 3 (Legal) — [`ROADMAP.md`](./ROADMAP.md) item 19.

> ⚠️ **This document, and the templates it describes, are drafts prepared by non-lawyers as part of an open-source project. Neither has been reviewed or approved by any licensed attorney. Do not sign, rely on, or treat any version of this material as legally effective. This is explicitly a starting point for professional legal review — not a substitute for it.** See [`CONTRIBUTING.md`](./CONTRIBUTING.md).

Two templates come out of this item, because they're executed at different times: [`templates/legal/succession-instructions.md`](./templates/legal/succession-instructions.md) (signed once, at deposit) and [`templates/legal/certificate-of-succession.md`](./templates/legal/certificate-of-succession.md) (executed later, only if succession actually happens). A filled-in worked example of the former, for the fictional Jane Smith capsule, lives at [`examples/minimal-capsule/legal/succession-instructions.md`](./examples/minimal-capsule/legal/succession-instructions.md).

## The gap this closes

`CUSTODY.md` (item 15) already specifies the succession *mechanics* in full — what triggers it (explicit notice, a documented unresponsiveness timeout, or a documented breach), and what activation and transfer actually involve (obtain a verified replica, check it against the manifest, log the event, take over). `CUSTODIAN_AGREEMENT.md` (item 18) already binds the *primary* custodian to real, enforceable obligations.

But nothing yet legally binds a *successor*. `capsule.yaml`'s `custody.successors` list just names them — per `ACTORS.md`, a successor custodian is "inert until activated," which is true procedurally but leaves an actual legal gap: at the moment a real succession is needed, decades on, a "successor" who was only ever *named*, never asked to actually agree to anything, could simply decline. `THREAT_MODEL.md` #3 already worries about a successor that's since collapsed; this document worries about the closely related case of a successor that never really committed in the first place.

## When successors commit: all at once, at deposit

This was the real judgment call here — put to the user rather than decided silently, since the alternative (each successor signing separately, possibly incrementally, possibly not until they're actually called upon) is a real, more flexible option with a real cost. **Chosen: every successor named in the chain signs the Succession Instructions at the same time as the Deed of Gift and Custodian Agreement**, alongside Donor and the primary Custodian. Each successor's obligation — to assume the full Custodian Agreement automatically, without renegotiation, if a valid `CUSTODY.md` trigger ever names them next — is locked in from day one. The cost, acknowledged: this requires every successor to be identified, reachable, and willing to sign before the capsule is ever deposited, which is a real, sometimes impractical constraint on a creator's choice of distant or not-yet-contacted fallback institutions.

**Adding a successor later** (a creator names an additional or replacement fallback after the capsule is already sealed) requires the same exceptional path `INTEGRITY.md`'s `reseal` mechanism already exists for — this isn't a new problem, just this document applying an existing one: the new successor executes an amended or supplemental Succession Instructions at the time of that `reseal` event, not left implicit.

## Trustee succession: recommended, not required, to formalize the same way

`CUSTODY.md`'s trustee succession (re-splitting the key with an adjusted roster) has an analogous gap — nothing obligates a *new* trustee joining via a re-split to actually accept the role. This document doesn't require the same all-at-once, notarized-feeling formality for trustees that it does for institutional custodians, because trustees are frequently individuals — family, friends — for whom that would feel like the wrong register entirely (per `ACTORS.md`, a trustee "may be a person... an institution, or the custodian itself"). Instead, `templates/legal/trustee-acceptance.md` offers a short, optional acknowledgment a new trustee can sign when they receive a share, confirming they understand their role under `ACCESS.md`/`ENCRYPTION.md` — recommended especially for institutional trustees, genuinely optional for a family member for whom handing them a printed card of SLIP-0039 words already carries plenty of weight on its own.

## The Certificate of Succession: executed later, and deliberately not part of the sealed bag

A real succession event might happen decades after sealing. Whatever documents it — confirming who received the capsule, under which trigger, with what verification — can't go in the capsule's own `legal/` directory, because that directory is checksummed in `tagmanifest-sha256.txt` and frozen at sealing (`DEED_OF_GIFT.md` established this convention; a document that doesn't exist yet at sealing time obviously can't be part of a checksum computed then).

The resolution follows the same pattern `ENCRYPTION.md` already uses for trustee shares: **the actual signed Certificate is held by the new custodian as its own institutional record, not embedded in the bag**, and the succession event itself is recorded where post-seal events already belong — a `succession_transfer` entry in `preservation-log.yaml` (`INTEGRITY.md`, item 11), referencing the Certificate by date rather than containing its text. No new file-location mechanism was needed here; this is a direct application of machinery items 11 and 13 already built.

### Worked example

```yaml
# preservation-log.yaml, at the moment of an actual succession event
- date: "2101-07-19"
  type: succession_transfer
  agent: "Riverside Public Library"
  outcome: "Documented unresponsiveness trigger — Springfield Historical Society missed 2 consecutive scheduled check-ins with no response to outreach, per CUSTODIAN_AGREEMENT.md"
  note: "Certificate of Succession executed 2101-07-19, held by Riverside Public Library per its pre-commitment under Succession Instructions dated 2026-08-30. Replica verified against manifest-sha256.txt/tagmanifest-sha256.txt on receipt."
```

## What's deliberately out of scope here

- **Jurisdiction-specific law and tax consequences** — same caveats as the other Phase 3 documents.
- **Legal authority to actually compel a reluctant successor** if, despite having pre-signed, one refuses when actually called upon. This document creates the commitment; enforcing it if broken is `ACCESS_RESTRICTION_AGREEMENT.md`'s breach/succession-chain logic, and beyond that, general contract law this project doesn't attempt to specify.
- **What happens if every successor in the chain has, in fact, also collapsed by the time succession is needed** — `CUSTODY.md`'s already-flagged, unsolved "exhausted chain" limitation. Pre-committing successors makes each individual link stronger; it doesn't lengthen the chain.

## Open questions

- **Whether requiring all successors to sign at deposit is actually realistic** for a creator whose fallback choice is a distant institution with no existing relationship — the option not chosen (incremental, separate acceptances) exists for exactly this reason and may turn out to be necessary in practice for some capsules, even though the all-at-once approach is the stronger default.
- **Whether trustee acceptance should be more formal than "recommended, optional"** for cases where the trustee genuinely is a stranger-ish institution rather than a known individual — this document treats "trustee" as usually informal by default, which may not fit every real capsule's actual trustee roster.

## Status

First pass, written during Phase 3 (Legal), addressing [`ROADMAP.md`](./ROADMAP.md) item 19. Not usable — drafted by non-lawyers, pending review by a licensed attorney. See [`CONTRIBUTING.md`](./CONTRIBUTING.md).
