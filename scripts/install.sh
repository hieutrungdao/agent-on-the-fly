#!/usr/bin/env bash
# Copyright 2026 Hieu Trung Dao
# SPDX-License-Identifier: Apache-2.0
#
# Install the latest `aotf` release. Works on Linux x86_64; v0.0.2-alpha
# does not ship macOS or aarch64 builds.
#
# Usage:
#   curl -sSfL https://github.com/hieutrungdao/agent-on-the-fly/releases/latest/download/install.sh | bash
#
# Optional environment overrides:
#   AOTF_VERSION=v0.0.2-alpha.1   # pin a specific tag
#   AOTF_INSTALL_DIR=$HOME/.local/bin

set -euo pipefail

REPO="hieutrungdao/agent-on-the-fly"
VERSION="${AOTF_VERSION:-latest}"
INSTALL_DIR="${AOTF_INSTALL_DIR:-$HOME/.local/bin}"

# Resolve platform.
uname_s="$(uname -s)"
uname_m="$(uname -m)"
case "$uname_s/$uname_m" in
  Linux/x86_64)
    asset_suffix="linux-x86_64"
    ;;
  *)
    echo "aotf: walking-skeleton (v0.0.2-alpha.1) only ships Linux x86_64." >&2
    echo "  detected: $uname_s/$uname_m" >&2
    echo "  build from source: cargo install --git https://github.com/$REPO --bin aotf" >&2
    exit 1
    ;;
esac

if [[ "$VERSION" == "latest" ]]; then
  base_url="https://github.com/$REPO/releases/latest/download"
else
  base_url="https://github.com/$REPO/releases/download/$VERSION"
fi

mkdir -p "$INSTALL_DIR"

download() {
  local name="$1"
  local dest="$2"
  local url="$base_url/$name"
  if command -v curl >/dev/null 2>&1; then
    curl -sSfL "$url" -o "$dest"
  elif command -v wget >/dev/null 2>&1; then
    wget -q "$url" -O "$dest"
  else
    echo "aotf install: need curl or wget" >&2
    exit 1
  fi
}

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

echo "downloading aotf-$asset_suffix..."
download "aotf-$asset_suffix" "$tmp/aotf"
download "aotfd-$asset_suffix" "$tmp/aotfd"
download "aotf-gatekeeper-$asset_suffix" "$tmp/aotf-gatekeeper"
download "SHA256SUMS" "$tmp/SHA256SUMS"

echo "verifying checksums..."
# Filter to only the three binaries we downloaded.
grep -E " (aotf|aotfd|aotf-gatekeeper)-$asset_suffix$" "$tmp/SHA256SUMS" \
  | sed "s| \(.*\)-$asset_suffix$| $tmp/\1|" \
  | sha256sum --check --status \
  || { echo "aotf install: checksum mismatch — aborting" >&2; exit 1; }

chmod +x "$tmp/aotf" "$tmp/aotfd" "$tmp/aotf-gatekeeper"

mv "$tmp/aotf" "$INSTALL_DIR/aotf"
mv "$tmp/aotfd" "$INSTALL_DIR/aotfd"
mv "$tmp/aotf-gatekeeper" "$INSTALL_DIR/aotf-gatekeeper"

echo "installed to $INSTALL_DIR/aotf"
case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *)
    echo
    echo "  $INSTALL_DIR is not on your PATH."
    echo "  Add this to your shell profile:"
    echo "    export PATH=\"$INSTALL_DIR:\$PATH\""
    ;;
esac

echo
"$INSTALL_DIR/aotf" --version || true
