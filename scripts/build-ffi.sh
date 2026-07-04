#!/usr/bin/env bash
set -euo pipefail

cargo build --release -p git-ai-ffi
test -f target/release/libgit_ai_core.a

if [[ "$(uname -s)" == "Darwin" ]]; then
  test -f target/release/libgit_ai_core.dylib
else
  test -f target/release/libgit_ai_core.so
fi
