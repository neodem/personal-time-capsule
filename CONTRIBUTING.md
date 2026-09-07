# Contributing

Personal Time Capsule is built in public, on purpose. Per [`PRINCIPLES.md`](./PRINCIPLES.md) #10, we actively want archivists, preservation specialists, cryptographers, estate lawyers, historians, and ordinary people to challenge our assumptions — being told we're wrong early is a feature of this process, not a failure of it.

## Where the project is right now

**Phase 1 (Definition)** and **Phase 2 (Specification)** are both complete; we're now in **Phase 3 (Legal)**, with first drafts of the deed of gift, access restriction agreement, and custodian agreement — see [`VISION.md`](./VISION.md) for the roadmap. That means the most valuable contribution right now usually isn't code (there isn't much yet). It's scrutiny of the design documents: [`PRINCIPLES.md`](./PRINCIPLES.md), [`THREAT_MODEL.md`](./THREAT_MODEL.md), [`ACTORS.md`](./ACTORS.md), [`LIFECYCLE.md`](./LIFECYCLE.md), [`STANDARDS.md`](./STANDARDS.md), the full Phase 2 model set (`FORMAT.md`, `METADATA.md`, `INTEGRITY.md`, `PRIVACY.md`, `ENCRYPTION.md`, `ACCESS.md`, `CUSTODY.md` — especially `ENCRYPTION.md` and `CUSTODY.md`, which explicitly ask for review by anyone with real cryptographic or institutional-succession experience) — and now [`DEED_OF_GIFT.md`](./DEED_OF_GIFT.md), [`ACCESS_RESTRICTION_AGREEMENT.md`](./ACCESS_RESTRICTION_AGREEMENT.md), and [`CUSTODIAN_AGREEMENT.md`](./CUSTODIAN_AGREEMENT.md), which need something stronger than scrutiny: none of them are usable by anyone until a licensed attorney has actually reviewed them. If you are one, or know one who might be interested, that's the single most valuable thing this project needs right now.

## The most useful thing you can do: challenge an assumption

If something in this repo is naive, wrong, already solved better elsewhere, or missing a threat/actor/case that matters, [open an issue](../../issues/new/choose) using the **Challenge an assumption** template. Point at the specific document and section, say what's wrong or missing, and why it matters. You don't need a fix in hand — surfacing the problem is the contribution.

## Other ways to contribute

- **Discuss** — comment on an existing issue, or open a new one for a question that doesn't fit the template.
- **Propose doc changes** — small wording/clarity fixes can go straight to a pull request; anything that changes a principle, threat, actor, or lifecycle stage should start as an issue first so it can be discussed before the doc changes.
- **Contribute code** — once Phase 4 (Software) starts, the same rule applies: open an issue to discuss the approach before a large PR, so design disagreements surface before implementation work is wasted.

## Ground rules

- Be direct. A blunt "this is wrong, here's why" is more useful to us than a soft "have you considered." Principle #10 exists so we can hear the second one without ego getting in the way.
- Cite the document and section you're responding to. This project spans several documents that all cross-reference each other; specificity keeps discussion tractable.
- No credential required. You don't need to be a professional archivist or lawyer to have valuable input — Principle #1 is that ordinary lives (and ordinary observations) have value.

## Licensing of contributions

By submitting a change, you agree it's licensed the same way as the content it changes: specification and documentation contributions under [CC BY 4.0](./LICENSE-SPEC.md), code contributions under [Apache-2.0](./LICENSE).
