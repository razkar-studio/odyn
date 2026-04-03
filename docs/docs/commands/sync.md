---
id: sync
title: odyn sync
description: Restore odyn_deps/ from Odyn.lock
sidebar_position: 3
slug: /commands/sync
---

Restores `odyn_deps/` to exactly match `Odyn.lock`. Run this after cloning a project, pulling changes from a collaborator, or after deleting `odyn_deps/`.

## Usage

```sh
odyn sync [options]
```

## Options

| Flag | Default | Description |
|---|---|---|
| `--force` | off | Revert locally modified dependencies instead of aborting. |
| `--skip <name>` | — | Skip a specific dependency. Can be passed multiple times. |

## What it does

For each entry in `Odyn.lock`, `sync` checks the state of the corresponding folder in `odyn_deps/`:

- **Present and at the pinned commit:** verified, nothing to do.
- **Present but at a different commit:** treated as modified. Sync aborts unless `--force` is passed.
- **Missing:** cloned from the source URL, then reset to the pinned commit.

If a missing dependency was previously shallow-cloned, `sync` automatically runs `git fetch --unshallow` before resetting to ensure the pinned commit is reachable. If unshallow fails, it attempts a direct fetch of the pinned commit as a fallback.

`sync` is safe to run multiple times. It always produces the same result.

## Examples

```sh
# Standard sync after cloning a project
odyn sync

# Discard local modifications and force a clean state
odyn sync --force

# Sync everything except one dependency
odyn sync --skip math

# Skip multiple dependencies
odyn sync --skip math --skip json
```

## Notes

- `sync` aborts if any dependency in `odyn_deps/` is at a different commit than what `Odyn.lock` specifies, unless `--force` is used.
- `--skip` accepts the dependency's name as it appears in `Odyn.lock`, not its source URL.
- If `Odyn.lock` is empty or missing, `sync` exits cleanly with no changes.

:::warning
`--force` performs a hard reset on modified dependencies. Any local changes inside those folders will be lost.
:::
