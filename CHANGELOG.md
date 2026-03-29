# Changelog

Odyn is constantly updating. All notable changes to it is documented here.

## [0.2.0] - Soon

The More The Merrier!

### Added

- `odyn update-self` — now fully implemented. Detects your platform, fetches the latest release from Codeberg, verifies SHA256, and replaces the current binary. Supports all shipped platforms.
  - SHA256 verification against `SHA256SUMS` (or `SHA256SUMS-macos` on macOS)
  - Handles Windows quirk of not being able to overwrite a running binary
- `odyn sync --force` — resets locally modified dependencies back to their pinned commits instead of erroring
- `odyn sync --skip <name>` — skips a specific dependency entirely during sync. Chainable.
- `odyn get --commit <hash>` — pin a specific commit instead of HEAD. No more touching `Odyn.lock` by hand.
- `install.sh` — install script for Linux, macOS, FreeBSD, NetBSD, and Android. Tries `curl` first, falls back to `wget`. Verifies SHA256. Installs to `~/.local/bin`.
- `install.ps1` — install script for Windows. Uses built-in PowerShell tools. Automatically adds to PATH.
- SHA256SUMS and SHA256SUMS-macos files on every release
- Forgejo Actions CI — automatic multi-platform builds on every `v*` tag
- GitHub Actions CI — automatic macOS builds (x86_64 and aarch64) uploaded to Codeberg releases

### Platform Support (up from 9 to ~25 binaries)

- Linux: added aarch64 musl, i686 musl, ARMv7, ARMv7 musl, ARMv6, ARMv6 musl, POWERPC64, POWERPC64LE, s390x, SPARC64
- Android (via Termux): aarch64, ARMv7, x86_64
- NetBSD: x86_64
- macOS: x86_64 (Intel), aarch64 (Apple Silicon)


## [0.1.0] - 2026-03-27

Initial release!

### Added

- `odyn init <name>` — scaffold a new Odin project with `src/`, `odyn_deps/`, `ols.json`, and an empty `Odyn.lock`
  - `--license <type>` — generate a license file. Supports `mit`, `apache`, `gpl3`, `bsd2`, `bsd3`, `mpl2`, `unlicense`, `zlib`, `isc`
  - `--with-readme` — add a `README.md` stub
  - `--no-src` — skip the `src/` directory
- `odyn get <source> [name]` — clone a dependency and pin its commit in `Odyn.lock`
  - Accepts `user/repo` shorthand or a full URL
  - `--platform <name>` — resolve `user/repo` against a specific platform. Supports `github` (default), `codeberg`, `gitlab`, `sourcehut`, `bitbucket`, `framagit`, `disroot`, `notabug`, `savannah`
  - Local paths are explicitly rejected to keep lockfiles portable
- `odyn sync` — make `odyn_deps/` match `Odyn.lock` exactly. Re-clones missing deps, errors on modified ones
- `odyn status` — report every dependency as ok, missing, or modified. Exits non-zero if anything is wrong
- `odyn update <name>` — pull the latest commit for a dependency and re-pin it in `Odyn.lock`
- `odyn remove <name>` — delete the folder and remove the entry from `Odyn.lock`
- `odyn update-self` — stub. Download the latest binary from the Releases page for now
- `Odyn.lock` — TOML lockfile format pinning each dependency to an exact commit
- `ols.json` generation with the `deps:` collection pre-configured
