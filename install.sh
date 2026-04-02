#!/bin/sh
# Odyn installer
# https://codeberg.org/razkar/odyn

set -eu

REPO="razkar/odyn"
BINARY_NAME="odyn"

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
RESET='\033[0m'

info()    { printf "${BLUE}    install${RESET} %s\n" "$1"; }
success() { printf "${GREEN}    install${RESET} %s\n" "$1"; }
warn()    { printf "${YELLOW}       warn${RESET} %s\n" "$1"; }
error()   { printf "${RED}      error${RESET} %s\n" "$1"; exit 1; }

tmp_dir=""
cleanup() {
    if [ -n "$tmp_dir" ] && [ -d "$tmp_dir" ]; then
        rm -rf "$tmp_dir"
    fi
}
trap cleanup EXIT

if command -v curl > /dev/null 2>&1; then
    DOWNLOADER="curl"
elif command -v wget > /dev/null 2>&1; then
    DOWNLOADER="wget"
else
    error "neither curl nor wget is installed. please install one and try again."
fi

download() {
    url="$1"
    dest="$2"
    if [ "$DOWNLOADER" = "curl" ]; then
        curl -fsSL "$url" -o "$dest"
    else
        wget -q "$url" -O "$dest"
    fi
}

download_string() {
    url="$1"
    if [ "$DOWNLOADER" = "curl" ]; then
        curl -fsSL "$url"
    else
        wget -q "$url" -O -
    fi
}

os="$(uname -s)"
case "$os" in
    Linux)   os="linux" ;;
    Darwin)  os="macos" ;;
    FreeBSD) os="freebsd" ;;
    NetBSD)  os="netbsd" ;;
    *)       error "unsupported operating system: $os" ;;
esac

if [ "$os" = "linux" ] && [ -n "${ANDROID_ROOT:-}" ]; then
    os="android"
fi

arch="$(uname -m)"
case "$arch" in
    x86_64)           arch="x86_64" ;;
    aarch64 | arm64)  arch="aarch64" ;;
    armv7l | armv7)   arch="armv7" ;;
    armv6l | armv6)   arch="armv6" ;;
    i386 | i686)      arch="i686" ;;
    riscv64)          arch="riscv64" ;;
    powerpc64le)      arch="powerpc64le" ;;
    powerpc64)        arch="powerpc64" ;;
    s390x)            arch="s390x" ;;
    sparc64)          arch="sparc64" ;;
    *)                error "unsupported architecture: $arch" ;;
esac

case "${os}-${arch}" in
    linux-x86_64)      binary="odyn-linux-x86_64" ;;
    linux-aarch64)     binary="odyn-linux-aarch64" ;;
    linux-i686)        binary="odyn-linux-i686" ;;
    linux-riscv64)     binary="odyn-linux-riscv64" ;;
    linux-armv7)       binary="odyn-linux-armv7" ;;
    linux-armv6)       binary="odyn-linux-armv6" ;;
    linux-powerpc64le) binary="odyn-linux-powerpc64le" ;;
    linux-powerpc64)   binary="odyn-linux-powerpc64" ;;
    linux-s390x)       binary="odyn-linux-s390x" ;;
    linux-sparc64)     binary="odyn-linux-sparc64" ;;
    macos-x86_64)      binary="odyn-macos-x86_64" ;;
    macos-aarch64)     binary="odyn-macos-aarch64" ;;
    freebsd-x86_64)    binary="odyn-freebsd-x86_64" ;;
    freebsd-i686)      binary="odyn-freebsd-i686" ;;
    netbsd-x86_64)     binary="odyn-netbsd-x86_64" ;;
    android-aarch64)   binary="odyn-android-aarch64" ;;
    android-armv7)     binary="odyn-android-armv7" ;;
    android-x86_64)    binary="odyn-android-x86_64" ;;
    *)                 error "no binary available for ${os}-${arch}. build from source: https://codeberg.org/razkar/odyn" ;;
esac

