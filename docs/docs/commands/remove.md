---
id: remove
title: odyn remove
description: Remove a dependency from your project
sidebar_position: 4
slug: /commands/remove
---

Deletes a dependency's folder from `odyn_deps/` and removes its entry from `Odyn.lock`.

## Usage

```sh
odyn remove <name>
```

`<name>` is the dependency's name as it appears in `Odyn.lock`, which matches the folder name under `odyn_deps/`.

## Examples

```sh
odyn remove math
```

## Notes

- `remove` errors if `<name>` is not found in `Odyn.lock`.
- Only the named dependency is affected. Other dependencies are not touched.
- The `odyn_deps/<name>` directory is deleted recursively.
