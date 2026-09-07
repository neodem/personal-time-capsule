<!--
Personal Time Capsule — Succession Instructions template (v0.1, DRAFT)
See ../../SUCCESSION_INSTRUCTIONS.md for the full rationale and open questions.

>>> NOT LEGAL ADVICE. NOT REVIEWED BY A LICENSED ATTORNEY. <<<
Do not sign, rely on, or use this document until it has been reviewed
by a qualified attorney licensed in your jurisdiction.

Fill in every [BRACKETED] field. Fields marked [TO BE DETERMINED —
NEEDS LEGAL REVIEW] are placeholders this project's non-lawyer
drafters deliberately left open rather than guess at. Add a numbered
Successor section (Section 3) for each successor custodian named in
the Capsule's custody chain.
-->

# SUCCESSION INSTRUCTIONS

This Succession Instructions agreement ("Instructions") is made on [DATE], between:

**Donor:** [DONOR'S FULL LEGAL NAME] ("Donor"),

**Primary Custodian:** [PRIMARY CUSTODIAN'S FULL LEGAL NAME] ("Primary Custodian"),

and each of the following, each a "Successor Custodian":

**Successor Custodian 1:** [SUCCESSOR 1'S FULL LEGAL NAME]

**Successor Custodian 2:** [SUCCESSOR 2'S FULL LEGAL NAME, if any]

[Add additional numbered successors as needed, matching the Capsule's `capsule.yaml` `custody.successors` list.]

concerning the Capsule identified below.

- **Capsule identifier:** [capsule.yaml `capsule.id`]
- **Date sealed:** [sealing date]

## 1. Pre-Commitment

Each Successor Custodian agrees that, upon a valid succession trigger under [`CUSTODY.md`] of the Personal Time Capsule specification (explicit notice, a documented unresponsiveness timeout, or a documented breach of the Access Restriction Agreement dated [DATE]) naming that Successor Custodian as the next custodian in the chain, that Successor Custodian shall automatically assume all obligations of the Custodian Agreement dated [DATE] — without requirement of further negotiation, amendment, or re-execution — effective upon its receipt and verification of a replica of the Capsule per `CUSTODY.md`'s transfer procedure.

## 2. Order of Succession

Successor Custodians shall assume the role of Custodian in the order listed above, matching the order recorded in the Capsule's `capsule.yaml` `custody.successors` list, unless a Successor Custodian is itself unable to serve at the time called upon, in which case the next Successor Custodian in order shall be called upon instead.

## 3. Adding a Successor Later

If Donor or the then-current Custodian wishes to name an additional or replacement Successor Custodian after this Capsule has been sealed, doing so requires: (a) a `reseal` event as described in [`INTEGRITY.md`], and (b) execution of an amended or supplemental Succession Instructions signed by the new Successor Custodian, prior to that Successor Custodian being treated as bound under Section 1.

## 4. Trustee Succession

Where the Capsule's `encryption.key_custody.trustees` includes trustees, a new trustee added via a re-split under `CUSTODY.md` is encouraged, but not required, to execute the accompanying Trustee Acceptance (see `templates/legal/trustee-acceptance.md`) acknowledging receipt of a share and their role under [`ACCESS.md`] and [`ENCRYPTION.md`].

## 5. Certificate of Succession

At the time a succession described in Section 1 actually occurs, the assuming Successor Custodian shall execute a Certificate of Succession (see `templates/legal/certificate-of-succession.md`), retained by the assuming Successor Custodian as its own record — not added to the Capsule's sealed `legal/` directory — and referenced by date in the `succession_transfer` event logged in the Capsule's `preservation-log.yaml`.

## 6. Incorporation by Reference

This Instructions document incorporates by reference the Capsule's own `capsule.yaml`, the Deed of Gift dated [DATE], the Access Restriction Agreement dated [DATE], and the Custodian Agreement dated [DATE].

## 7. Governing Law

This Instructions document shall be governed by the laws of [TO BE DETERMINED — NEEDS LEGAL REVIEW].

## Signatures

Donor: _________________________________ Date: ______________

[DONOR'S PRINTED NAME]

Primary Custodian representative: _________________________________ Date: ______________

[REPRESENTATIVE'S PRINTED NAME AND TITLE]

Successor Custodian 1 representative: _________________________________ Date: ______________

[REPRESENTATIVE'S PRINTED NAME AND TITLE]

[Add a signature block for each additional Successor Custodian.]
