<!--
Worked example of ../../../templates/legal/custodian-agreement.md,
filled in for this fictional example capsule. See
../../../CUSTODIAN_AGREEMENT.md.

>>> ILLUSTRATIVE ONLY. Not a real agreement, not reviewed by an
attorney, not usable as-is. All names below are fictional. <<<
-->

# CUSTODIAN AGREEMENT

This Custodian Agreement ("Agreement") is made on 2026-08-30, between:

**Donor:** Jane Smith ("Donor")

and

**Custodian:** Springfield Historical Society ("Custodian"),

concerning the Capsule identified below, gifted to Custodian by Donor under a Deed of Gift dated 2026-08-30.

- **Capsule identifier:** urn:uuid:7c2e9f10-4b3d-4e2a-9a71-5d8f3c1b2e40
- **Date sealed:** 2026-08-30

## 1. Preservation Obligations

Custodian shall:

(a) Hold the Capsule and take reasonable measures to prevent its loss, damage, or unauthorized modification;

(b) Perform a fixity check (recomputing and verifying checksums against the Capsule's `manifest-sha256.txt` and `tagmanifest-sha256.txt`, or against the ciphertext as received where content is encrypted, per `INTEGRITY.md`) not less than once per calendar year, and additionally after any custody transfer and before and after any physical media migration;

(c) Upon a failed fixity check: quarantine the affected item without deleting or overwriting it, notify Donor (if reachable) and Riverside Public Library (the successor custodian named in this Capsule's `capsule.yaml`), attempt restoration from another custodian's independently verified replica, and log the outcome — including an unrecoverable outcome — as a `fixity_check` event in this Capsule's `preservation-log.yaml`.

## 2. Confidentiality

Custodian shall not attempt to decrypt, access, or otherwise view the contents of the `email` collection until its release trigger (`after_death_plus_years`, 50 years) is satisfied and confirmed per the process described in `ACCESS.md`, as recorded in the Access Restriction Agreement dated 2026-08-30. This Capsule's other collections (`person`, `provenance`, `photos`, `writing`) are public with an immediate release policy and are not subject to this restriction.

## 3. Trustee Attestation Coordination

This illustrative capsule's `encryption.status` is `none` — no trustees or key shares exist yet, so this Section is not currently operative. If the `email` collection is later encrypted per `ENCRYPTION.md`, Custodian shall, not less than once per calendar year (on the same schedule as Section 1(b)), request confirmation from each key-holding trustee named in the Capsule that the trustee can still produce their share, and shall log each such confirmation (or its absence) as a `trustee_attestation` event in this Capsule's `preservation-log.yaml`.

## 4. Unresponsiveness and Succession

If Custodian fails to complete a scheduled fixity check under Section 1(b):

(a) Riverside Public Library, or any key-holding trustee named in this Capsule, may attempt direct outreach to Custodian;

(b) If Custodian fails to complete the next consecutive scheduled check-in and has not responded to the outreach under (a), this shall constitute a documented unresponsiveness trigger under `CUSTODY.md`, entitling the succession chain to transfer this Capsule to Riverside Public Library as described there.

Custodian responding at any point before a trigger under (b) resets this Section.

## 5. Breach Documentation

A breach of the Access Restriction Agreement dated 2026-08-30 is documented, for purposes of the succession trigger described there, by a named party asserting the breach occurred together with a specific, corroborating description of what happened (including at minimum a date and a description of the action or omission constituting the breach) — logged as a `succession_transfer` event in this Capsule's `preservation-log.yaml`.

## 6. Cost and Funding

[TO BE DETERMINED — NEEDS LEGAL REVIEW; not resolved in this illustrative example.]

## 7. Term

This Agreement remains binding on Custodian, and on Riverside Public Library or any further successor who receives the Capsule, until every collection's recorded release policy has been fully satisfied or the Capsule ceases to exist.

## 8. Incorporation by Reference

This Agreement incorporates by reference this Capsule's own `capsule.yaml`, the Deed of Gift dated 2026-08-30, the Access Restriction Agreement dated 2026-08-30, and the Succession Instructions dated 2026-08-30 (see `legal/succession-instructions.md` in this bag).

## 9. Governing Law

[TO BE DETERMINED — NEEDS LEGAL REVIEW; not specified in this illustrative example.]

## Signatures

Donor: *(illustrative example — unsigned)* Date: 2026-08-30

Jane Smith

Custodian representative: *(illustrative example — unsigned)* Date: 2026-08-30

[Illustrative example — no real representative]
