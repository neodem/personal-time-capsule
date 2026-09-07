<!--
Worked example of ../../../templates/legal/access-restriction-agreement.md,
filled in for this fictional example capsule. See
../../../ACCESS_RESTRICTION_AGREEMENT.md.

>>> ILLUSTRATIVE ONLY. Not a real agreement, not reviewed by an
attorney, not usable as-is. All names below are fictional. <<<
-->

# ACCESS RESTRICTION AGREEMENT

This Access Restriction Agreement ("Agreement") is made on 2026-08-30, between:

**Donor:** Jane Smith ("Donor")

and

**Custodian:** Springfield Historical Society ("Custodian"),

concerning the Capsule identified below, gifted to Custodian by Donor under a Deed of Gift dated 2026-08-30.

- **Capsule identifier:** urn:uuid:7c2e9f10-4b3d-4e2a-9a71-5d8f3c1b2e40
- **Date sealed:** 2026-08-30

## 1. Release Schedule

Custodian shall not permit access to any collection listed below before its recorded trigger is satisfied, and shall grant access to it promptly once that trigger is satisfied and confirmed, per the technical process described in `ACCESS.md` and `ENCRYPTION.md` of the Personal Time Capsule specification this Capsule conforms to.

| Collection | Privacy Zone | Release Policy | Notes |
|---|---|---|---|
| person | public | immediate | Human-context layer (biography, timeline, people, places) |
| provenance | public | immediate | Event/provenance log |
| photos | public | immediate | |
| email | restricted | after_death_plus_years: 50 | Release date = 50 years after Donor's confirmed date of death |
| writing | public | immediate | |

Individual items within a collection may carry their own, stricter override, recorded in this Capsule's own `privacy-overrides.yaml` — currently empty for this illustrative capsule, meaning no item-level exceptions exist beyond the table above.

## 2. Confirmation of Death-Triggered Releases

For the `email` collection (Release Policy: `after_death_plus_years`), Custodian shall not treat the trigger as satisfied absent:

(a) A named party confirming Donor's death; and

(b) At least one independent corroborating source (e.g. a death certificate reference, a published obituary, confirmation from a second family member),

both documented in this Capsule's `preservation-log.yaml` as a `release_trigger` event, per `ACCESS.md`'s recommended evidentiary standard.

## 3. Third-Party Privacy

Custodian acknowledges that release of the `email` collection under Section 1 does not override any stricter, item-level restriction that may later be recorded in this Capsule's `privacy-overrides.yaml` (empty as of sealing), and that such restrictions exist specifically to protect persons other than Donor referenced in the Capsule's content — for example, the correspondent named in `data/email/2019-07-14-family-reunion.eml`.

## 4. Breach

A breach of this Agreement occurs if Custodian permits access to any collection before its recorded trigger is satisfied, or fails to grant access within a reasonable time after the trigger is satisfied and confirmed per Section 2 (where applicable).

Upon a documented breach:

(a) The breach shall be logged in this Capsule's `preservation-log.yaml`;

(b) The breach shall be treated as a custodian-succession trigger under this Capsule's `capsule.yaml` `custody` provisions — Springfield Historical Society as primary custodian, Riverside Public Library as successor — and the Personal Time Capsule specification's `CUSTODY.md`, entitling whoever then holds standing to initiate transfer of the Capsule to Riverside Public Library.

[TO BE DETERMINED — NEEDS LEGAL REVIEW; not resolved in this illustrative example.]

## 5. Term

This Agreement remains binding on Custodian, and on Riverside Public Library or any further successor who receives the Capsule, until every collection's recorded release policy has been fully satisfied or the Capsule ceases to exist.

## 6. Incorporation by Reference

This Agreement incorporates by reference this Capsule's own `capsule.yaml`, the Deed of Gift dated 2026-08-30, and the Custodian Agreement dated 2026-08-30 (see `legal/custodian-agreement.md` in this bag).

## 7. Governing Law

[TO BE DETERMINED — NEEDS LEGAL REVIEW; not specified in this illustrative example.]

## Signatures

Donor: *(illustrative example — unsigned)* Date: 2026-08-30

Jane Smith

Custodian representative: *(illustrative example — unsigned)* Date: 2026-08-30

[Illustrative example — no real representative]
