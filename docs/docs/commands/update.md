---
id: update
title: odyn update
description: Re-pin a dependency to its latest commit
sidebar_position: 5
slug: /commands/update
---

Fetches the latest commit from a dependency's source and re-pins it in `Odyn.lock`.

## Usage

```sh
odyn update <name>
```

`<name>` is the dependency's name as it appears in `Odyn.lock`.

## What it does

`update` runs a `git fetch` followed by a hard reset to `FETCH_HEAD` inside `odyn_deps/<name>`, then records the new HEAD commit in `Odyn.lock`. Only the named dependency is affected.

If the dependency was originally cloned with `--depth` (shallow clone), `update` automatically runs `git fetch --unshallow` first to retrieve the full history before fetching the latest commits.

## Examples

```sh
odyn update math
```

After running this, `Odyn.lock` will reflect the new commit hash, and `odyn_deps/math` will be at that commit.

## Notes

- `update` errors if `<name>` is not found in `Odyn.lock`.
- There's no "update all" flag. Run `update` once per dependency you want to advance.
- To move to a specific commit rather than the latest HEAD, use [`odyn get`](./get) with `--commit` after removing the dependency, or edit `Odyn.lock` directly and run `odyn sync --force`.

:::tip
Commit the updated `Odyn.lock` after running `odyn update` so collaborators get the same version on their next `odyn sync`.
:::
