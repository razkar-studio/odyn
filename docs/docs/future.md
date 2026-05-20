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
- [x] `update-self`: `--pre-release` queries the latest release of Odyn regardless of stability
- [x] `update-self`: `--nightly` queries the latest *commit* of Odyn

## Additions
- [ ] `run`: resolves to `odin run src -collections:deps=odyn_deps` where `src` could be `.` depending if it exists or not
  + [ ] `run`: needs odin compiler though?
- [x] `init`: `--migrate`, adds an `ols.json` entry for `deps`, an empty `Odyn.lock`, and an empty `odyn_deps/` directory in an existing project.
- [x] `--version`: why haven't this been added??? (update: it's added)
  + [x] `--version`: (optionally) platform-specific, (and maybe) install-method-specific message
- [ ] `install`: clones a provided repository and `odin build .` (or smartly chooses a directory with an `.odin` file), and puts the binary inside `~/.odyn/bin` which prompted to be put in `PATH`.
  + [ ] `install --bin`: fetches any pre-built binaries on the provided repository and installs that directly
  + needs odin compiler
- [ ] `search`: Searches Odin packages from a "list of lists," configurable list databases. Still needs to manually install though

## Soon
- [ ] caching (less accurate term, i think it's a memo?), storing metadata of `get` and duplicating local paths instead of cloning when metadata matches
  + more details, the `get`-ted repos are automatically logged in a mini database (in `~/.odyn/memo`) with metadatas such as the repo name, commit hash, *local directory path*, all data needed for Odyn to know exactly what to clone
  + then, when `get` gets called, it searches the log and if it finds a match, it searches for the local directory path, validates it, and then clones the directory instead of using `git clone`.
  + this makes `get`ting often used repos uses local I/O speed instead of network + I/O speed, which is generally, significantly faster.
  + alternatively, instead of storing local paths (which is weak and hard to validate), we can clone the repos directly in something like `~/.odyn/cache/*`. the pros and cons are very visible.

## Probably
- [ ] `odin [--version]`: installs the odin compiler on your machine (probably not?)
