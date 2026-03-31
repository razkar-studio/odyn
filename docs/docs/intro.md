---
id: intro
title: Introduction
description: What is Odyn and what problem does it solve?
sidebar_position: 1
slug: /intro
---

## What is Odyn?

Odyn is a *reproducible vendoring tool* for the Odin programming language. Not a package manager.

At the core, Odyn clones Git repos into `odyn_deps/` and writes the commit hash to `Odyn.lock`. That's it. You could theoretically do this yourself with a spreadsheet and a free afternoon.

It has no registry, no account, nor any solver. No transitive dependencies filling everything up without you knowing. What goes in is entirely your call, Odyn just makes sure it's pinned and reproducible on any machine.

## What Odyn Is Not

Odyn is not a package manager, and it never will be. It automates exactly one thing: reproducible vendoring.

:::info
Odyn disallows *local git repos* by nature, since it prevents reproducibility across other machines.
It's better to commit your local dependencies instead.
:::

## Quick Start
```sh
odyn init myproject
cd myproject

odyn get odin-community/math
odyn sync
```

`odyn get` clones and pins the commit hash to `Odyn.lock`. `odyn sync` makes everything match that lockfile on other machines.

Let's get to installing Odyn on your machine. Don't worry, it covers more platforms than you think.
