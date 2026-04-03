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
odyn update-self [options]
```

## Options

| Flag | Description |
|---|---|
| `--pre-release` | Update to the latest pre-release instead of the latest stable release |
| `--nightly` | Build and install the latest commit from `main` via `cargo install --git` |
| `--commit <hash>` | Build a specific commit when using `--nightly` instead of `main` HEAD |
| `--force-stable` | Force update to the latest stable release even if the local version is newer |

`--pre-release`, `--nightly`, and `--force-stable` are mutually exclusive. `--commit` requires `--nightly`.

:::note
`--nightly` builds from source and **requires Cargo to be installed** on your system.
It does not download a pre-built binary. The installed binary will identify itself as a nightly build when you run `odyn --version`.
:::

## What it does

### Stable / pre-release

1. Queries the Codeberg API for the target release tag.
2. If the current version is already up to date (or newer), exits without downloading anything. Use `--force-stable` to override this check.
3. Downloads the correct binary for your OS and architecture.
4. Fetches the release's `SHA256SUMS` file and verifies the download matches.
5. Replaces the running `odyn` binary with the downloaded one.

### Nightly (`--nightly`)

1. Fetches the latest commit SHA from the `main` branch (or uses the one provided via `--commit`).
2. Runs `cargo install --git https://codeberg.org/razkar/odyn.git --force --no-default-features --rev <commit> --root <temp>` to build into a temporary directory.
3. Copies the built binary over the currently running `odyn` binary in place, using a safe rename-to-backup strategy so the original is restored if the copy fails.
4. The installed binary self-identifies as a nightly build (`odyn --version` shows `Nightly, commit <hash>`).

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
- On all platforms, the current binary is renamed to a backup file before the new one is written. If the copy fails, the backup is restored so you are never left without a working `odyn`.
- After updating, restart your shell or re-invoke `odyn` for the new version to take effect.
- Requires an internet connection to reach the Codeberg API and release assets.
