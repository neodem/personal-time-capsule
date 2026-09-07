<!--
Personal Time Capsule — Deed of Gift template (v0.1, DRAFT)
See ../../DEED_OF_GIFT.md for the full rationale and open questions.

>>> NOT LEGAL ADVICE. NOT REVIEWED BY A LICENSED ATTORNEY. <<<
Do not sign, rely on, or use this document for an actual gift, estate
plan, or transfer of property until it has been reviewed by a
qualified attorney licensed in your jurisdiction.

Fill in every [BRACKETED] field. Fields marked [TO BE DETERMINED —
NEEDS LEGAL REVIEW] are placeholders this project's non-lawyer
drafters deliberately left open rather than guess at.
-->

# DEED OF GIFT

This Deed of Gift ("Deed") is made on [DATE], between:

**Donor:** [DONOR'S FULL LEGAL NAME], of [DONOR'S ADDRESS] ("Donor")

and

**Recipient:** [CUSTODIAN INSTITUTION'S FULL LEGAL NAME], of [INSTITUTION'S ADDRESS] ("Custodian").

## 1. The Gift

Donor hereby gives, transfers, and delivers to Custodian, as an unconditional gift, custody of the digital archive described below (the "Capsule"), subject to the terms of this Deed.

- **Capsule identifier:** [capsule.yaml `capsule.id`]
- **Date sealed:** [sealing date, per capsule.yaml `capsule.created` / the relevant `data/provenance/events.yaml` entry]
- **Brief description:** [a short plain-language description, e.g. from capsule.yaml's `purpose.description`]
- **Manifest reference:** the SHA-256 checksums recorded in the Capsule's own `manifest-sha256.txt` and `tagmanifest-sha256.txt` as of the sealing date above.

This Deed applies only to the Capsule as it existed at the time of sealing referenced above. It does not extend to any other capsule Donor may create, or to any later, separately sealed version.

## 2. What Is Transferred

Custodian receives:

(a) Physical and digital custody of the Capsule;

(b) The right and obligation to preserve, verify, and replicate the Capsule in accordance with the technical practices the Capsule itself records and any separate Custodian Agreement executed between the parties;

(c) A non-exclusive license to reproduce, migrate the format of, and provide access to the Capsule's contents, strictly in accordance with the release policy recorded in the Capsule's own `capsule.yaml` and any Access Restriction Agreement executed alongside this Deed.

This Deed does not transfer copyright — see Section 3.

## 3. Copyright and Ownership

Donor retains all copyright and other intellectual property rights in the content of the Capsule. This Deed transfers custody of the Capsule and the license described in Section 2 only. Upon Donor's death, copyright passes to Donor's heirs, successors, or assigns under applicable law, subject to the license granted in Section 2, which survives Donor's death and continues to bind Donor's heirs, successors, and assigns.

## 4. Third-Party Material

Donor acknowledges that the Capsule may contain material referencing, depicting, or authored in part by persons other than Donor ("Third Parties") who have not separately consented to the Capsule's creation. Donor represents that, to the best of Donor's knowledge, Donor has the lawful right to include such material as part of a personal archive of this kind. **Donor makes no representation that every Third Party referenced in the Capsule has consented to its creation or inclusion.** The parties acknowledge that privacy protection for Third Parties is instead addressed through the privacy zones, redaction, and access restrictions recorded in the Capsule itself and in any Access Restriction Agreement — not through a warranty in this Deed.

## 5. Incorporation by Reference

This Deed incorporates by reference:

- The Capsule's own `capsule.yaml`, including its recorded `access` (privacy zone and release policy) terms;
- The Access Restriction Agreement, if executed separately: [reference/attach];
- The Custodian Agreement, if executed separately: [reference/attach].

In the event of a conflict between this Deed and any of the above: [TO BE DETERMINED — NEEDS LEGAL REVIEW].

## 6. No Warranty of Perpetual Institutional Existence

Donor acknowledges that Custodian's continued existence is not guaranteed. The Capsule's succession chain, recorded in `capsule.yaml`'s `custody` section, is the intended remedy if Custodian ceases to be able to fulfill its obligations under this Deed.

## 7. Governing Law

This Deed shall be governed by the laws of [TO BE DETERMINED — NEEDS LEGAL REVIEW].

## Signatures

Donor: _________________________________ Date: ______________

[DONOR'S PRINTED NAME]

Custodian representative: _________________________________ Date: ______________

[REPRESENTATIVE'S PRINTED NAME AND TITLE]
