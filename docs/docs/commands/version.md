---
id: version
title: odyn version
description: Print version and environment information
sidebar_position: 8
slug: /commands/version
---

Prints the current Odyn version, install method, and detected git version.

## Usage

```sh
odyn version [--verbose]
odyn --version [--verbose]
```

Both forms are identical. `--version` is the traditional flag form; `version` is the subcommand form.

## Output

```
Odyn vX.X.X Linux x86_64
    Reproducible vendoring tool for the Odin programming language.
    git version 2.49.0
```

The second line after the version heading is color-coded by install method:

| Install method | Label shown |
|---|---|
| Release binary | OS and architecture (e.g. `Linux x86_64`) |
| `cargo install` | `Cargo Edition` |
| Built from source | `Nightly, commit <hash>` |

The git version line is shown in orange if git is on your PATH, or in red as **Git Not Installed** if it isn't.

## Options

| Flag | Description |
|---|---|
| `--verbose` | Print install path and build date below the standard output |

## Verbose output

```sh
odyn version --verbose
```

```
Odyn vX.X.X Linux x86_64
    Reproducible vendoring tool for the Odin programming language.
    git version 2.49.0
    installed at /usr/local/bin/odyn
    built on 2026-04-02
```

The `installed at` path is the real on-disk location of the running binary (symlinks resolved). The `built on` date is baked into the binary at compile time.
