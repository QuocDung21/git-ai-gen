#!/usr/bin/env bash
set -euo pipefail

cargo build --release -p git-ai
test -x target/release/git-ai
