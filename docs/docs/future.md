---
id: future
title: The Future
description: Future features for Odyn
sidebar_position: 6
slug: /future
---

A checklist of thoughts for the future of Odyn.

## Changes
- [x] `get`: `--commit`
- [x] `get`: `--depth`
- [x] `get`: `-- ...` where `...` gets sourced to `git clone`
- [ ] `get`: `sync`-ing inside dependencies if `Odyn.lock` exists inside it
- [ ] fixing possible bugs
- [x] adding bugs
- [ ] `update-self`: `--pre-release` queries the latest release of Odyn regardless of stability
- [ ] `update-self`: `--nightly` queries the latest *commit* of Odyn

## Additions
- [ ] `run`: resolves to `odin run src -collections:deps=odyn_deps` where `src` could be `.` depending if it exists or not
  + [ ] `run`: needs odin compiler though?
- [x] `init`: `--migrate`, adds an `ols.json` entry for `deps`, an empty `Odyn.lock`, and an empty `odyn_deps/` directory in an existing project.
- [x] `--version`: why haven't this been added??? (update: it's added)
  + [x] `--version`: (optionally) platform-specific, (and maybe) install-method-specific message

## Probably
- [ ] `odin [--version]`: installs the odin compiler on your machine (probably not?)
- [ ] caching, storing metadata of `get` and duplicating local paths instead of cloning when metadata matches
