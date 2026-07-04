# Cleanup UI Examples

Platform-specific cleanup UI examples live here. Each app calls the shared Rust FFI core and should keep platform UI code isolated from `apps/tui/src/cli` and the TUI modules.

## Structure

```text
examples/cleanup-ui/
├── README.md
├── linux-gtk/
└── windows-winui/
```

## Current Example

- The canonical macOS SwiftUI app now lives in `apps/macos/LauncherApp`, matching the workspace layout used by `look`.

## Future Platform Slots

- `linux-gtk`: Linux desktop UI can live here when added.
- `windows-winui`: Windows desktop UI can live here when added.

## Shared Rules

- Build Rust core from the repository root with `cargo build -p git-ai-ffi`.
- Use only exported FFI functions from `bridge/ffi/src/lib.rs`.
- Do not import Rust CLI/TUI internals into platform UI examples.
- Keep each platform's build system inside its own folder.
