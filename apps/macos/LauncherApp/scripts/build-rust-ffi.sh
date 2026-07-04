#!/usr/bin/env bash
set -euo pipefail

PROJECT_DIR="${PROJECT_DIR:-$(cd "$(dirname "$0")/.." && pwd)}"
REPO_ROOT="$(cd "$PROJECT_DIR/../../.." && pwd)"

if [ -f "$HOME/.cargo/env" ]; then
  # shellcheck disable=SC1091
  . "$HOME/.cargo/env"
fi

export PATH="$HOME/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:$PATH"
export CARGO_TARGET_DIR="$REPO_ROOT/target"

if [ "$CONFIGURATION" = "Release" ]; then
  PROFILE=release
  CARGO_FLAGS="--release"
else
  PROFILE=debug
  CARGO_FLAGS=""
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "error: cargo not found. Install Rust or ensure ~/.cargo/bin is available to Xcode." >&2
  exit 1
fi

cargo build --manifest-path "$REPO_ROOT/bridge/ffi/Cargo.toml" $CARGO_FLAGS

LIB_PATH="$REPO_ROOT/target/$PROFILE/libgit_ai_core.a"
if [ ! -f "$LIB_PATH" ]; then
  echo "error: Rust FFI library was not produced: $LIB_PATH" >&2
  exit 1
fi

mkdir -p "$PROJECT_DIR/RustBuild"
cp "$LIB_PATH" "$PROJECT_DIR/RustBuild/libgit_ai_core.a"
