# ptc (reference CLI) — experimental

An early, experimental prototype of the reference CLI described in [`ROADMAP.md`](../ROADMAP.md) items 21–29 (plus 25a). `ptc init`, `ptc import`, and `ptc seal` exist so far. This is Phase 4 (Software) work — Phase 2 (Specification) and Phase 3 (Legal, first-draft stage) are both complete, but Phase 3's legal documents still need review by a licensed attorney before they're usable; nothing here should be treated as fully authoritative until that's happened.

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

Scaffolds a new capsule skeleton at `<path>`: `bagit.txt`, an empty `data/person/` directory, and a starter `capsule.yaml`. All flags are optional — omitted fields (per Principle #2, the creator controls what identifying information appears at all) are simply left out of the generated file rather than filled with placeholder junk.

`access.default_release_policy` is always written as `type: custom` with a `"TBD"` description — `ptc init` has no business inventing a real release policy on a creator's behalf. `ptc seal` (below) refuses to run until that placeholder has been replaced with a real one.

```sh
ptc import <path> --source <file-or-dir> --collection NAME [--source-description TEXT] [--agent TEXT]
```

Copies a file or a whole directory tree into `<path>/data/<collection>/` (`LIFECYCLE.md` stage 2). A directory source is nested under its own name — `--source .../1945 --collection photos` lands at `data/photos/1945/...`, not flattened directly into `data/photos/` — so importing several source directories into one collection (a "1945" folder and a "1946" folder both going into "photos", say) can't silently collide or blend together. A single-file source has no such structure to preserve and lands directly under the collection. If `--collection` isn't already declared in `capsule.yaml`, `ptc import` registers it there (`name`/`path` only — privacy zone, release policy, etc. are left for the creator to add later). Either way, it appends an `import` event to `data/provenance/events.yaml` (`METADATA.md`, `ROADMAP.md` item 10), creating that file with a short header if it doesn't exist yet.

`ptc import` refuses (non-zero exit, clear error, nothing copied) if: the source doesn't exist; any destination file would be overwritten; the destination is inside (or is) the source; or `capsule.yaml`'s `status.lifecycle_stage` is already past the pre-seal amend loop (`sealing` or later) — importing at that point would silently invalidate an existing manifest instead of being caught as tampering, so re-seal after importing if material needs to be added that late.

Like `ptc seal`, `capsule.yaml` is patched surgically (a new `collections:` entry, and `status.lifecycle_stage` advanced from `origination` to `import` the first time) rather than re-serialized, so hand-written comments survive.

```sh
ptc seal <path>
```

Seals a capsule (`LIFECYCLE.md` stage 5, `ROADMAP.md` item 25a — there was no command for this stage before): computes `manifest-sha256.txt` over everything under `data/`, writes `bag-info.txt` (`Bagging-Date`, `Payload-Oxum`, `External-Identifier`, ...), confirms `bagit.txt` exists, computes `tagmanifest-sha256.txt` over whichever known tag files actually exist (`bagit.txt`, `bag-info.txt`, `capsule.yaml`, `privacy-overrides.yaml` if present, everything under `legal/` if present), and marks `capsule.yaml`'s `status.lifecycle_stage` as `sealing`.

`ptc seal` refuses to run (non-zero exit, clear error) if `access.default_release_policy` is still `ptc init`'s placeholder — edit `capsule.yaml` first. See `FORMAT.md`'s `access` section for the real release policy types.

**A deliberate limitation**: updating `status.lifecycle_stage` patches just that one line of `capsule.yaml` in place, rather than re-serializing the whole file — a full re-serialize would silently strip any comments a creator (or `ptc init`) added. This means `ptc seal` can't currently update any *other* field in `capsule.yaml`; if that's ever needed, it'll need the same careful, targeted approach, not a blanket rewrite.

## Known limitations / next steps

- **Non-interactive only.** Both commands take everything via flags/positional args. A future version should walk a non-technical creator through the same choices interactively — the target audience for this whole project is explicitly *not* assumed to be technical.
- No `--force` flag to reinitialize `ptc init` into a non-empty directory (intentional, for now — safer default).
- **Structural, not full JSON Schema, validation.** `ptc init`/`ptc seal` both read and write `capsule.yaml` through real `serde` types (`ptc::model`) that mirror `schema/capsule.schema.json`'s required fields, types, and enum values, with `deny_unknown_fields` rejecting anything the schema wouldn't allow — a real improvement over the original hand-built-string approach, which validated nothing at all. What's still missing: the schema's *conditional* requirements (e.g. `release_policy.date` being required specifically when `type: fixed_date`) aren't enforced by the Rust types alone. A future `ptc verify` running an actual JSON Schema validator against the parsed YAML would be the authoritative check.
- `ptc seal` doesn't yet handle encrypted collections specially (that's `ptc encrypt`, not built yet) — it hashes whatever is on disk under `data/` at the time it's run.
- `ptc import` only sets a collection's `name`/`path` when auto-registering it — privacy zone, release policy, and encryption settings are left as the creator's own edit, same as `ptc init` leaving the release policy as a placeholder.
- No `ptc inspect`, `ptc scan`, `ptc redact`, `ptc encrypt`, `ptc verify`, `ptc unlock`, or `ptc deposit` yet (`ROADMAP.md` items 23–29).
