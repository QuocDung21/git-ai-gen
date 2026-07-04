#!/usr/bin/env bash
set -euo pipefail

cargo fmt --all --check
cargo check
cargo check --no-default-features -p git-ai
cargo check -p git-ai-ffi
cargo test
cargo clippy --all-targets -- -D warnings
cargo clippy --no-default-features -p git-ai -- -D warnings
cargo clippy -p git-ai-ffi -- -D warnings
