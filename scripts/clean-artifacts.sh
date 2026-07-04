#!/usr/bin/env bash
set -euo pipefail

rm -rf target
find logs -type f -name '*.log' -delete
find packaging/dist/artifacts -type f ! -name '.gitkeep' -delete
find packaging/macos/app-artifacts -type f ! -name '.gitkeep' -delete
