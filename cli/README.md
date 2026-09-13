# ptc (reference CLI) — experimental

An early, experimental prototype of the reference CLI described in [`ROADMAP.md`](../ROADMAP.md) items 21–29 (plus 25a). `ptc init`, `ptc import`, `ptc inspect`, `ptc scan`, `ptc redact`, `ptc seal`, `ptc encrypt`, and `ptc unlock` exist so far. This is Phase 4 (Software) work — Phase 2 (Specification) and Phase 3 (Legal, first-draft stage) are both complete, but Phase 3's legal documents still need review by a licensed attorney before they're usable; nothing here should be treated as fully authoritative until that's happened.

Written in Rust; licensed [Apache-2.0](../LICENSE) like the rest of this project's code/tooling.

## Stability

**The CLI itself — its subcommands, flags, output, and exit codes — is this project's public integration contract**, the thing other tools (including a future GUI) are meant to build on, the same way tools build on `git`/`ffmpeg`/`pandoc`: by running `ptc` as a subprocess and reading its output, not by linking against it. This is a deliberate choice, consistent with `PRINCIPLES.md` #3 (no proprietary lock-in) — the promise shouldn't require anyone to write Rust.

This crate also exposes a library target (`ptc::model`, `ptc::checksum`, `ptc::seal`, ...) that a Rust-native GUI toolkit is welcome to depend on directly instead of shelling out. That's a genuine option, not a formality — but it comes with **no stability promise**. The library's types and function signatures are free to change between versions without notice; only the compiled binary's behavior is versioned as a public contract.

## Building

```sh
cargo build
```

### Windows-specific gotcha (GNU target)

If `rustup show` reports your host as `x86_64-pc-windows-gnu` rather than `-msvc`, a bare `cargo build` fails with `dlltool.exe: program not found` or similar — the GNU target needs a `dlltool`/linker this crate's dependencies require, and a bare `rustup` install doesn't put one on `PATH`. Three ways to fix it, in the order worth trying:

