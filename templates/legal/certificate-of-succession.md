<!--
Personal Time Capsule — Certificate of Succession template (v0.1, DRAFT)
See ../../SUCCESSION_INSTRUCTIONS.md for the full rationale.

>>> NOT LEGAL ADVICE. NOT REVIEWED BY A LICENSED ATTORNEY. <<<

IMPORTANT: Unlike the other Phase 3 templates, this one is NOT executed
at deposit — it's executed later, only if and when an actual succession
event occurs. It is NOT added to the Capsule's sealed legal/ directory
(which is frozen at sealing and can't accept new files). Instead, retain
the executed Certificate as the assuming Custodian's own institutional
record, and reference it by date in the succession_transfer event
logged in the Capsule's preservation-log.yaml. See SUCCESSION_INSTRUCTIONS.md.

Fill in every [BRACKETED] field.
-->

# CERTIFICATE OF SUCCESSION

This Certificate of Succession ("Certificate") is made on [DATE OF ACTUAL SUCCESSION EVENT], by:

**New Custodian:** [NEW CUSTODIAN'S FULL LEGAL NAME], a Successor Custodian named in the Succession Instructions dated [ORIGINAL SUCCESSION INSTRUCTIONS DATE],

concerning the Capsule identified below.

- **Capsule identifier:** [capsule.yaml `capsule.id`]
- **Prior Custodian:** [PRIOR CUSTODIAN'S NAME]

## 1. Trigger

Succession was triggered by: [explicit notice / a documented unresponsiveness timeout / a documented breach of the Access Restriction Agreement], per [`CUSTODY.md`] of the Personal Time Capsule specification, as documented in the Capsule's `preservation-log.yaml` at [reference or date of the relevant log entries].

## 2. Verification

New Custodian confirms it has obtained a replica of the Capsule and verified that replica's `manifest-sha256.txt` and `tagmanifest-sha256.txt` against its own independently computed checksums, per `CUSTODY.md`'s transfer procedure, with the following result: [pass / discrepancies noted: describe].

## 3. Assumption of Obligations

Effective as of the date of this Certificate, New Custodian assumes all obligations of the Custodian Agreement dated [ORIGINAL CUSTODIAN AGREEMENT DATE] and the Access Restriction Agreement dated [ORIGINAL ACCESS RESTRICTION AGREEMENT DATE], per its pre-commitment under the Succession Instructions referenced above, without further negotiation or re-execution required.

## 4. Prior Custodian Acknowledgment (if reachable and able)

[PRIOR CUSTODIAN'S NAME] acknowledges the transfer described above. *(If Prior Custodian is not reachable or not able to acknowledge — e.g., due to the unresponsiveness that triggered this succession — state that here instead: "Prior Custodian did not respond to outreach attempted on [DATE(S)] and did not execute this acknowledgment.")*

## Signature

New Custodian representative: _________________________________ Date: ______________

[REPRESENTATIVE'S PRINTED NAME AND TITLE]

Prior Custodian representative (if acknowledging): _________________________________ Date: ______________

[REPRESENTATIVE'S PRINTED NAME AND TITLE, OR NOTE PER SECTION 4]
