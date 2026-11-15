# Contributing

Personal Time Capsule is built in public, on purpose. Per [`PRINCIPLES.md`](./PRINCIPLES.md) #10, we actively want archivists, preservation specialists, cryptographers, estate lawyers, historians, and ordinary people to challenge our assumptions — being told we're wrong early is a feature of this process, not a failure of it.

## If you've never used GitHub before, start here

This is the most common and most valuable kind of contribution, and it requires none of the things that word "contributing" might bring to mind — no coding, no git, no pull requests. Just:

1. Click **[open an issue](../../issues/new/choose)**. (GitHub will ask you to sign in or create a free account first, if you don't have one — it takes about two minutes and just needs an email address.)
2. Pick **"Challenge an assumption."**
3. Fill in the four plain-language prompts it gives you: which document, what's wrong or missing, why it matters, and (optionally) what you'd do instead.
4. Click submit. That's the whole contribution.

An "issue" is just a public note attached to this project — closer to leaving a comment on a shared document than submitting code. You don't need a fix in mind, polished writing, or any credential — Principle #1 is that ordinary observations have real value here. Nothing else in this document is required reading unless you specifically want to edit a document or write code yourself.

## Where the project is right now

- **The idea and the full spec** — done.
- **Legal paperwork** — first drafts exist, but **not usable by anyone yet** — a licensed attorney still needs to review them. If you are one, or know one who might volunteer a look, that's the single most valuable thing this project needs right now.
- **The reference software** (`cli/`) — underway; see its "Known limitations" in [`cli/README.md`](./cli/README.md) for what's still rough, SLIP-0039 key-splitting most of all.

See [`ROADMAP.md`](./ROADMAP.md) for the complete, document-by-document list.

## If you already use git and GitHub

- **Discuss** — comment on an existing issue, or open a new one for a question that doesn't fit the template.
- **Propose doc changes** — small wording/clarity fixes can go straight to a pull request; anything that changes a principle, threat, actor, or lifecycle stage should start as an issue first so it can be discussed before the doc changes.
- **Contribute code** — the same rule applies to `cli/`: open an issue to discuss the approach before a large PR, so design disagreements surface before implementation work is wasted.

## Ground rules

- Be direct. A blunt "this is wrong, here's why" is more useful to us than a soft "have you considered." Principle #10 exists so we can hear the second one without ego getting in the way.
- Cite the document and section you're responding to. This project spans several documents that all cross-reference each other; specificity keeps discussion tractable.
- No credential required. You don't need to be a professional archivist or lawyer to have valuable input — Principle #1 is that ordinary lives (and ordinary observations) have value.

## Licensing of contributions

By submitting a change, you agree it's licensed the same way as the content it changes: specification and documentation contributions under [CC BY 4.0](./LICENSE-SPEC.md), code contributions under [Apache-2.0](./LICENSE).
