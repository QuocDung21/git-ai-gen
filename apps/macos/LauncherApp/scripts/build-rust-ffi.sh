#!/usr/bin/env bash
set -euo pipefail

PROJECT_DIR="${PROJECT_DIR:-$(cd "$(dirname "$0")/.." && pwd)}"
REPO_ROOT="$(cd "$PROJECT_DIR/../../.." && pwd)"

if [ -f "$HOME/.cargo/env" ]; then
  # shellcheck disable=SC1091
  . "$HOME/.cargo/env"
fi

export PATH="$HOME/.cargo/bin:/opt/homebrew/opt/rustup/bin:/opt/homebrew/bin:/usr/local/bin:$PATH"
export CARGO_TARGET_DIR="$REPO_ROOT/target"

if [ "$CONFIGURATION" = "Release" ]; then
  PROFILE=release
  CARGO_FLAGS="--release"
else
  PROFILE=debug
  CARGO_FLAGS=""
fi

CARGO_BIN="${CARGO_BIN:-}"
if [ -z "$CARGO_BIN" ]; then
  CARGO_BIN="$(command -v cargo || true)"
fi
if [ -z "$CARGO_BIN" ]; then
  CARGO_BIN="$(/bin/zsh -lc 'command -v cargo' 2>/dev/null || true)"
fi
if [ -z "$CARGO_BIN" ]; then
  echo "error: cargo not found. Install Rust or set CARGO_BIN to the cargo executable path for Xcode." >&2
  echo "hint: common paths are ~/.cargo/bin/cargo and /opt/homebrew/opt/rustup/bin/cargo." >&2
  exit 1
fi

"$CARGO_BIN" build --manifest-path "$REPO_ROOT/bridge/ffi/Cargo.toml" $CARGO_FLAGS

LIB_PATH="$REPO_ROOT/target/$PROFILE/libgit_ai_core.a"
if [ ! -f "$LIB_PATH" ]; then
  echo "error: Rust FFI library was not produced: $LIB_PATH" >&2
  exit 1
fi

mkdir -p "$PROJECT_DIR/RustBuild"
cp "$LIB_PATH" "$PROJECT_DIR/RustBuild/libgit_ai_core.a"
