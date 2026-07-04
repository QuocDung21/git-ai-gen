# GitAiMacApp

Canonical SwiftUI macOS app shell for `git-ai`, organized in the same shape as `look` under `apps/macos/LauncherApp`.

## Structure

```text
apps/macos/LauncherApp/
├── Package.swift
├── git-ai-app.xcodeproj/
├── CGitAiCore/
│   ├── git_ai_core.c
│   └── include/
│       ├── git_ai_core.h
│       └── module.modulemap
├── LauncherLogicTests/
└── git-ai-app/
    ├── App/
    │   └── GitAiCleanupApp.swift
    └── Features/
        └── Cleanup/
            ├── Models/
            ├── Services/
            └── Views/
```

Swift calls only the stable C ABI:

```c
char *git_ai_cleanup_scan_node_modules(const char *path);
char *git_ai_cleanup_scan_build_folders(const char *path);
char *git_ai_cleanup_scan_devcleaner(const char *path);
char *git_ai_cleanup_scan_node_modules_stream(
    const char *path,
    git_ai_cleanup_scan_callback callback,
    void *user_data
);
char *git_ai_cleanup_scan_node_modules_stream_cancellable(
    const char *path,
    git_ai_cleanup_scan_callback callback,
    git_ai_cleanup_should_cancel_callback should_cancel,
    void *user_data
);
char *git_ai_cleanup_scan_build_folders_stream(
    const char *path,
    git_ai_cleanup_scan_callback callback,
    void *user_data
);
char *git_ai_cleanup_scan_build_folders_stream_cancellable(
    const char *path,
    git_ai_cleanup_scan_callback callback,
    git_ai_cleanup_should_cancel_callback should_cancel,
    void *user_data
);
char *git_ai_cleanup_scan_devcleaner_stream(
    const char *path,
    git_ai_cleanup_scan_callback callback,
    void *user_data
);
char *git_ai_cleanup_scan_devcleaner_stream_cancellable(
    const char *path,
    git_ai_cleanup_scan_callback callback,
    git_ai_cleanup_should_cancel_callback should_cancel,
    void *user_data
);
char *git_ai_cleanup_delete_paths(const char *paths_json);
void git_ai_free_string(char *s);
```

The Swift UI does not import Rust internals or `apps/tui/src/cli`.

## Build Rust Core

From the repository root:

```bash
cargo build -p git-ai-ffi
```

This creates:

```text
target/debug/libgit_ai_core.a
```

## Run SwiftUI App

From this folder:

```bash
swift run
```

The package links against `../../../target/debug/libgit_ai_core.a` or `../../../target/release/libgit_ai_core.a`.

## Open in Xcode

Open the native project:

```bash
open apps/macos/LauncherApp/git-ai-app.xcodeproj
```

The `GitAi` scheme has a `Build Rust FFI` phase that builds `bridge/ffi` and links `libgit_ai_core.a`.
The build phase delegates to `scripts/build-rust-ffi.sh`, which loads `~/.cargo/env` so Xcode GUI builds can find `cargo`.

## Xcode Previews

Open `git-ai-app/Features/Cleanup/Views/CleanupPreview.swift` and select the `GitAiMacLogic` scheme for Canvas previews. If Xcode does not preview package files on your machine, use `swift run` for the app workflow.

## Notes

- The default UI scan targets `node_modules`.
- Build folders and DevCleaner-style scans are available only from the Settings modal.
- Settings are organized with tabs so future cleanup options can be added without crowding the main screen.
- Streaming scan sends each found folder to Swift immediately through a C callback.
- Broad scan roots such as the home folder require confirmation and can be cancelled while scanning.
- Delete accepts a JSON string array of paths and returns JSON `reports`.
- The Swift wrapper always calls `git_ai_free_string` after reading Rust strings.
- macOS Swift code is organized by feature so new screens can be added without crowding the target root.
- The macOS app intentionally stays outside the Rust TUI so native shells can evolve without touching Rust dashboard modules.
