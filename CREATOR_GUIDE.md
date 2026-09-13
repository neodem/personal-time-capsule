# A Guide for Creators

This is the plain-language version of this project. If you want the technical specification — directory layouts, file formats, schemas — that's [`FORMAT.md`](./FORMAT.md). This document is for you if you're just trying to understand what a "capsule" is, why you might want to make one, and roughly what that would involve. No technical background assumed.

## What is a capsule?

A capsule is a package of your digital life — email, photos, writing, audio, video, whatever you've got — put together in a way that's meant to survive you by decades, maybe longer, and eventually be readable by someone who never met you: a descendant, a historian, a stranger doing research on what ordinary life was like in your time.

It's not a backup. A backup is for you, so you don't lose your own stuff next year. A capsule is for someone else, much later, who has no idea who you are yet.

## Why would I want to do this?

Most of what makes up a life today — texts, emails, phone photos, casual writing — sits in accounts and formats that don't outlive their owner by much. Cloud accounts get closed. Old hard drives stop spinning. Nobody exports their email before they die. A century from now, the letters and photo albums of ordinary people from 1926 are treasured historical sources. The digital equivalent from today is on track to just... not exist.

You don't need to be famous or important for this to matter. Historians have always cared as much about ordinary, unremarkable records — a grocery list, a boring argument in an email thread, a bad vacation photo — as they do about the official, curated record of a time. That mundane material, in context, at scale, is often more revealing than anything anyone meant to preserve.

## What you stay in control of

This project is built around one rule above the others: **it's your archive, and it stays your decision.** Concretely, that means:

- **You decide what goes in.** All of it, some of it, one folder of it — your call, and you can change your mind for as long as the capsule is still in your hands.
- **You don't have to clean anything up first.** A messy inbox or an unsorted photo library is not a reason to wait. Preserving the mess is explicitly fine — arguably it's more honest than a tidied-up version would be.
- **You decide who eventually sees it, and when.** Right away, at a fixed date, only after you've died, a set number of years after that, or never for parts you want to keep private forever. Different parts can have different rules — your journal doesn't have to open on the same day as your vacation photos.
- **You decide what stays private.** Anything involving other people — a friend's email address, a kid's face, a financial detail — is something you get to screen for and hold back, restrict, or exclude entirely. Nobody else consented to being in your archive; that's on you to account for, and the tooling is meant to help you do it, not decide it for you.
- **Nothing here depends on this project surviving.** Your capsule is built to be opened with ordinary software decades from now — a text editor, a photo viewer, whatever a normal file already needs — even if this project, and everyone involved in it, is long gone.

## How it would actually work

Roughly, in the order you'd do it:

1. **Decide to start.** Doesn't need to be complete or well thought out — pick a starting point (an email export, a photo folder) and begin.
2. **Bring your stuff in.** Email, photos, documents, whatever you're including, moved into the capsule as-is. Nothing gets rewritten or "cleaned" at this stage.
3. **Add the human context.** Who's in it, what mattered, what was going on in your life or the world at the time. This is the part that turns a folder of files into something a stranger could actually understand later — arguably the most valuable part of the whole thing, and entirely optional to how much of it you do.
4. **Screen for anything sensitive.** A pass — assisted by tooling, decided by you — over the material to catch things like account numbers, other people's private information, or anything you'd rather restrict or leave out.
5. **Lock it.** Once you're satisfied, the capsule is sealed (so any later tampering or corruption is detectable) and, if you've chosen a release date other than "immediately," encrypted so it can't be opened early — not even by whoever ends up storing it.
6. **Hand it off.** The finished capsule goes to an institution (a library, archive, or similar) willing to hold onto it, check on it periodically, and pass it along safely if something happens to them before your release date arrives.

You can go back and add to, revise, or reorganize a capsule at any point before you lock it. Nothing about this has to happen all at once, and a capsule you never lock or hand off is still a legitimate personal archive — you don't have to finish stage 6 for stages 1–4 to be worthwhile.

You also don't have to do this only once. Locking and handing off a capsule doesn't mean you're done adding to your own history — you can start a new one later that picks up where the last one left off, whenever you have enough new material to make that worthwhile. Doing this every several years, rather than waiting for one single archive that feels completely finished, actually gets more of your life to safety sooner, since nothing is protected by redundant, independently-held copies until it's locked and handed off. If you do this, mention the earlier capsule in the new one — there's a field for it in the technical spec (`series`) — so whoever eventually holds both knows they belong together.

## What's real today, and what isn't yet

Being straight about this: **this project is still being designed.** There is a written specification in progress and one small, experimental piece of software (`ptc init`, which only scaffolds an empty capsule skeleton — see [`cli/README.md`](./cli/README.md)). There is not yet a way to actually walk through the six steps above end-to-end. [`ROADMAP.md`](./ROADMAP.md) tracks exactly what's done and what isn't, item by item, if you want the specifics.

If you're reading this hoping to make a capsule today: not yet, but soon-ish, and the design work happening now is aimed squarely at making that process usable by someone with no technical background at all — that's the whole point.

## Common questions

**Do I need to be technical to eventually do this?** No. That's a hard requirement of the project, not an aspiration — the tooling is meant to walk you through it, not hand you a spec to implement yourself.

**What if I change my mind about something after I've locked it?** Before you lock (seal) a capsule, change anything, anytime. After locking, the whole point is that it can't be quietly altered — that's what makes it trustworthy to whoever eventually opens it. If your circumstances genuinely change, that's a conversation with whoever's holding your capsule, not something the format itself is designed to make easy to walk back.

**What if the archive/library holding my capsule closes down?** Capsules are meant to be handed off with a backup plan — a chain of fallback institutions — specifically so one institution disappearing doesn't mean your archive is lost. This part of the design (`ROADMAP.md` item 15) isn't finished yet, but it's a first-class concern, not an afterthought.

**Can I include stuff about other people — emails from friends, photos with other people in them?** Yes, and honestly that's most of what makes an archive like this valuable. The screening step exists specifically so you can think about what to restrict or leave out on their behalf, since they never agreed to be part of your capsule.

**Is this a company or a product I'll have to pay for?** No. There's no company, no product, and no plan to monetize this, ever — see [`README.md`](./README.md).

## Where to go next

- [`README.md`](./README.md) and [`VISION.md`](./VISION.md) — the fuller pitch and mission
- [`ROADMAP.md`](./ROADMAP.md) — exactly what's built and what isn't, right now
- [`FORMAT.md`](./FORMAT.md) — the technical specification, if you want it
- [`CONTRIBUTING.md`](./CONTRIBUTING.md) — if something here seems wrong, missing, or naive, that's wanted feedback, not an inconvenience

## Status

First pass at [`ROADMAP.md`](./ROADMAP.md) item 9 (human-readable documentation), written during Phase 2 (Specification). Meant to stay in sync with the process this document describes as the underlying models (items 10–15) get designed — if those change what a creator actually experiences, this document should change with them.
