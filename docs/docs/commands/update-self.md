---
id: update-self
title: odyn update-self
description: Update the Odyn binary to the latest release
sidebar_position: 7
slug: /commands/update-self
---

Downloads the latest Odyn release from Codeberg and replaces the current binary in place.

## Usage

```sh
odyn update-self
```

No flags.

## What it does

1. Queries the Codeberg API for the latest release tag.
2. If the current version is already up to date (or newer), exits without downloading anything.
3. Downloads the correct binary for your OS and architecture.
4. Fetches the release's `SHA256SUMS` file and verifies the download matches.
5. Replaces the running `odyn` binary with the downloaded one.

## Supported platforms

| OS | Architectures |
|---|---|
| Linux | x86_64, aarch64, i686, riscv64, armv7, powerpc64le, s390x, sparc64 |
| Windows | x86_64, i686 |
| macOS | x86_64, aarch64 |
| Android | x86_64, aarch64, armv7 |
| FreeBSD | x86_64, i686 |
| NetBSD | x86_64 |

If your platform isn't listed, `update-self` exits with an error pointing you to the [releases page](https://codeberg.org/razkar/odyn/releases), Cargo, or building from source.

## Notes

- The downloaded binary is verified against SHA256SUMS before being installed. If the checksum doesn't match, the download is deleted and the command errors without touching your current binary.
- On Windows, the running binary is renamed to `.exe.old` before the new one is written, and cleaned up afterwards.
- After updating, restart your shell or re-invoke `odyn` for the new version to take effect.
- Requires an internet connection to reach the Codeberg API and release assets.
