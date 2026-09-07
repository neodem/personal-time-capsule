<!--
Personal Time Capsule — Custodian Agreement template (v0.1, DRAFT)
See ../../CUSTODIAN_AGREEMENT.md for the full rationale and open questions.

>>> NOT LEGAL ADVICE. NOT REVIEWED BY A LICENSED ATTORNEY. <<<
Do not sign, rely on, or use this document for an actual custodial
relationship until it has been reviewed by a qualified attorney
licensed in your jurisdiction.

Fill in every [BRACKETED] field. Fields marked [TO BE DETERMINED —
NEEDS LEGAL REVIEW] are placeholders this project's non-lawyer
drafters deliberately left open rather than guess at.
-->

# CUSTODIAN AGREEMENT

This Custodian Agreement ("Agreement") is made on [DATE], between:

**Donor:** [DONOR'S FULL LEGAL NAME] ("Donor")

and

**Custodian:** [CUSTODIAN INSTITUTION'S FULL LEGAL NAME] ("Custodian"),

concerning the Capsule identified below, gifted to Custodian by Donor under a Deed of Gift dated [DATE OF DEED OF GIFT].

- **Capsule identifier:** [capsule.yaml `capsule.id`]
- **Date sealed:** [sealing date]

## 1. Preservation Obligations

Custodian shall:

(a) Hold the Capsule and take reasonable measures to prevent its loss, damage, or unauthorized modification;

(b) Perform a fixity check (recomputing and verifying checksums against the Capsule's `manifest-sha256.txt` and `tagmanifest-sha256.txt`, or against the ciphertext as received where content is encrypted, per [`INTEGRITY.md`]) not less than once per calendar year, and additionally after any custody transfer and before and after any physical media migration;

(c) Upon a failed fixity check: quarantine the affected item without deleting or overwriting it, notify Donor (if reachable) and the trustees and successor custodians named in the Capsule's `capsule.yaml`, attempt restoration from another custodian's independently verified replica, and log the outcome — including an unrecoverable outcome — as a `fixity_check` event in the Capsule's `preservation-log.yaml`.

## 2. Confidentiality

Custodian shall not attempt to decrypt, access, or otherwise view the contents of any collection whose release trigger, as recorded in the Capsule's `capsule.yaml` and the Access Restriction Agreement dated [DATE], has not been satisfied and confirmed per the process described in [`ACCESS.md`].

## 3. Trustee Attestation Coordination

Custodian shall, not less than once per calendar year (on the same schedule as Section 1(b)), request confirmation from each key-holding trustee named in the Capsule that the trustee can still produce their share, and shall log each such confirmation (or its absence) as a `trustee_attestation` event in the Capsule's `preservation-log.yaml`.

## 4. Unresponsiveness and Succession

If Custodian fails to complete a scheduled fixity check or trustee-attestation confirmation under Sections 1(b) or 3:

(a) Any successor custodian or key-holding trustee named in the Capsule's `capsule.yaml` may attempt direct outreach to Custodian;

(b) If Custodian fails to complete the next consecutive scheduled check-in and has not responded to the outreach under (a), this shall constitute a documented unresponsiveness trigger under [`CUSTODY.md`], entitling the succession chain to proceed as described there.

Custodian responding at any point before a trigger under (b) resets this Section.

## 5. Breach Documentation

A breach of the Access Restriction Agreement dated [DATE] is documented, for purposes of the succession trigger described there, by a named party asserting the breach occurred together with a specific, corroborating description of what happened (including at minimum a date and a description of the action or omission constituting the breach) — logged as a `succession_transfer` event in the Capsule's `preservation-log.yaml`.

## 6. Cost and Funding

[TO BE DETERMINED — NEEDS LEGAL REVIEW: this project's drafters did not attempt to specify funding or cost-sharing terms. See `CUSTODIAN_AGREEMENT.md`'s "Open questions."]

## 7. Term

This Agreement remains binding on Custodian, and on any successor custodian who receives the Capsule under `CUSTODY.md`'s succession provisions, until every collection's recorded release policy has been fully satisfied or the Capsule ceases to exist.

## 8. Incorporation by Reference

This Agreement incorporates by reference the Capsule's own `capsule.yaml`, the Deed of Gift dated [DATE], and the Access Restriction Agreement dated [DATE]. In the event of a conflict: [TO BE DETERMINED — NEEDS LEGAL REVIEW].

## 9. Governing Law

This Agreement shall be governed by the laws of [TO BE DETERMINED — NEEDS LEGAL REVIEW].

## Signatures

Donor: _________________________________ Date: ______________

[DONOR'S PRINTED NAME]

Custodian representative: _________________________________ Date: ______________

[REPRESENTATIVE'S PRINTED NAME AND TITLE]
