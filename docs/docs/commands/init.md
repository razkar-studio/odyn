---
id: init
title: odyn init
description: Scaffold a new Odin project with Odyn support
sidebar_position: 1
slug: /commands/init
---

Scaffolds a new Odin project directory with a standard layout and Odyn pre-configured.

## Usage

```sh
odyn init <project-name> [options]
```

## What it creates

Running `odyn init myproject` produces the following:

```
myproject/
  src/
    main.odin     # "Hellope, myproject!" starter
  odyn_deps/      # empty, ready for dependencies
  Odyn.lock       # empty lockfile
  ols.json        # registers odyn_deps/ as the `deps` collection
  LICENSE         # MIT by default
```

The generated `ols.json` registers `odyn_deps/` as the `deps` collection, so your editor's language server picks it up immediately. You still need to pass the collection flag when building:

```sh
odin run src -collection:deps=odyn_deps
```

## Options

|        Flag         | Default | Description |
|---------------------|---------|-------------|
| `--license <name>`  | `mit`   | License to write to `LICENSE`. See [Licenses](#licenses) below. |
| `--with-readme`     | off     | Creates a `README.md` stub at the project root. |
| `--no-src`          | off     | Skips creating `src/` and places `main.odin` at the project root instead. |

## Examples

```sh
# Basic project
odyn init myproject

# With Apache license and a README
odyn init myproject --license apache --with-readme

# Flat layout (no src/ directory)
odyn init myproject --no-src

# Unlicensed
odyn init myproject --license unlicense
```

## Licenses

The `--license` flag accepts the following values:

|     Value   | License |
|-------------|---------|
| `mit`       | MIT License |
| `apache`    | Apache License 2.0 |
| `gpl3`      | GNU General Public License v3.0 |
| `bsd2`      | BSD 2-Clause License |
| `bsd3`      | BSD 3-Clause License |
| `mpl2`      | Mozilla Public License 2.0 |
| `unlicense` | The Unlicense |
| `zlib`      | zlib License |
| `isc`       | ISC License |

Passing any other string writes a plain `License: <value>` file instead of erroring out.

## Notes

- `init` fails if the target directory already exists.
- The project name is used verbatim as the directory name and inside `main.odin`'s package name.
