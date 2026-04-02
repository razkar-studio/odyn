---
id: getting-started
title: Getting Started
description: Your first project with Odyn
sidebar_position: 3
slug: /getting-started
---

Let's start using Odyn. Odyn's workflow is intentionally minimal. This page walks you through everything from project creation to importing a dependency in your Odin code.

## Checking Odyn's Existence

First, let's check if you actually installed Odyn properly.

```sh
odyn --version
```

This outputs the version of Odyn. Unlike most `--version` commands, Odyn is context-aware, which means it's different based on your machine. For example, if you installed on a Windows x86_64 machine, it would output the following:

```
Odyn vX.X.X Windows x86_64
    Reproducible vendoring tool for the Odin programming language.
```

*Where `X.X.X` is your actual Odyn version.*

<details>

<summary>More Outputs</summary>

Showcasing Odyn's context-aware version command.

Generic:
```
Odyn vX.X.X <os> <arch>
    Reproducible vendoring tool for the Odin programming language.
```
*Where `<os>` is your operating system (e.g Android, macOS, Linux) and `<arch>` is your architecture (e.g. x86_64, aarch64)*

Installed from source:
```
Odyn vX.X.X Nightly, commit <hash>
    Reproducible vendoring tool for the Odin programming language.
```
*Where `<hash>` is the git commit hash when you cloned from source.*

Installed using Cargo:
```
Odyn vX.X.X Cargo Edition
    Reproducible vendoring tool for the Odin programming language.
```

</details>

If that works, then Odyn exists. You're good to go!

<details>

<summary>Advanced</summary>

Party trick, you can trick Odyn's version output by setting an `ODYN_INSTALL_METHOD` environment variable.
[The code](https://codeberg.org/razkar/odyn) checks the value of `ODYN_INSTALL_METHOD` and creates the output, it could either be:
- `release`, which would check for your operating system and architecture.
- `source`, which would output Nightly and the commit hash it was installed on.
  + The hash is another environment variable, `ODYN_GIT_HASH`. From my testing, if you were already using this method, you can't modify the hash output by using the environment variable, for some reason.
- `cargo`, outputs "Cargo Edition"

All of the environment variables was actually set by Odyn under the hood, so you don't need to worry about setting an
environment variable everytime you run `--version`. This trick is just editing the already set environment variables to your liking.

</details>

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