# Choose install location
USE_SUDO=0
if [ "$os" = "android" ]; then
    # Termux: install into the Termux prefix bin
    INSTALL_DIR="${PREFIX:-/data/data/com.termux/files/usr}/bin"
elif [ "$(id -u)" = "0" ]; then
    # Already root: default to system-wide silently
    INSTALL_DIR="/usr/local/bin"
elif [ -t 0 ]; then
    printf "\n${CYAN}  Install type:${RESET}\n"
    printf "    1) User        ~/.local/bin\n"
    printf "    2) System-wide /usr/local/bin  (requires sudo)\n"
    printf "  Choice [1]: "
    read -r install_choice
    case "${install_choice:-1}" in
        2)
            INSTALL_DIR="/usr/local/bin"
            USE_SUDO=1
            ;;
        *)
            INSTALL_DIR="$HOME/.local/bin"
            ;;
    esac
    printf "\n"
else
    # Non-interactive (piped): default to user install
    INSTALL_DIR="$HOME/.local/bin"
fi

info "fetching latest version..."
api_response="$(download_string "https://codeberg.org/api/v1/repos/${REPO}/releases/latest")"
version="$(printf '%s' "$api_response" | grep -o '"tag_name":"[^"]*"' | grep -o ':[^}]*' | tr -d ':"')"

if [ -z "$version" ]; then
    error "could not determine latest version from Codeberg API"
fi

info "latest version is $version"

base_url="https://codeberg.org/${REPO}/releases/download/${version}"
binary_url="${base_url}/${binary}"

case "$os" in
    macos | android | freebsd | netbsd)
        sums_url="${base_url}/SHA256SUMS-github" ;;
    *)
        sums_url="${base_url}/SHA256SUMS" ;;
esac

tmp_dir="$(mktemp -d)"
tmp_binary="${tmp_dir}/${binary}"

info "downloading ${binary}..."
download "$binary_url" "$tmp_binary"

if [ ! -s "$tmp_binary" ]; then
    error "downloaded file is empty"
fi

info "verifying checksum..."
sums="$(download_string "$sums_url")"
expected="$(printf '%s' "$sums" | grep " ${binary}$" | awk '{print $1}')"

if [ -z "$expected" ]; then
    error "could not find checksum for $binary in SHA256SUMS"
fi

if command -v sha256sum > /dev/null 2>&1; then
    actual="$(sha256sum "$tmp_binary" | awk '{print $1}')"
elif command -v shasum > /dev/null 2>&1; then
    actual="$(shasum -a 256 "$tmp_binary" | awk '{print $1}')"
else
    warn "no sha256 tool found, skipping checksum verification"
    actual="$expected"
fi

if [ "$actual" != "$expected" ]; then
    error "SHA256 mismatch! expected $expected, got $actual. aborting."
fi

success "checksum verified"

chmod +x "$tmp_binary"
if [ "$USE_SUDO" = "1" ]; then
    sudo mkdir -p "$INSTALL_DIR"
    sudo mv "$tmp_binary" "${INSTALL_DIR}/${BINARY_NAME}"
else
    mkdir -p "$INSTALL_DIR"
    mv "$tmp_binary" "${INSTALL_DIR}/${BINARY_NAME}"
fi

success "odyn $version installed to ${INSTALL_DIR}/${BINARY_NAME}"

case ":$PATH:" in
    *":$INSTALL_DIR:"*)
        success "$INSTALL_DIR is already on your PATH. you're good to go!"
        ;;
    *)
        if [ "$USE_SUDO" = "1" ]; then
            warn "$INSTALL_DIR is not on your PATH."
            warn "add the following to your shell config (~/.bashrc, ~/.zshrc, etc.):"
            printf "\n    export PATH=\"%s:\$PATH\"\n\n" "$INSTALL_DIR"
        else
            warn "~/.local/bin is not on your PATH."
            warn "add the following to your shell config (~/.bashrc, ~/.zshrc, etc.):"
            printf "\n    export PATH=\"\$HOME/.local/bin:\$PATH\"\n\n"
            warn "then restart your shell or run: source ~/.bashrc"
        fi
        ;;
esac
