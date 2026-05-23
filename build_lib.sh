#!/bin/bash
set -e

cargo build --release --lib

mkdir -p macos-app
cp target/release/libgit_ai_core.a macos-app/libgit_ai_core.a || true

mkdir -p git-chill/git-chill
cp target/release/libgit_ai_core.a git-chill/git-chill/libgit_ai_core.a || true
