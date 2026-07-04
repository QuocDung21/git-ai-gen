# Architecture Guide

`git-ai-gen` is organized as a workspace inspired by `look`: app shells live under `apps/`, shared Rust logic lives under `core/`, and bridge boundaries live under `bridge/`.

## Workspace Layout

```text
git-ai-gen/
├── apps/
│   ├── tui/                 # Rust TUI package and current git-ai binary
│   └── macos/
│       └── LauncherApp/     # SwiftUI macOS shell, shaped like look
├── bridge/
│   └── ffi/                 # Dedicated C ABI crate
├── core/
│   └── git-ai-core/         # Pure/shared Rust logic
├── docs/
├── examples/
├── packaging/
│   ├── dist/
│   ├── homebrew/
│   └── macos/
├── scripts/
└── Cargo.toml               # workspace manifest
```

## Current Runtime Boundary

The current production TUI crate is `apps/tui`. The native macOS shell lives in `apps/macos/LauncherApp`. Pure shared logic lives in `core/git-ai-core`. FFI exports live in `bridge/ffi`, which produces the static library artifact expected by native consumers.

```mermaid
flowchart LR
    User["Terminal user"] --> TUI["apps/tui\nRatatui dashboard"]
    MacUser["macOS user"] --> Mac["apps/macos/LauncherApp\nSwiftUI shell"]
    TUI --> Shared["core/git-ai-core\ngit/helper/models/locales/theme/cleanup"]
    Mac --> FFI
    FFI["bridge/ffi\nC ABI exports"] --> Shared
    Shared --> Git["git CLI\nstd::process::Command"]
```

## Package Boundary

```mermaid
flowchart LR
    TUI["apps/tui"] --> Core["core/git-ai-core"]
    Mac["apps/macos/LauncherApp"] --> Bridge
    Bridge["bridge/ffi"] --> Core
    Core --> Git["git CLI"]
```

Current and intended ownership:

- `core/git-ai-core`: Git wrappers, cleanup scanner, helper logic, models, theme data, locale helpers.
- `bridge/ffi`: `staticlib` / `cdylib` / `rlib` ABI package depending on `core/git-ai-core`.
- `apps/tui`: Ratatui dashboard, interactive CLI, modal/event handlers, and app state.
- `apps/macos/LauncherApp`: SwiftUI macOS app shell and logic tests that call Rust only through the C ABI.

## Build Commands

From the workspace root:

```bash
cargo check
cargo check --no-default-features -p git-ai
cargo check -p git-ai-ffi
cargo test
cargo clippy --all-targets -- -D warnings
cargo clippy --no-default-features -p git-ai -- -D warnings
cargo clippy -p git-ai-ffi -- -D warnings
```

The same verification flow is available through:

```bash
scripts/check.sh
```

## Migration Rule

Keep each migration step buildable. The `git-ai-ffi` package owns ABI exports and keeps the `libgit_ai_core` library name for downstream native consumers.
