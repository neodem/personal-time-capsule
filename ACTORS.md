# Actors

This document defines the roles involved in a capsule's life, and what each can and can't do. These are **roles, not job titles** — one real person or organization may play several (e.g. a creator may also act as their own key-holding trustee for a while; a small custodian institution may also hold a key share). The specification should refer back to these roles rather than inventing new terminology per document.

See [`PRINCIPLES.md`](./PRINCIPLES.md) #2 ("the archive belongs to the creator") and [`THREAT_MODEL.md`](./THREAT_MODEL.md) for the reasoning behind several of the boundaries below.

## Creator

The person whose digital life the capsule documents. In the vast majority of cases, also the person who initiates and builds the capsule.

- **Can:** decide what goes into the capsule and what's excluded; annotate it with human context; set the privacy zone for any item; choose the release policy (including changing it, while the capsule is still under their control); choose the custodian(s) and successor chain; choose or delegate key custody; amend or add to the capsule up until it's sealed and deposited (see `LIFECYCLE.md`); revoke or redirect any of the above at will.
- **Can't:** be forced by the standard itself into a particular structure, level of curation, or disclosure — Principle #6 (preserve the mess) means the tooling should never require tidiness as a precondition for preservation.
- **Holds keys?** Initially yes, by default — the creator controls their own material until they choose to lock it.

## Assistant / delegate

Someone helping a creator who can't or doesn't want to operate the tooling themselves — an adult child helping an aging parent, a caregiver, a friend. Not a separate class of rights; the assistant is only ever acting *as* the creator, on the creator's instruction.

- **Can:** do anything the creator could do, on the creator's behalf and at the creator's direction.
- **Can't:** make decisions the creator hasn't authorized, or continue acting once the creator withdraws that authorization. If the creator is not capable of authorizing (e.g. cognitive decline, incapacity), that's a legal question for Phase 3, not something this role resolves on its own.

## Named subject / third party

A person who appears *in* the archive — a correspondent, a person in a photo, someone mentioned in a journal — but who is not operating any part of the system.

- **Can:** nothing, procedurally — they have no operational role. But they are the reason Principle #7 (privacy is fundamental) exists: the standard has to protect their interests even though they never agreed to be part of the capsule and may still be alive at release.
- **Can't:** consent to or veto inclusion in most cases (the creator made this record, e.g. an email thread, and typically can't retroactively get the other party's sign-off) — which is precisely why sensitive-data scanning, redaction, and privacy zones exist as tooling the *creator* uses on their behalf.

## Beneficiary / recipient

Whoever the creator intends to eventually have access — could be named family members, "any descendant," a research institution, or the general public. Not necessarily known by name at capsule creation time (e.g. "my great-grandchildren").

- **Can:** access the capsule once the release condition is met and unlock occurs; nothing before that.
- **Can't:** alter the release policy, access content early, or compel a custodian to release early — that would defeat the entire point of separating preservation from access (Principle #8).

## Custodian institution

An archive, library, or other institution that physically/digitally holds, preserves, verifies, and replicates a capsule.

- **Can:** store the capsule; run integrity checks against its manifest (fixity checking, per PREMIS/LOCKSS practice); replicate it to other custodians in the succession chain; confirm the capsule's continued existence to the creator or trustees; refuse a deposit it isn't equipped to honor.
- **Can't (by design, where the release policy calls for it):** decrypt or read capsule content before the release condition is met — Principle #8 means holding a capsule and being able to read it are meant to be separable. A custodian may or may not also serve as a key-holding trustee; those are different roles and shouldn't be assumed to be the same party.

## Successor custodian

A custodian institution designated as a fallback if the primary custodian collapses (see `THREAT_MODEL.md` #3). Inert until activated.

- **Can:** everything a custodian institution can, once the succession event occurs and the capsule (or a replica of it) is transferred to them.
- **Can't:** anything, before that point — they may not even know a given capsule exists until the chain activates.

## Key-holding trustee

Whoever holds a decryption key, or a share of one (e.g. via secret-sharing across multiple trustees), for a capsule with a time- or condition-locked release policy. May be a person (family friend, executor, lawyer), an institution, or the custodian itself, depending on what the creator chooses.

- **Can:** hold their key or key-share securely; participate in reconstructing the decryption capability once the release condition and quorum requirements (if any) are met; confirm their continued ability to do so, periodically, to the creator or custodian. For a death-triggered policy, expected to independently satisfy themselves the condition is genuinely met before contributing their share, not merely act on an executor's say-so — see `ACCESS.md`.
- **Can't:** unlock a capsule alone if the policy requires a quorum of multiple trustees (this is the main defense against both coercion and a single point of failure — see `THREAT_MODEL.md` #6 and #7); unlock early regardless of quorum, since the release condition itself (date, event) still has to be satisfied.

## Executor / estate representative

The legally recognized person handling a creator's affairs after death, per the creator's will or applicable law. Relevant mainly for release policies triggered by death.

- **Can:** confirm the triggering event (the creator's death) to custodians and trustees, per whatever legal process Phase 3 defines; carry out instructions the creator left regarding the capsule.
- **Can't:** override the creator's stated release policy just because they're now handling the estate — the capsule's terms were the creator's decision (Principle #2), not the executor's to renegotiate, unless the creator explicitly delegated that discretion; unilaterally trigger unlock — their confirmation starts the process, but per `ACCESS.md` it's the trustee quorum, not the executor alone, that actually has to be satisfied.

## Auditor / verifier

Anyone — a custodian, a successor evaluating whether to accept a transfer, a researcher, a curious member of the public if the manifest is public — checking a capsule's integrity without necessarily having access to its encrypted content.

- **Can:** verify checksums/manifests against the capsule's stated contents; confirm a capsule hasn't been tampered with or corrupted; do this without holding any decryption key at all, since integrity verification and content access are different operations.
- **Can't:** learn the encrypted content itself from this process alone.

## Future researcher / historian

The long-horizon beneficiary the whole project is ultimately for (see `VISION.md`) — someone, decades or centuries out, trying to understand a life or a period through the capsule.

- **Can:** once access is granted, read the capsule using nothing more than a filesystem, ordinary standard software (a text editor, an image viewer, a media player — whatever the file calls for), and the published specification (Principle #3) — no proprietary software, and ideally no still-existing organization, required.
- **Can't:** assume every capsule they encounter followed later, improved versions of the spec — versioning and backward-readability are a specification-phase concern this actor makes real.

## Toolmaker / maintainer

The people (currently: this project's contributors) who build and publish the specification and reference tooling.

- **Can:** propose and evolve the spec and CLI, in public, inviting challenge (Principle #10).
- **Can't:** be a dependency of any individual capsule's validity or accessibility. If this project and every tool it ever produced disappeared tomorrow, a capsule created today must still be readable (Principle #3) — the maintainer role is deliberately *not* load-bearing for any capsule already out in the world.

## Status

First pass, written during Phase 1 (Definition), alongside `THREAT_MODEL.md` and `LIFECYCLE.md`. Expect roles here — especially key-holding trustee and executor — to get more precise once Phase 3 (Legal) work starts.
