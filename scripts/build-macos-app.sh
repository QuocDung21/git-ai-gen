#!/usr/bin/env bash
set -euo pipefail

cargo build -p git-ai-ffi

(
  cd apps/macos/LauncherApp
  swift build
  swift test
)
