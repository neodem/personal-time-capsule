<!--
Personal Time Capsule — Creator Instructions worksheet (v0.1, DRAFT)

This is different from the other four Phase 3 documents (deed of
gift, access restriction agreement, custodian agreement, succession
instructions): those are agreements between the creator and an
institution, reviewed and signed by both sides. This one isn't signed
by anyone else — it's the creator's own letter of instruction to
whoever they leave behind (executor, family, trustees), written in
plain language rather than legal language.

Because of that, it doesn't carry the same "don't use this until a
lawyer reviews it" risk the other four do — there's no institution
whose obligations depend on getting this right, and no one else signs
it. That said, if anything here touches your actual will or estate
plan, it's still worth showing to whoever is helping with that.

WHERE THIS SHOULD LIVE: unlike the other four documents, this one is
likely to need updates over the years (new contact info, a trustee
who's moved, second thoughts about something) even after your capsule
is sealed and deposited. Keep it with your other estate planning
papers — with your will, told to your executor — rather than only
inside the sealed capsule, where it would be frozen and unchangeable
once sealing happens. You can also put a copy in the capsule's legal/
directory if you want, understanding that copy will stay exactly as
it was on the day you sealed the capsule.

Fill in whatever sections are relevant to your capsule; skip what
doesn't apply. This is a worksheet, not a form with required fields.
-->

# CREATOR INSTRUCTIONS

For: [YOUR NAME]
Concerning capsule: [capsule.yaml `capsule.id`]
Last updated: [DATE]

## 1. Where things are

- **The sealed capsule itself** is held by: [custodian name and contact info], and/or a copy is at: [any other location — a hard drive, a cloud account, etc.]
- **This worksheet** is kept: [with my will / with my executor / other — say where]
- **My deed of gift, access restriction agreement, custodian agreement, and succession instructions** are: [with the capsule / with my attorney / other]

## 2. Who to contact, in order

1. [Name, relationship, contact info] — [why they're first, e.g. "my executor"]
2. [Name, relationship, contact info]
3. [Name, relationship, contact info]

## 3. Key-holding trustees (if my capsule is encrypted)

If part of my capsule is encrypted, the people below each hold a piece of the key needed to unlock it. No single one of them can unlock anything alone — that's on purpose. See `ACCESS.md` and `ENCRYPTION.md` if you want the technical explanation; here's the plain version: a certain number of them (not all) need to agree the release condition has actually been met before anything unlocks.

| Name | Contact | What they hold |
|---|---|---|
| [name] | [contact] | [e.g. "1 of 5 shares"] |
| [name] | [contact] | |

## 4. What actually happens when I die (or another release condition is met)

In plain language, not legal language:

1. Whoever handles my affairs (my executor, or whoever's first on the list above) confirms what happened and lets the people in Section 3 know, along with the custodian.
2. The trustees each independently check that this is real before doing anything — that's a safeguard, not distrust of any one of them.
3. Once enough of them agree, the relevant part of my capsule unlocks. Other parts may stay locked longer, or forever, depending on what I set up — see the Access Restriction Agreement for the actual schedule.
4. None of this needs to happen quickly. There's no deadline that expires if people take their time to get it right.

## 5. What NOT to do

- Don't ask the custodian to release anything early "just this once." They're not supposed to say yes, and if they do, it's supposed to trigger the succession process in `CUSTODY.md` — not something anyone should actually want to happen.
- Don't worry if the custodian institution closes or merges someday. There's a backup custodian named in the capsule for exactly that reason.

## 6. Personal context

[This is the part with no legal or technical counterpart — why you built this, what matters about it, anything you want whoever reads this to know before they read the capsule itself. As much or as little as you want.]

## 7. Review reminder

I intend to review and update this worksheet: [e.g. "every few years," "whenever a trustee or contact changes," "no particular schedule"]. Last reviewed: [DATE].