- **Point Cargo at rustup's own bundled toolchain first** — the GNU target actually ships a complete, self-contained `dlltool`/`gcc`/`ld` already, at a path that's guaranteed not to contain spaces (unlike a manual MinGW install under `C:\Users\<name>\...`, which breaks if the username has a space — see below):
  ```sh
  set RUSTFLAGS=-C dlltool=%USERPROFILE%\.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\x86_64-pc-windows-gnu\bin\self-contained\dlltool.exe -C linker=%USERPROFILE%\.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\x86_64-pc-windows-gnu\bin\self-contained\x86_64-w64-mingw32-gcc.exe
  ```
  (Adjust the `stable-x86_64-pc-windows-gnu\...` prefix to match your actual `rustup toolchain list` / `RUSTUP_HOME` if it's not the default location.) This is the simplest fix — nothing extra to install, and it's what this project's own examples were built and verified with.
- **Switch to the MSVC target** (the other standard option): install Visual Studio Build Tools with the "C++ build tools" workload, then `rustup default stable-x86_64-pc-windows-msvc`.
- **Or install a complete MinGW-w64 yourself** (e.g. via `winget install BrechtSanders.WinLibs.POSIX.UCRT`) and point `RUSTFLAGS` at *that* instead. If your Windows username contains a space, make sure the path you point at doesn't pass through any directory with a space in it — some GNU binutils/gcc internals mishandle that, and a junction/symlink to a space-free path is **not** sufficient (`gcc` resolves its own real path internally). This is why the first option above is generally easier: rustup's bundled copy already lives at a space-free path regardless of your username.

This is an environment quirk, not a project dependency — it doesn't affect the MSVC target or Linux/macOS builds.

### Building inside a cloud-sync folder (Dropbox, OneDrive, Google Drive, ...)

If this repo lives inside a folder actively synced by Dropbox or similar, `cargo build`/`cargo test` can intermittently fail with `failed to remove temporary directory: The process cannot access the file because it is being used by another process` — the sync client grabs a lock on a `target/` temp file at the same moment Cargo tries to rename it. Retrying sometimes works, but the reliable fix is building somewhere the sync client doesn't watch:

```sh
set CARGO_TARGET_DIR=%TEMP%\ptc-target
cargo build
```

## Usage

```sh
ptc init <path> [--creator-name NAME] [--creator-contact CONTACT] [--purpose TEXT]
```

Scaffolds a new capsule skeleton at `<path>`: `bagit.txt`, a starter `capsule.yaml`, and `data/person/` (`FORMAT.md`'s human-context layer) populated with starter `biography.md`, `timeline.md`, `people.yaml`, and `places.yaml` — not just the empty directory. These are genuinely empty guidance text, not fictional example content (per Principle #2, a creator's own life isn't invented for them) — `--creator-name`, if given, only shows up in `biography.md`'s heading. All flags are optional — omitted fields are simply left out of the generated file rather than filled with placeholder junk.

`access.default_release_policy` is always written as `type: custom` with a `"TBD"` description — `ptc init` has no business inventing a real release policy on a creator's behalf. `ptc seal` (below) refuses to run until that placeholder has been replaced with a real one.

```sh
ptc import <path> --source <file-or-dir> --collection NAME [--source-description TEXT] [--agent TEXT]
```

Copies a file or a whole directory tree into `<path>/data/<collection>/` (`LIFECYCLE.md` stage 2). A directory source is nested under its own name — `--source .../1945 --collection photos` lands at `data/photos/1945/...`, not flattened directly into `data/photos/` — so importing several source directories into one collection (a "1945" folder and a "1946" folder both going into "photos", say) can't silently collide or blend together. A single-file source has no such structure to preserve and lands directly under the collection. If `--collection` isn't already declared in `capsule.yaml`, `ptc import` registers it there (`name`/`path` only — privacy zone, release policy, etc. are left for the creator to add later). Either way, it appends an `import` event to `data/provenance/events.yaml` (`METADATA.md`, `ROADMAP.md` item 10), creating that file with a short header if it doesn't exist yet — including the batch's actual destination (`path`, e.g. `data/photos/1945`) alongside `source`'s free-text description, so a collection assembled from several batches over time stays traceable to which batch a given file came from.

`ptc import` refuses (non-zero exit, clear error, nothing copied) if: the source doesn't exist; any destination file would be overwritten; the destination is inside (or is) the source; or `capsule.yaml`'s `status.lifecycle_stage` is already past the pre-seal amend loop (`sealing` or later) — importing at that point would silently invalidate an existing manifest instead of being caught as tampering, so re-seal after importing if material needs to be added that late.

Like `ptc seal`, `capsule.yaml` is patched surgically (a new `collections:` entry, and `status.lifecycle_stage` advanced from `origination` to `import` the first time) rather than re-serialized, so hand-written comments survive.

```sh
ptc inspect <path>
```

A read-only summary of a capsule's current state — no writes, no exit-code gate to satisfy first. Prints: capsule identity and lifecycle stage; whether the `data/person/` human-context layer's four files exist (with entry counts for `people.yaml`/`places.yaml`); `data/provenance/events.yaml`'s event count and most recent entry; each declared collection's actual on-disk file count/size plus its *resolved* privacy zone and release policy (falling back to `capsule.yaml`'s capsule-wide default and saying so, rather than just printing "not set"); any directory under `data/` that isn't `person`/`provenance` or a declared collection (material that landed there without going through `ptc import`); whether `privacy-overrides.yaml`, `legal/`, and `preservation-log.yaml` are present; and whether `manifest-sha256.txt`/`tagmanifest-sha256.txt` exist (their *presence*, not their correctness — verifying checksums against disk is `ptc verify`'s job, `ROADMAP.md` item 27, not built yet).

Sits, per `LIFECYCLE.md`'s command ordering, between `import`/annotation and `scan`/screening: "what's actually in here right now," distinct from integrity verification or sensitive-data scanning. Everything it reads is read leniently (no `deny_unknown_fields`) — a hand-edited or future-schema-extended file shouldn't make a reporting tool itself fail to run; structural validation is `ptc verify`'s job.

```sh
ptc scan <path> [--collection NAME]
```

`LIFECYCLE.md` stage 4 (Screening) tooling: walks the capsule's payload (all of `data/`, or one collection with `--collection`) looking for candidate sensitive-data findings and prints a report — a file/line, a category (`PRIVACY.md`'s vocabulary: `financial`, `credentials`, `government-id`), a description, and a masked excerpt where anything is safe to show at all (an SSN or credit card number's last 4 digits; nothing for a matched credential, ever). Detects: US SSN format; credit card numbers (Luhn-checksum validated to cut false positives on arbitrary long digit runs); AWS access key IDs; PEM private key block headers; and a generic `password`/`api_key`/`secret`/`token` assignment heuristic. Files that look binary (a NUL byte in a leading sample, same heuristic `git` uses) are skipped, not scanned.

**`ptc scan` writes nothing, ever** — no `privacy-overrides.yaml` entry, draft or otherwise. Per `LIFECYCLE.md`'s own framing of this stage ("the tooling surfaces candidates, the creator decides," `PRINCIPLES.md` #2), turning a finding into an actual override is the creator's own edit to `privacy-overrides.yaml` (see `PRIVACY.md`), using the category `ptc scan` printed as a starting point.

**Deliberately not attempted**: `medical` and `third-party-privacy` (`PRIVACY.md`'s other two starter categories) need semantic understanding — knowing a name belongs to someone other than the creator, or that a paragraph describes a diagnosis — that no regex can responsibly claim to provide, so `ptc scan` doesn't pretend to detect them.

```sh
ptc redact <path> --target <rel-path> [--apply]
```

`LIFECYCLE.md` stage 4 (Screening) tooling, alongside `ptc scan`: where scan only reports, `ptc redact` actually neutralizes findings in one creator-named file (`--target`, relative to the capsule root, e.g. `data/email/2020-taxes.eml`). **Dry run by default** — without `--apply` it prints exactly the same report `ptc scan` would show for this one file and writes nothing; only `--apply` rewrites it. Every matched SSN, credit card number, and AWS access key is replaced with `[REDACTED]`; a credential-assignment match keeps its label and redacts only the value (`password: [REDACTED]`, not the whole line).

**A private key block header is deliberately never auto-redacted.** The detector only recognizes the `-----BEGIN...-----` line, not the multi-line key body that follows — "redacting" just that line would blank the label while leaving the actual key material intact, which is worse than not touching it at all since it would look handled. `ptc redact` still reports it (tagged `[MANUAL REVIEW -- not auto-redactable]`) so the creator knows to handle it by hand, but `--apply` skips it.

**Answers `PRIVACY.md`'s own open question about redaction mechanics** (item 25's job: "kept alongside in a stricter zone, or overwritten entirely?") by overwriting the target file in place, with **no automatic backup copy**. The safety net is the workflow — dry run first, explicit `--apply` to commit — not a hidden file; a creator who wants an undo path should keep their own copy or use version control before applying. A successful `--apply` logs a `screening` event to `data/provenance/events.yaml` (`path`, and an `outcome` summarizing how many redactions were applied).

`ptc redact` refuses (non-zero exit, clear error, nothing written) if: `--target` isn't under `data/` (payload, not a tag file like `capsule.yaml`); the target doesn't exist or isn't a file; the target looks binary; or the capsule is already past the pre-seal amend loop (`sealing` or later) — same reasoning, and the same shared guard, as `ptc import`.

```sh
ptc seal <path>
```

Seals a capsule (`LIFECYCLE.md` stage 5, `ROADMAP.md` item 25a — there was no command for this stage before): computes `manifest-sha256.txt` over everything under `data/`, writes `bag-info.txt` (`Bagging-Date`, `Payload-Oxum`, `External-Identifier`, ...), confirms `bagit.txt` exists, computes `tagmanifest-sha256.txt` over whichever known tag files actually exist (`bagit.txt`, `bag-info.txt`, `capsule.yaml`, `privacy-overrides.yaml` if present, everything under `legal/` if present), and marks `capsule.yaml`'s `status.lifecycle_stage` as `sealing`.

`ptc seal` refuses to run (non-zero exit, clear error) if `access.default_release_policy` is still `ptc init`'s placeholder — edit `capsule.yaml` first. See `FORMAT.md`'s `access` section for the real release policy types.

**A deliberate limitation**: updating `status.lifecycle_stage` patches just that one line of `capsule.yaml` in place, rather than re-serializing the whole file — a full re-serialize would silently strip any comments a creator (or `ptc init`) added. This means `ptc seal` can't currently update any *other* field in `capsule.yaml`; if that's ever needed, it'll need the same careful, targeted approach, not a blanket rewrite.

```sh
ptc encrypt <path> [--apply]
```

`LIFECYCLE.md` stage 6, which happens *after* sealing (stage 5). Encrypts every payload file in each unit — a named collection, or `data/person`/`data/provenance` under the capsule-wide default — whose resolved `release_policy` isn't `immediate`, one fresh 256-bit XChaCha20-Poly1305 key per unit (`ENCRYPTION.md`'s "one key per collection, not one key per capsule"). Each encrypted file's plaintext bytes are replaced by ciphertext with `.enc` appended to the filename; the original plaintext is removed once the ciphertext is written. **Dry run by default** — without `--apply` it reports which units would be encrypted and how many files each has, generating no keys and touching no files; only `--apply` does the real thing. Re-running `ptc encrypt --apply` on an already-fully-encrypted capsule is a safe no-op (it checks the filesystem for `.enc` files, not a possibly-stale status field) — see the note on `capsule.yaml` below for why a live status field isn't available here anyway.

Requires `capsule.yaml`'s `status.lifecycle_stage` to be exactly `sealing` (i.e., `ptc seal` has run and nothing has re-sealed since) — refuses otherwise.

**`ptc encrypt` never writes to `capsule.yaml`.** `INTEGRITY.md` says `capsule.yaml` and the other identity-defining tag files stay frozen forever once sealed, with no defined mechanism to legitimately amend one afterward (`reseal` covers `manifest-sha256.txt` only). That also means `capsule.yaml`'s `status.lifecycle_stage` can never show anything past `sealing` in the file itself, even after real encryption/deposit/decades of dormant preservation actually happen — a reader determining a capsule's true current stage should consult `preservation-log.yaml`'s most recent entry once `lifecycle_stage` reads `sealing`, not treat that field as live. `ptc encrypt` records what it did there instead: one `encryption`-typed entry per unit, written immediately after that unit finishes (not batched at the end), so a run interrupted partway through still leaves a truthful record of exactly what completed.

**Built around a `Cipher` trait (`cli/src/cipher.rs`), not `chacha20poly1305`'s API directly.** `ENCRYPTION.md` locks `capsule.yaml`'s `encryption.algorithm` to the single value `"xchacha20-poly1305"` — deliberately not creator-selectable, for real downgrade-attack and maintenance reasons that document explains. That's a decision about the *format*, not this codebase's internals: the trait exists so a future `spec_version` bump to a different algorithm means writing one new `Cipher` impl and changing `cipher::default_cipher`'s one line, not re-deriving `encrypt.rs`'s file-walking and key-management logic around a new crate's API shape.

**Key-splitting (SLIP-0039) is not implemented.** `ENCRYPTION.md` calls for splitting each unit's key into a trustee quorum via SLIP-0039, but no solid, actively-maintained Rust crate for it exists: the one real implementation, `sssmc39`, is unmaintained, pre-1.0, and self-described as a work-in-progress (last touched 2020); the other, `slip39`, is GPL-3.0-or-later, license-incompatible with this project's Apache-2.0 code. Until that changes, `--apply` prints each unit's raw key (hex-encoded, 256-bit) directly to stdout with a clear warning — **the creator is responsible for recording and distributing it themselves**; nothing splits it, nothing stores it, and it is not shown again.

```sh
ptc unlock <path> --collection NAME [--key HEX] [--apply]
```

`LIFECYCLE.md` stage 10, the inverse of `ptc encrypt`: given one unit's key (a collection name, or `person`/`provenance`, matching whatever `ptc encrypt` reported it as), decrypts every `.enc` file in that unit back to plaintext and removes the ciphertext. **Takes the raw key directly** — there's no SLIP-0039 quorum reconstruction to do, since `ptc encrypt` never split it in the first place (see above). `--key` takes the hex string `ptc encrypt --apply` printed; if omitted, `ptc unlock` reads one line from stdin instead, which avoids the key ending up in shell history the way a flag would.

**Dry run by default**, same convention as `ptc redact`/`ptc encrypt`: without `--apply`, every `.enc` file in the unit is decrypted into memory to confirm the key actually works, and nothing is written or removed. This makes a dry run a legitimate way to answer "is this the right key for this collection?" without committing to anything. `--apply` performs the same decryption and then writes the plaintext and removes the ciphertext.

**Validates every file before writing any of them.** If the key is wrong (or it's the wrong unit's key), decryption fails on the very first file — one key belongs to exactly one unit, so there's no scenario where some files in a unit decrypt and others don't — and nothing on disk is touched at all, rather than leaving the unit half-unlocked. Re-running `ptc unlock --apply` on a unit with no `.enc` files left is a safe no-op, the same idempotency convention `ptc encrypt` uses.

A successful `--apply` logs an `unlock` event to `preservation-log.yaml`, the same reasoning as `ptc encrypt`: unlock happens long after sealing, so it never touches `capsule.yaml` (frozen forever once sealed — see `ptc encrypt`'s own note on this, above).

## Known limitations / next steps

- **Non-interactive only.** Both commands take everything via flags/positional args. A future version should walk a non-technical creator through the same choices interactively — the target audience for this whole project is explicitly *not* assumed to be technical.
- No `--force` flag to reinitialize `ptc init` into a non-empty directory (intentional, for now — safer default).
- **Structural, not full JSON Schema, validation.** `ptc init`/`ptc seal` both read and write `capsule.yaml` through real `serde` types (`ptc::model`) that mirror `schema/capsule.schema.json`'s required fields, types, and enum values, with `deny_unknown_fields` rejecting anything the schema wouldn't allow — a real improvement over the original hand-built-string approach, which validated nothing at all. What's still missing: the schema's *conditional* requirements (e.g. `release_policy.date` being required specifically when `type: fixed_date`) aren't enforced by the Rust types alone. A future `ptc verify` running an actual JSON Schema validator against the parsed YAML would be the authoritative check.
- `ptc seal` doesn't yet handle encrypted collections specially (that's `ptc encrypt`, not built yet) — it hashes whatever is on disk under `data/` at the time it's run.
- `ptc import` only sets a collection's `name`/`path` when auto-registering it — privacy zone, release policy, and encryption settings are left as the creator's own edit, same as `ptc init` leaving the release policy as a placeholder.
- `ptc inspect`'s collection-size and undeclared-directory scans are simple recursive walks with no encryption-awareness -- now that `ptc encrypt` exists, an encrypted collection's file count is still accurate (it counts `.enc` files like any other), but `ptc inspect` doesn't yet surface "this collection is encrypted" as its own fact; that's discoverable today only by noticing the `.enc` extensions or checking `preservation-log.yaml`.
- `ptc scan`'s credential-assignment heuristic (`password:`/`api_key=`/... followed by an 8+ character value) is deliberately broad and will produce false positives (an empty template field, a made-up placeholder) -- acceptable since nothing acts on a finding automatically, but worth knowing before treating a long finding list as alarming.
- `ptc scan`/`ptc redact` read whole files into memory with no size cap -- fine at personal-archive scale, but a pathologically large plaintext file could be slow. No cap was added since nothing else in this codebase caps file size either (`ptc seal` hashes full files the same way).
- `ptc redact` only targets one file per invocation -- there's no `--collection`-wide "redact everything scan would flag" mode. Deliberate: pointing the tool at a specific file is itself part of the creator's decision (`PRINCIPLES.md` #2), not something to automate away.
- `ptc redact --apply` has no undo mechanism beyond whatever backup/version control the creator kept themselves -- see its "no automatic backup copy" note above.
- `ptc encrypt` isn't resumable/atomic across an interrupted run. Each file is written then its plaintext removed, one at a time; if the process dies mid-unit, files already converted stay `.enc` (fine) but any not yet reached are still plaintext, and a subsequent `ptc encrypt --apply` will treat that unit as not-yet-encrypted and issue it a *new* key covering only the remaining files -- the unit ends up split across two keys with no record tying them together beyond two separate `preservation-log.yaml` entries. Acceptable for a personal-scale reference tool; a production custody system would want this to be transactional.
- `ptc encrypt --apply` prints each unit's raw key to stdout, which can linger in terminal scrollback/history -- a real, acknowledged exposure surface, accepted for now as the simplest, most transparent option until SLIP-0039 splitting (or some other key-handling story) exists.
- `ptc unlock --key` on the command line has the same shell-history exposure as `ptc encrypt`'s printed key -- prefer piping the key in over stdin (the default when `--key` is omitted) where that's practical.
- `ptc unlock` decrypts a whole unit's `.enc` files into memory before writing anything, so it can validate the key up front -- fine at personal-archive scale, consistent with the rest of this codebase's no-size-cap approach, but a very large encrypted collection would use meaningfully more memory during `--apply` than `ptc encrypt` did to create it (which writes one file at a time).
- No `ptc deposit` yet (`ROADMAP.md` item 29) -- the last one.
