---
id: getting-started
title: Getting Started
description: Your first project with Odyn
sidebar_position: 3
slug: /getting-started
---

Let's start using Odyn. Odyn's workflow is intentionally minimal. This page walks you through everything from project creation to importing a dependency in your Odin code.

## Initialize a Project
```sh
odyn init myproject
cd myproject
```

This creates a project layout with `src/`, `odyn_deps/`, a pre-configured `ols.json`, and an empty `Odyn.lock`.

:::tip
`ols.json` is configured out of the box to register `odyn_deps/` as a collection called `deps`, so your editor's autocomplete works immediately without any extra setup. Note that you will need to pass the `deps` collection to the compiler when building, we'll get to that below.
:::

## Add a Dependency
```sh
odyn get odin-community/math
```

This clones the repository into `odyn_deps/` and pins its current commit hash to `Odyn.lock`. By default, `odyn get` resolves shorthands against GitHub, so the example above resolves to something like `https://github.com/odin-community/math`. To use another platform:
```sh
odyn get bergberg/mathberg --platform codeberg
```

That resolves it with Codeberg instead, into `https://codeberg.org/bergberg/mathberg`.

## Sync
```sh
odyn sync
```

`odyn sync` makes `odyn_deps/` match exactly what's in `Odyn.lock`. Run this after cloning a project, pulling changes from a collaborator, or when you recursively remove `odyn_deps/`. By accident, of course.

## Import in Odin

The `ols.json` that Odyn set up wouldn't even work if you didn't pass the `deps` collection when building, pass it like so:
```sh
odin run src -collection:deps=odyn_deps
```

Then import any vendored dependency from any file in your project, using `deps:`
```odin
import "deps:math"
```

## What to Commit

**Commit `Odyn.lock`,** this is what makes `odyn sync` reproduce the exact state on any machine.

`odyn_deps/` can be safely gitignored, it's fully reproducible from the lockfile. That said, committing it too is valid if you prefer everything self-contained in the repo. I recommend committing `odyn_deps/` in the case that a repo that the lockfile is pointing to is made inaccessible.
