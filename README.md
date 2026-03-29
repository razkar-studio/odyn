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

It has no registry, no account, nor any solver. No transitive dependencies filling everything up without you knowing.

Before you ask anything or make any complaints: [FaQs](#frequently-asked-questions)!

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

### Pre-requisites

For Odyn to function properly, the requirements are the following:

* Git in your `PATH`, or beside the Odyn binary

Yep, that's it.

### Install Script

Most simple and straightforward way.

**Linux/macOS/FreeBSD/NetBSD/Android:**
```sh
curl -fsSL https://codeberg.org/razkar/odyn/raw/branch/main/install.sh | sh
```

or with `wget`:
```sh
wget -qO- https://codeberg.org/razkar/odyn/raw/branch/main/install.sh | sh
```

**Windows (PowerShell):**
```ps1
irm https://codeberg.org/razkar/odyn/raw/branch/main/install.ps1 | iex
```

> [!NOTE]
> On Windows, you may need to allow script execution first:
> ```powershell
> Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
> ```

### Codeberg Releases

[The Codeberg repository](https://codeberg.org/razkar/odyn/releases) has pre-built binaries for Windows, macOS, Linux, Android (via Termux), FreeBSD, and NetBSD. Install the binary that fits your system and put it in your `PATH`.

The full architecture table can be found in [MORE.md](./MORE.md).

> [!IMPORTANT]
> Some platforms may not support `update-self`. For those, install binaries manually from the Releases page.
> Examples: every musl variant, powerpc64 (defaults to LE), ARMv6 (defaults to ARMv7).
> Always prefer installing from Releases if your system is not `update-self` supported.

> [!NOTE]
> If you're on Windows but unsure, download `x86_64`. Apple Silicon (M1/M2/M3) users download `aarch64`,
> Intel Mac users download `x86_64`. On Linux, you probably know your architecture, just note it works on all distros.

### Build From Source

Requires Rust 1.85.0 or newer.

```sh
git clone https://codeberg.org/razkar/odyn.git
cd odyn
cargo build --release
```

Binary lands at `target/release/odyn`. Put it on your `PATH`.

### Using Cargo

```sh
cargo install odyn
```

## Commands

Commands marked ✅ are complete and stable.

| Command | Description |
|---|---|
| ✅ `odyn init <n>` | New Odin project with `src/`, `odyn_deps/`, `ols.json`, and an empty `Odyn.lock` |
| ✅ `odyn get <source> [name]` | Clone a dependency and pin its commit. Accepts `user/repo` shorthand or a full URL |
| ✅ `odyn sync` | Make `odyn_deps/` match `Odyn.lock`. Re-clones missing deps, errors on modified ones |
| ✅ `odyn status` | Report every dependency as ok, missing, or modified. Exits non-zero if anything is wrong |
| ✅ `odyn update <n>` | Pull the latest commit for a dependency and re-pin it |
| ✅ `odyn remove <n>` | Delete the folder and remove the entry from `Odyn.lock` |
| ✅ `odyn update-self` | Update Odyn itself to the latest release |

### `odyn init` flags

| Flag | Description |
|---|---|
| `--license <type>` | License file to generate. Defaults to `mit`. Also accepts `apache`, `gpl3`, `bsd2`, `bsd3`, `mpl2`, `unlicense`, `zlib`, `isc` |
| `--with-readme` | Add a `README.md` stub |
| `--no-src` | Skip the `src/` directory |

### `odyn get` flags

| Flag | Description |
|---|---|
| `--platform <n>` | Platform to resolve `user/repo` against. Defaults to `github`. Supports `github`, `codeberg`, `gitlab`, `sourcehut`, `bitbucket`, `framagit`, `disroot`, `notabug`, `savannah` |
| `--commit <hash>` | Pin a specific commit instead of HEAD |

### `odyn sync` flags

| Flag | Description |
|---|---|
| `--force` | Reset locally modified dependencies to their pinned commits instead of erroring |
| `--skip <n>` | Skip a specific dependency entirely. Chainable |

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

## Frequently Asked Questions

### Q. Why Rust?
Rust makes cross-compilation easy with `cross`, which allows Odyn to have a wide binary matrix.
Pre-built binaries are the recommended install path for Odyn, and having Odyn support more platforms is never a negative.

### Q. Is Odyn A Package Manager?
**No, and it never will be.** According to *gingerBill*, the creator of the Odin programming language,
package managers are evil because according [to his article](https://www.gingerbill.org/article/2025/09/08/package-managers-are-evil/), it does numerous things:

1. Automating dependencies: automating dependency hell wouldn't get rid of the *hell*, but instead would accelerate the problem instead of solving it.
2. Package managers define what a package *is*, which leads to competing definitions, which would lead to *package-manager-managers*, as he cited.
3. ...and more. I recommend reading his article about it.

In it, he mentioned that
> "Copying and vendoring each package manually, and fixing the specific versions down is the most practical approach."

Which is **exactly** what Odyn automates, and nothing more.

### Q. So What's Odyn Then?
Odyn is a *reproducible vendoring tool* for the Odin programming language. Not a package manager.

### Q. Why Not Git Submodules?
Because git submodules are famously unpleasant to work with, and the happy path is a lie.
The happy path looks simple, but the moment you step off the happy path, submodules start biting you. A few real scenarios:

* Someone clones your repo without `--recurse-submodules`: they get empty directories and no obvious error message. This happens constantly to people who don't know or forget the flag.
  + Compared to Odyn: The only thing someone needs to do after cloning is just `odyn sync` in the root. No flags.
* You want to update a dependency: it's not just `git pull` inside the submodule directory, it's `git submodule update --remote --merge`, and then you have to commit the updated pointer in the parent repo separately. Easy to get wrong, easy to leave in a detached HEAD state.
  + Compared to Odyn: Just run `odyn update <dep>` and it works.
* You delete a submodule: there's no `git submodule remove`. You have to manually edit `.gitmodules`, `.git/config`, run `git rm --cached`, delete the directory, and commit. Four steps to remove a submodule.
  + Compared to Odyn: Run `odyn remove <dep>`. One step.
* A collaborator runs `git pull` on the parent: the submodule pointer updates but the actual submodule content doesn't unless they also run `git submodule update`. Now their build is broken and they don't know why.
  + Compared to Odyn: `odyn sync` again.

If I liked using git submodules, Odyn wouldn't exist. Let that sink in.

That being said, if you're comfortable with git submodules, just use it. There's no one forcing you to use Odyn, whatever thing works for you, just use it.

### Q. Who Are You?
Uhh, what?

### Q. Will Odyn Be Getting An Odin Rewrite?
Maybe, maybe not. It's not planned, but it's not off the list.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a full history of changes.

## License

Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.

Cheers, RazkarStudio.  
© 2026 RazkarStudio. All rights reserved.
