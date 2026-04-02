---
slug: context-aware-outputs
title: Context-Aware Outputs Are Cool, Actually
authors: razkar
tags: [cli, dx, context-aware]
---

A program that outputs something is cool. But you know something even cooler?
A program that outputs something different based on the context. This trait is called *context-aware*,
and in this case, I like to call it a *"context-aware output"* for an output that has this trait.

<!-- truncate -->

## What Does "Context-Aware" Mean?

A context-aware output is one that adapts to *who* is running it and *how*.

The most common example you've probably already seen is color: a program that prints colored text when
you're in a terminal, but plain text when piped into a file. That's context-awareness, the program
knows it's being redirected and adjusts accordingly.

But context doesn't have to mean "is my output a TTY?" It can mean anything the program knows about
its environment: the OS, the architecture, how it was installed, whether it's a nightly build, etc. The
point is that the output becomes *meaningful to the specific person running it*.

Certain things are better context-aware, but not all of them should be if they slow things down. Too many context 
awareness is also scary, and is leaning towards telemetry, a fancy word for surveillance, which is not cool.

## Odyn's `--version` Flag

Most CLI tools have a `--version` flag. Most of them output something like:

```
mytool 1.2.3
```

That's fine, it does what you expect it to do, which is outputting the version. 
It's functional, but often forgettable.

But Odyn's `--version` does something a little different. Try it yourself, the output changes depending
on *how Odyn was installed*. Let me elaborate:

**From a release binary** (the recommended install path):
```
Odyn v0.3.0 Linux x86_64
    Reproducible vendoring tool for the Odin programming language.
```
The OS and architecture are shown in color, specific to *your* platform. Linux is yellow, Windows is
blue, macOS is gray, Android is green, FreeBSD is red, NetBSD is orange. You get a quick visual
confirmation that you have the right binary for your machine.

**From `cargo install odyn`**:
```
Odyn v0.3.0 Cargo Edition
    Reproducible vendoring tool for the Odin programming language.
```
You know you're running the crates.io release, compiled for your local machine.

**Built from source**:
```
Odyn v0.3.0 Nightly, commit a1b2c3d
    Reproducible vendoring tool for the Odin programming language.
```
You get the short commit hash, handy when reporting bugs or checking if you're on the latest main.

## How It Works

This is all handled at **compile time**, not at runtime. Odyn's `build.rs` sets an
`ODYN_INSTALL_METHOD` environment variable when the binary is compiled:

- If `ODYN_INSTALL_METHOD` is set in the environment (e.g. by the release CI), it's used as-is.
- If a `.git` directory exists, it's assumed to be a source build, and `git rev-parse --short HEAD`
  is called to capture the commit hash.
- Otherwise, it falls back to `"cargo"`.

The version command then reads this baked-in value with Rust's `env!()` macro, and branches on it.
In this case, there's no runtime overhead nor any detection logic. The binary just *knows* how it was born.

```rust
let extra = match env!("ODYN_INSTALL_METHOD") {
    "cargo"   => "Cargo Edition".to_string(),
    "source"  => format!("Nightly, commit {}", env!("ODYN_GIT_HASH")),
    "release" => format!("{} {}", std::env::consts::OS, std::env::consts::ARCH),
    _         => "".to_string(),
};
```

*(Simplified slightly for clarity.)*

## Why Bother?

> Why do you really need a `--version` flag, of all features, to be context-aware in the first place?

Because when something breaks and someone pastes their `--version` output into an issue, you
immediately know whether they're on a release binary for the right platform, or if they accidentally
compiled a source build from a week-old commit.

It's a small thing. But small things add up, and they're the difference between a tool that feels
polished and one that just works.

Context-aware outputs are cool. Now you know why.
