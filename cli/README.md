# ptc (reference CLI) — experimental

An early, experimental prototype of the reference CLI described in [`ROADMAP.md`](../ROADMAP.md) items 21–29. Only `ptc init` exists so far. This is Phase 4 (Software) work started early/for-fun, out of the roadmap's normal order — Phases 2 (Specification) and 3 (Legal) are not finished. Nothing here should be treated as authoritative until the underlying spec (`FORMAT.md`, `ROADMAP.md` items 9–15) is further along.

Written in Rust; licensed [Apache-2.0](../LICENSE) like the rest of this project's code/tooling.

## Building

```sh
cargo build
```

### Windows-specific gotcha (GNU target)

If `rustup show` reports your host as `x86_64-pc-windows-gnu` rather than `-msvc`, the linker/`dlltool` bundled with a bare `rustup` install is incomplete for this crate's dependencies (missing `as.exe`, the assembler `dlltool` shells out to internally) — you'll see `dlltool ... CreateProcess` or `dlltool.exe: program not found` errors. Two ways to fix it:

- **Switch to the MSVC target** (simplest, standard on Windows): install Visual Studio Build Tools with the "C++ build tools" workload, then `rustup default stable-x86_64-pc-windows-msvc`.
- **Or install a complete MinGW-w64** (e.g. via `winget install BrechtSanders.WinLibs.POSIX.UCRT`) and point Cargo at it:
  ```sh
  set RUSTFLAGS=-C dlltool=<path-to-mingw64>\bin\dlltool.exe -C linker=<path-to-mingw64>\bin\gcc.exe
  ```
  If your Windows username contains a space, make sure `<path-to-mingw64>` doesn't pass through any directory with a space in it — some GNU binutils/gcc internals mishandle that. A junction/symlink to a space-free path is **not** sufficient (`gcc` resolves its own real path internally); copy the toolchain to a space-free location instead if needed.

This is an environment quirk, not a project dependency — it doesn't affect the MSVC target or Linux/macOS builds.

## Usage

```sh
ptc init <path> [--creator-name NAME] [--creator-contact CONTACT] [--purpose TEXT]
```

Scaffolds a new capsule skeleton at `<path>`: `bagit.txt`, an empty `data/person/` directory, and a starter `capsule.yaml` (validated against [`../schema/capsule.schema.json`](../schema/capsule.schema.json)). All flags are optional — omitted fields (per Principle #2, the creator controls what identifying information appears at all) are simply left out of the generated file rather than filled with placeholder junk.

`access.default_release_policy` is always written as `type: custom` with a `"TBD"` description — `ptc init` has no business inventing a real release policy on a creator's behalf.

This does **not** produce a complete BagIt bag yet: `manifest-sha256.txt`, `tagmanifest-sha256.txt`, and `bag-info.txt` belong to the Sealing lifecycle stage (`LIFECYCLE.md` stage 5), which isn't implemented as a command yet.

## Known limitations / next steps

- **Non-interactive only.** This first pass takes everything via flags. A future version should walk a non-technical creator through the same choices interactively (prompts for creator name, purpose, etc.) rather than requiring them to know the flags — the target audience for this whole project is explicitly *not* assumed to be technical.
- No `--force` flag to reinitialize into a non-empty directory (intentional, for now — safer default).
- No schema validation is performed by the binary itself; it's hand-verified against `schema/capsule.schema.json` during development. A future `ptc verify` should do this at runtime.
