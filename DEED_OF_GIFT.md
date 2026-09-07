# Deed of Gift (draft)

Phase 3 (Legal) — [`ROADMAP.md`](./ROADMAP.md) item 16, the first Phase 3 item.

> ⚠️ **This document, and the template it describes, are drafts prepared by non-lawyers as part of an open-source project. Neither has been reviewed or approved by any licensed attorney. Do not sign, rely on, or treat any version of this material as legally effective for an actual gift, estate plan, or transfer of property. This is explicitly a starting point for professional legal review — not a substitute for it.** See [`CONTRIBUTING.md`](./CONTRIBUTING.md); estate lawyers reviewing this project's Phase 3 work is exactly the kind of scrutiny being actively sought.

The actual fill-in-the-blank template lives at [`templates/legal/deed-of-gift.md`](./templates/legal/deed-of-gift.md); a filled-in, worked example for the fictional Jane Smith capsule lives at [`examples/minimal-capsule/legal/deed-of-gift.md`](./examples/minimal-capsule/legal/deed-of-gift.md).

## What a deed of gift is for, here

The founding conversation ([`research/origin/2026-08-20-initial-conversation.md`](./research/origin/2026-08-20-initial-conversation.md)) already sketched this as a focused transfer instrument — "I hereby transfer..." — deliberately distinct from access restrictions, custodian obligations, and succession, which are their own separate Phase 3 items (17-19). This document doesn't duplicate those, or the technical terms `capsule.yaml` already records (`ACCESS.md`, `CUSTODY.md`) — it's the instrument that legally enacts the transfer those technical and legal documents together govern.

Concretely, a deed of gift needs to:

1. **Identify the parties** — the donor (creator) and the recipient (custodian institution).
2. **Identify the gift precisely** — not "my digital archive" in the abstract, but *this specific, sealed capsule*, referenced by its `capsule.yaml` `capsule.id` and the manifest checksums recorded at sealing. A creator might make more than one capsule over a lifetime, or amend one for years before sealing it; the deed attaches to one sealed instance, not to whatever the creator might do next.
3. **Say what's actually transferred** — custody of the capsule, and a license to preserve/verify/replicate/eventually provide access per its recorded terms — separately from copyright, addressed next.
4. **Settle copyright/ownership of the content** — the actual judgment call this document exists to make a default recommendation on.
5. **Cross-reference, not duplicate, the other governing documents** — the Access Restriction Agreement (item 17), Custodian Agreement (item 18), and `capsule.yaml`'s own recorded `access` terms.
6. **Acknowledge, honestly, what the donor isn't warranting** — see "Third-party material," below.

## Copyright default: the creator retains it; the custodian gets a license

This project's default position — put to the user as a genuine tradeoff, since real archival practice varies and this is exactly the kind of call this project has consistently surfaced rather than picked silently — is that **copyright in the capsule's actual content stays with the creator**, passing to their heirs/estate under ordinary inheritance law, while the **custodian receives custody plus a license** broad enough to actually do its job: preserve, verify, replicate, and eventually provide access per the release policy already recorded in `capsule.yaml`.

This follows directly from `PRINCIPLES.md` #2 (the archive belongs to the creator) — a family's actual literary rights in a parent's letters or a grandparent's diary shouldn't pass permanently to an institution just because the institution is holding the media. The tradeoff, honestly stated: the license granted to the custodian has to be broad enough that the custodian doesn't need to track down and ask an estate's permission for every routine preservation action (format migration, replication to a successor custodian) decades later. The template's license language tries to be that broad without being an outright copyright transfer; whether it actually succeeds at that balance is exactly the kind of thing that needs real legal review.

## Third-party material: an honest limitation, not a warranty

Per `PRINCIPLES.md` #6 (preserve the mess) and `ACTORS.md`'s named subject/third party role, a capsule can legitimately contain material about people who never consented to being archived — that's a known, accepted feature of this project's design, not an oversight. A deed of gift that had the donor warrant "I have obtained all necessary rights and consents for everything in this collection" would be **false** for a huge share of realistic capsules — nobody gets a signed release from every person who ever emailed them. The template instead has the donor represent only that they have the lawful right to donate the collection itself, and explicitly does *not* claim third-party consent — privacy zones, redaction, and the Access Restriction Agreement are where that concern actually gets handled, not a warranty clause that would be routinely false if taken literally.

## Where the deed sits in the capsule, and why it's checksummed

`FORMAT.md` already reserves a bag-root `legal/` directory (a sibling of `data/`, not under it) for exactly this kind of document. This document adds one more structural decision `FORMAT.md` left implicit: **`legal/`'s contents are included in `tagmanifest-sha256.txt`**, frozen at sealing, the same as `capsule.yaml` and `privacy-overrides.yaml`. A signed legal instrument is exactly the kind of thing that should be tamper-evident from the moment it's executed — there's no ongoing-mutation problem to solve here (unlike `preservation-log.yaml`, item 11) because a deed of gift is normally finalized at or before sealing, alongside the `custody` section it formalizes, not something that needs to keep changing afterward.

## What this document explicitly does not cover

- **Access timing and restrictions** — `ROADMAP.md` item 17 (Access Restriction Agreement), plus the technical release policy `ACCESS.md` already defines.
- **The custodian's ongoing obligations** (fixity-check cadence, succession) — `ROADMAP.md` item 18 (Custodian Agreement), building on `INTEGRITY.md` and `CUSTODY.md`, already specified technically.
- **What happens if the custodian collapses** — `CUSTODY.md` (already done) plus whatever `ROADMAP.md` item 18/19 formalizes legally.
- **Tax consequences of the gift.** Real charitable gifts to qualifying institutions can carry tax implications in some jurisdictions; this document says nothing about that at all. A real donor needs their own tax advice, separate from this project's scope entirely.
- **Jurisdiction-specific law.** This template is written in a generic, US-leaning frame (the author's own context) and makes no attempt to be valid across jurisdictions — see Open questions.

## Open questions

- **Jurisdiction.** Deed-of-gift practice, copyright law, and what makes a gift legally effective at all vary significantly by country and, within the US, by state. This template picks no jurisdiction and isn't reviewed for any specific one — a placeholder (`Section 7`) exists precisely because this needs real legal input, not a guess.
- **Precedence between this Deed and the other governing documents** (`capsule.yaml`'s recorded terms, the Access Restriction Agreement, the Custodian Agreement) isn't resolved — left as an explicit placeholder in the template rather than guessed at.
- **Whether "gift" is even the right legal frame for encrypted, not-currently-readable content.** Most real deeds of gift concern material the recipient can actually read or examine. A capsule with a `never`-release or decades-locked collection is, at the moment of the gift, something *nobody* — including the recipient — can currently access. Whether that changes anything legally about what's being "given" is a genuinely interesting question this project doesn't have the expertise to answer, and exactly the kind of thing worth a real estate attorney's specific attention.

## Status

First pass, written during Phase 3 (Legal), addressing [`ROADMAP.md`](./ROADMAP.md) item 16. Unlike the Phase 2 model documents, this one isn't just "open to challenge" in the usual sense — it should be treated as **not usable** until a licensed attorney has actually reviewed it. See [`CONTRIBUTING.md`](./CONTRIBUTING.md).
