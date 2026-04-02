---
id: status
title: odyn status
description: Check the state of all vendored dependencies
sidebar_position: 6
slug: /commands/status
---

Reports the state of every dependency listed in `Odyn.lock` against what's on disk in `odyn_deps/`.

## Usage

```sh
odyn status
```

No flags.

## Output

For each dependency, `status` prints one of the following:

| Status | Meaning |
|---|---|
| `Ok` | Present in `odyn_deps/` and at the exact pinned commit. |
| `Missing` | The `odyn_deps/<name>` folder does not exist. |
| `Modified` | Present but at a different commit than what `Odyn.lock` specifies. |

`status` exits with a non-zero code if any dependency is missing or modified.

## Examples

```sh
odyn status
```

Example output when everything is in order:

```
          Ok 'math' at a1b2c3d
          Ok 'json' at f4e5d6c
```

Example output when a dependency is out of sync:

```
     Missing 'math'
    Modified 'json': expected f4e5d6c but found 9a8b7c6
       Error some dependencies are missing or modified
```

Example integration in CI:

```yml
jobs:
  check:
    runs-on: runner
    steps:
      - name: Install Odyn
        run: curl -fsSL https://codeberg.org/razkar/odyn/raw/branch/main/install.sh | sh
        
      - name: Check dependencies
        run: odyn status
      # ...
```

## Notes

- If `Odyn.lock` has no entries, `status` prints an info message and exits cleanly.
- Run `odyn sync` to restore missing or modified dependencies to their pinned state.
