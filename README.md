<div align="center">

<img src="banner.svg" alt="Odyn" width="80%"/>

### Reproducible vendoring for Odin. Not a package manager.

[![License MIT](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue?style=flat-square)](https://codeberg.org/razkar/odyn/src/branch/main/LICENSE-MIT)
[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Codeberg](https://img.shields.io/badge/codeberg-razkar%2Fodyn-blue?style=flat-square&logo=codeberg)](https://codeberg.org/razkar/odyn)
[![Last Commit](https://img.shields.io/gitea/last-commit/razkar/odyn?gitea_url=https%3A%2F%2Fcodeberg.org&style=flat-square)](https://codeberg.org/razkar/odyn/commits/branch/main)

</div>

---

Odyn clones Git repos into `odyn_deps/` and writes the commit hash to `Odyn.lock`. That's it. You could do this yourself with a spreadsheet and a free afternoon, and Odyn knows it.

No registry. No account. No transitive resolution happening somewhere you can't see.

## Quick Start

```sh
odyn init myproject
cd myproject

odyn get odin-community/math
# or: odyn get razkar/farben --platform codeberg

odyn sync
```

`odyn init` gives you a working project layout with `ols.json` already configured, so your editor's autocomplete works out of the box. `odyn get` clones and pins. `odyn sync` makes everything match the lockfile, on any machine, every time.

## Installation

### Build From Source

Requires Rust 1.85.0 or newer.

```sh
git clone https://codeberg.org/razkar/odyn.git
cd odyn
cargo build --release
```

Binary lands at `target/release/odyn`. Put it on your `PATH`.

Prebuilt binaries are now available in the Releases tab. Refer [here](#codeberg-releases).

### Using Cargo

Cargo is the official build system and package manager for the Rust programming language.
Since Odyn is available at [crates.io](https://crates.io), the central package registry, 
you can simply run the following command if you have `cargo` installed.

```sh
cargo install odyn
```

Put the result in your `PATH`

### Codeberg Releases

By the time you're reading this, [the Codeberg repository](https://codeberg.org/razkar/odyn/releases) has probably posted a Release. Install the binary that fits your system there, and put it in your `PATH`. 
Here are the supported platforms on the manual binary, if not from source:

| Operating System | Architecture |             Status            |
|------------------|--------------|-------------------------------|
| **Windows**      | x86_64/AMD64 | :white_check_mark: Supported  |
|                  | i686 (32-bit)| :white_check_mark: Supported  |
|                  | aarch64      | :soon: Maybe Soon             |
| **macOS**        | aarch64      | :soon: Coming Soon            |
|                  | x86_64       | :soon: Coming Soon            |
| **Linux**        | x86_64       | :white_check_mark: Supported  |
|                  | x86_64 musl  | :white_check_mark: Supported  |
|                  | aarch64      | :white_check_mark: Supported  |
|                  | i686         | :white_check_mark: Supported  |
|                  | RISC-V 64    | :white_check_mark: Supported  |
| *FreeBSD*        | x86_64       | :white_check_mark: Supported  |
|                  | aarch64      | :x: Not Available             |
|                  | i686         | :white_check_mark: Supported  |
| *Others*         | Others       | Build from source / Use Cargo |

> [!NOTE]
> If you're on Windows but unsure, download `x86_64` (or check your system). Apple Silicon (M1/M2/M3 and friends) download `aarch64`,
> Mac Intel users download `x86_64`. If you're on Linux, you probably know which architecture you have, just note that it supports
> all distros.

Sorry for the lack of macOS support! The next release will partly use GitHub Actions which has native macOS runners, so both targets are coming in 0.2.0 and above.

For Windows ARM PCs, I'm currently waiting on GitHub Actions to add runner support there. Maybe soon. 

## Commands

Commands marked :white_check_mark: are complete and stable.

| Command | Description |
|---|---|
| :white_check_mark: `odyn init <name>` | New Odin project with `src/`, `odyn_deps/`, `ols.json`, and an empty `Odyn.lock` |
| :white_check_mark: `odyn get <source> [name]` | Clone a dependency and pin its commit. Accepts `user/repo` shorthand or a full URL |
| :white_check_mark: `odyn sync` | Make `odyn_deps/` match `Odyn.lock`. Re-clones missing deps, errors on modified ones |
| :white_check_mark: `odyn status` | Report every dependency as ok, missing, or modified. Exits non-zero if anything is wrong |
| :white_check_mark: `odyn update <name>` | Pull the latest commit for a dependency and re-pin it |
| :white_check_mark: `odyn remove <name>` | Delete the folder and remove the entry from `Odyn.lock` |
| `odyn update-self` | Update Odyn itself *(coming soon)* |

### `odyn init` flags

| Flag | Description |
|---|---|
| `--license <type>` | License file to generate. Defaults to `mit`, also accepts `apache`, `gpl3`, `bsd2`, `bsd3`, `mpl2`, `unlicense`, `zlib`, and `isc` |
| `--with-readme` | Add a `README.md` stub |
| `--no-src` | Skip the `src/` directory |

### `odyn get` flags

| Flag | Description |
|---|---|
| `--platform <name>` | Platform to resolve `user/repo` against. Defaults to `github`. Supports `github`, `codeberg`, `gitlab`, `sourcehut`, `bitbucket`, `framagit`, `disroot`, `notabug`, and `savannah` |

## The Lockfile

```toml
# This file is automatically @generated by Odyn.
# Do not edit this file manually unless you know what you're doing.

[[dep]]
name = "math"
source = "https://github.com/odin-community/math"
commit = "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2"
```

Commit `Odyn.lock`. That's what makes `odyn sync` reproduce the exact state on another machine. Gitignore `odyn_deps/` if you want, or commit it too. Either works. The lockfile is the important part.

## Importing Dependencies

`odyn init` configures `ols.json` to register `odyn_deps/` as a collection called `deps`, so this works from any file in your project:

```odin
import "deps:math"
```

Pass the collection to the compiler when building:

```sh
odin run src -collection:deps=odyn_deps
```

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a full history of changes.

## License

Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.

Cheers, RazkarStudio.  
© 2026 RazkarStudio. All rights reserved.
