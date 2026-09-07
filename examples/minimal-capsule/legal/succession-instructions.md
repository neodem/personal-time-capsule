<!--
Worked example of ../../../templates/legal/succession-instructions.md,
filled in for this fictional example capsule. See
../../../SUCCESSION_INSTRUCTIONS.md.

The Certificate of Succession and Trustee Acceptance templates aren't
filled in here: nothing has triggered succession for this capsule
(still freshly sealed, not yet deposited), and it has no trustees
(encryption.status: none) — see SUCCESSION_INSTRUCTIONS.md's own
worked example for what a filled-in Certificate would look like once
a succession event actually occurs.

>>> ILLUSTRATIVE ONLY. Not a real agreement, not reviewed by an
attorney, not usable as-is. All names below are fictional. <<<
-->

# SUCCESSION INSTRUCTIONS

This Succession Instructions agreement ("Instructions") is made on 2026-08-30, between:

**Donor:** Jane Smith ("Donor"),

**Primary Custodian:** Springfield Historical Society ("Primary Custodian"),

and each of the following, each a "Successor Custodian":

**Successor Custodian 1:** Riverside Public Library

concerning the Capsule identified below.

- **Capsule identifier:** urn:uuid:7c2e9f10-4b3d-4e2a-9a71-5d8f3c1b2e40
- **Date sealed:** 2026-08-30

## 1. Pre-Commitment

Riverside Public Library agrees that, upon a valid succession trigger under `CUSTODY.md` of the Personal Time Capsule specification (explicit notice, a documented unresponsiveness timeout, or a documented breach of the Access Restriction Agreement dated 2026-08-30) naming it as the next custodian, it shall automatically assume all obligations of the Custodian Agreement dated 2026-08-30 — without requirement of further negotiation, amendment, or re-execution — effective upon its receipt and verification of a replica of this Capsule per `CUSTODY.md`'s transfer procedure.

## 2. Order of Succession

There is currently one Successor Custodian (Riverside Public Library), matching this Capsule's `capsule.yaml` `custody.successors` list.

## 3. Adding a Successor Later

If Donor or the then-current Custodian wishes to name an additional or replacement Successor Custodian after this Capsule has been sealed, doing so requires a `reseal` event as described in `INTEGRITY.md` and execution of an amended or supplemental Succession Instructions signed by the new Successor Custodian.

## 4. Trustee Succession

This Capsule's `encryption.status` is `none` — it currently has no trustees, so this Section is not operative. If the `email` collection is later encrypted per `ENCRYPTION.md`, any new trustee added via a re-split under `CUSTODY.md` is encouraged, but not required, to execute a Trustee Acceptance.

## 5. Certificate of Succession

At the time an actual succession occurs, Riverside Public Library (or any further successor) shall execute a Certificate of Succession, retained as its own record — not added to this Capsule's sealed `legal/` directory — and referenced by date in the `succession_transfer` event logged in this Capsule's `preservation-log.yaml`. See `SUCCESSION_INSTRUCTIONS.md`'s own worked example of what that log entry would look like.

## 6. Incorporation by Reference

This Instructions document incorporates by reference this Capsule's own `capsule.yaml`, the Deed of Gift dated 2026-08-30, the Access Restriction Agreement dated 2026-08-30, and the Custodian Agreement dated 2026-08-30.

## 7. Governing Law

[TO BE DETERMINED — NEEDS LEGAL REVIEW; not specified in this illustrative example.]

## Signatures

Donor: *(illustrative example — unsigned)* Date: 2026-08-30

Jane Smith

Primary Custodian representative: *(illustrative example — unsigned)* Date: 2026-08-30

[Illustrative example — no real representative]

Successor Custodian 1 representative: *(illustrative example — unsigned)* Date: 2026-08-30

[Illustrative example — no real representative]
