---
id: get
title: odyn get
description: Add a dependency to your project
sidebar_position: 2
slug: /commands/get
---

Clones a Git repository into `odyn_deps/` and pins its commit hash in `Odyn.lock`.

## Usage

```sh
odyn get <source> [name] [options]
```

`<source>` is either a full Git URL or a `user/repo` shorthand. The shorthand resolves to GitHub by default.

## Options

| Flag | Default | Description |
|---|---|---|
| `--platform <name>` | `github` | Platform to resolve `user/repo` shorthands against. See [Platforms](#platforms) below. |
| `--commit <hash>` | HEAD | Pin a specific commit instead of the current HEAD. |
| `--depth <n>` | full | Perform a shallow clone fetching only the last `n` commits. |
| `-- <args>` | — | Extra arguments passed directly to `git clone`. Must come after all other flags. |

## Examples

```sh
# Shorthand resolving to https://github.com/odin-community/math
odyn get odin-community/math

# Full URL
odyn get https://github.com/odin-community/math

# Codeberg shorthand
odyn get bergberg/mathberg --platform codeberg

# Pin a specific commit
odyn get odin-community/math --commit a1b2c69

# Use a custom folder name in odyn_deps/
odyn get odin-community/math mymath

# Shallow clone (last 1 commit only)
odyn get odin-community/math --depth 1

# Pass extra git clone arguments
odyn get odin-community/math -- --filter=blob:none
```

## Platforms

The `--platform` flag accepts the following values:

| Value | Resolves to |
|---|---|
| `github` | `https://github.com` |
| `codeberg` | `https://codeberg.org` |
| `gitlab` | `https://gitlab.com` |
| `sourcehut` / `sr.ht` | `https://git.sr.ht` |
| `bitbucket` | `https://bitbucket.org` |
| `notabug` | `https://notabug.org` |
| `disroot` | `https://git.disroot.org` |
| `framagit` | `https://framagit.org` |
| `savannah` | `https://git.savannah.gnu.org/git` |

`gitea` is not supported as a shorthand platform since it has no single public instance. Pass a full URL instead.

:::info
For any other host, pass the full URL directly and skip `--platform`.
No worries, you rarely need to do this.
:::

## How the name is derived

The subfolder name under `odyn_deps/` is determined in this order:

1. The explicit `[name]` argument, if provided.
2. The last path segment of the URL, with any `.git` suffix stripped.

So `odyn get https://github.com/odin-community/math.git` produces `odyn_deps/math`.

:::tip
For two libraries with the identical name, it is recommended to use the `[name]` argument
to make them different.
:::

## Notes

- Local paths (`./`, `../`, `~/`, `file://`, Windows drive paths) are rejected. Push the repo to a remote and use that URL instead.
- If a dependency with the same source URL already exists in `Odyn.lock`, `get` skips without error.
- If `--commit` points to a hash that doesn't exist in the remote, the partial clone is removed and the command errors.
- `Odyn.lock` is only written after a successful clone. If the write fails, the cloned directory is cleaned up.

:::tip
Commit your `Odyn.lock` after running `odyn get`. Without it, `odyn sync` has nothing to restore from.
:::
