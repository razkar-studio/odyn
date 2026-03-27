# Changelog

Odyn is constantly updating. All notable changes to it is documented here.

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
