---
id: commands
title: Commands
description: All Odyn commands in one place
sidebar_position: 5
slug: /commands
---

import DocCardList from '@theme/DocCardList';

A reference for every command Odyn ships with.

All commands Odyn provides, one page each.

|       Command       | Description |
|---------------------|-------------|
| `odyn init`         | New Odin project with `src/`, `odyn_deps/`, `ols.json`, and an empty `Odyn.lock` |
| `odyn get`          | Clone a dependency and pin its commit |
| `odyn update`       | Pull the latest commit for a dependency and re-pin it |
| `odyn remove`       | Delete the folder and remove the entry from `Odyn.lock` |
| `odyn sync`         | Make `odyn_deps/` match `Odyn.lock` |
| `odyn status`       | Report every dependency as ok, missing, or modified |
| `odyn update-self`  | Update Odyn itself to the latest release |

All commands that touch dependencies require `git` to be on your `PATH`.

<DocCardList />
