# Security

Personal Time Capsule is a non-commercial, open-source project with no company or dedicated security team behind it — see [`README.md`](./README.md). That doesn't make security reports less welcome; if anything, a project whose whole premise rests on encryption, key custody, and long-term integrity guarantees needs this more than most.

## What's in scope

- **`cli/`** — the reference Rust CLI (`ptc init`, `import`, `inspect`, `scan`, `redact`, `seal`, `encrypt`, `verify`, `unlock`, `deposit`). Anything that could corrupt a capsule silently, misreport a failed integrity check as passing, leak key material, or otherwise undermine the guarantees `INTEGRITY.md` and `ENCRYPTION.md` describe.
- **The specification itself** (`FORMAT.md`, `ENCRYPTION.md`, `PRIVACY.md`, `ACCESS.md`, `CUSTODY.md`, etc.) — a design flaw here (not just a bug in the code) is just as much a security issue for anyone who builds on it.

## What's already known and documented, not news

Check each document's own caveats first — in particular:

- **SLIP-0039 key-splitting is not implemented.** `ptc encrypt`/`ptc unlock` hand a raw key directly to the creator; no solid, actively-maintained Rust crate exists yet. See `ENCRYPTION.md`'s "What's deliberately out of scope" and `cli/README.md`.
- **Phase 3 legal documents are not usable as-is** — drafted by non-lawyers, explicitly pending review by a licensed attorney.
- None of the Rust CLI is production-grade custody tooling yet; see each command's own "Known limitations" in `cli/README.md`.

If your finding is one of these, it's already tracked — no need to report it again, though a concrete attack scenario we haven't considered is still useful as a regular [issue](../../issues/new/choose).

## How to report something that isn't already known

For anything you'd rather not disclose publicly first — a way to forge a passing integrity check, corrupt a capsule without detection, extract key material you shouldn't be able to, or a similar real vulnerability — please use GitHub's [private vulnerability reporting](../../security/advisories/new) for this repository instead of opening a public issue. If that's unavailable to you for some reason, open an issue saying only that you have a security concern and asking for another way to reach the maintainer, without describing the vulnerability itself.

There's no bug bounty — this project doesn't have funding — but every report will be read, credited (if you want credit), and taken seriously.
