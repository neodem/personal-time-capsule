<!--
Personal Time Capsule — Access Restriction Agreement template (v0.1, DRAFT)
See ../../ACCESS_RESTRICTION_AGREEMENT.md for the full rationale and open questions.

>>> NOT LEGAL ADVICE. NOT REVIEWED BY A LICENSED ATTORNEY. <<<
Do not sign, rely on, or use this document for an actual access
restriction until it has been reviewed by a qualified attorney
licensed in your jurisdiction.

Fill in every [BRACKETED] field. Fields marked [TO BE DETERMINED —
NEEDS LEGAL REVIEW] are placeholders this project's non-lawyer
drafters deliberately left open rather than guess at.
-->

# ACCESS RESTRICTION AGREEMENT

This Access Restriction Agreement ("Agreement") is made on [DATE], between:

**Donor:** [DONOR'S FULL LEGAL NAME] ("Donor")

and

**Custodian:** [CUSTODIAN INSTITUTION'S FULL LEGAL NAME] ("Custodian"),

concerning the Capsule identified below, gifted to Custodian by Donor under a Deed of Gift dated [DATE OF DEED OF GIFT, if executed].

- **Capsule identifier:** [capsule.yaml `capsule.id`]
- **Date sealed:** [sealing date]

## 1. Release Schedule

Custodian shall not permit access to any collection listed below before its recorded trigger is satisfied, and shall grant access to it promptly once that trigger is satisfied and confirmed, per the technical process described in [`ACCESS.md`] and [`ENCRYPTION.md`] of the Personal Time Capsule specification this Capsule conforms to.

| Collection | Privacy Zone | Release Policy | Notes |
|---|---|---|---|
| [collection name, e.g. "photos"] | [public / restricted / never-release] | [immediate / fixed_date: DATE / after_death / after_death_plus_years: N / never / custom] | [any collection-specific notes] |
| [collection name] | [zone] | [policy] | |

Individual items within a collection may carry their own, stricter override, recorded in the Capsule's own `privacy-overrides.yaml` — such overrides remain in force even after the collection's general trigger is satisfied. Custodian shall consult the Capsule's own recorded terms, not only this table, before granting access to any specific item.

## 2. Confirmation of Death-Triggered Releases

For any collection whose Release Policy is `after_death` or `after_death_plus_years`, Custodian shall not treat the trigger as satisfied absent:

(a) A named party confirming the Donor's death; and

(b) At least one independent corroborating source (e.g. a death certificate reference, a published obituary, confirmation from a second family member),

both documented in the Capsule's `preservation-log.yaml` as a `release_trigger` event, per [`ACCESS.md`]'s recommended evidentiary standard. Custodian's obligation to release is conditioned on this documentation existing, not merely on a single party's assertion.

## 3. Third-Party Privacy

Custodian acknowledges that release of a collection under Section 1 does not override any stricter, item-level restriction recorded in the Capsule's `privacy-overrides.yaml`, and that such restrictions exist specifically to protect persons other than Donor who are referenced in the Capsule's content and have not separately consented to its creation.

## 4. Breach

A breach of this Agreement occurs if Custodian permits access to any collection before its recorded trigger is satisfied, or fails to grant access within a reasonable time after the trigger is satisfied and confirmed per Section 2 (where applicable).

Upon a documented breach:

(a) The breach shall be logged in the Capsule's `preservation-log.yaml`;

(b) The breach shall be treated as a custodian-succession trigger under the Capsule's `capsule.yaml` `custody` provisions and the Personal Time Capsule specification's [`CUSTODY.md`], entitling whoever then holds standing — Donor's executor, if the estate remains open, or otherwise any successor custodian or key-holding trustee named in the Capsule — to initiate transfer of the Capsule to the next custodian in the succession chain.

[TO BE DETERMINED — NEEDS LEGAL REVIEW: whether any additional remedy (damages, injunctive relief, a specifically named enforcement party) should exist alongside the succession-chain remedy above.]

## 5. Term

This Agreement remains binding on Custodian, and on any successor custodian who receives the Capsule under Section 4 or under the Capsule's succession chain, until every collection's recorded release policy has been fully satisfied or the Capsule ceases to exist.

## 6. Incorporation by Reference

This Agreement incorporates by reference the Capsule's own `capsule.yaml`, the Deed of Gift referenced above, and the Custodian Agreement, if executed separately: [reference/attach]. In the event of a conflict: [TO BE DETERMINED — NEEDS LEGAL REVIEW].

## 7. Governing Law

This Agreement shall be governed by the laws of [TO BE DETERMINED — NEEDS LEGAL REVIEW].

## Signatures

Donor: _________________________________ Date: ______________

[DONOR'S PRINTED NAME]

Custodian representative: _________________________________ Date: ______________

[REPRESENTATIVE'S PRINTED NAME AND TITLE]
