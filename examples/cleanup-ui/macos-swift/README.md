# GitAiCleanupUI

SwiftUI macOS example that calls `git-ai` cleanup through the Rust C ABI.

## Structure

```text
examples/cleanup-ui/macos-swift/
├── Package.swift
├── Sources/
│   ├── CGitAiCore/
│   │   ├── git_ai_core.c
│   │   └── include/git_ai_core.h
│   ├── GitAiCleanupUI/
│   │   └── App/
│   │       └── GitAiCleanupApp.swift
│   ├── GitAiCleanupUILibrary/
│   │   └── Features/
│   │       └── Cleanup/
│   │           ├── Models/
│   │           │   └── CleanupModels.swift
│   │           ├── Services/
│   │           │   └── RustCleanupBridge.swift
│   │           └── Views/
│   │               ├── CleanupBroadScanWarningView.swift
│   │               ├── CleanupPreview.swift
│   │               ├── CleanupResultsView.swift
│   │               ├── CleanupSettingsView.swift
│   │               └── CleanupView.swift
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

The Swift UI does not import Rust internals or `src/cli`.

## Build Rust Core

From the repository root:

```bash
cargo build --no-default-features
```

This creates:

```text
target/debug/libgit_ai_core.a
```

## Run SwiftUI Example

From this folder:

```bash
swift run
```

The package links against `../../../target/debug/libgit_ai_core.a`.

## Xcode Previews

Open `Sources/GitAiCleanupUILibrary/Features/Cleanup/Views/CleanupPreview.swift` and select the `GitAiCleanupUILibrary` scheme for Canvas previews. If Xcode does not preview package files on your machine, use `swift run` for the app workflow.

## Notes

- The default UI scan targets `node_modules`.
- Build folders and DevCleaner-style scans are available only from the Settings modal.
- Settings are organized with tabs so future cleanup options can be added without crowding the main screen.
- Streaming scan sends each found folder to Swift immediately through a C callback.
- Broad scan roots such as the home folder require confirmation and can be cancelled while scanning.
- Delete accepts a JSON string array of paths and returns JSON `reports`.
- The Swift wrapper always calls `git_ai_free_string` after reading Rust strings.
- macOS Swift code is organized by feature so new screens can be added without crowding the target root.
- This example intentionally keeps cleanup UI outside the Rust app so future Swift/macOS apps can replace it without touching Rust core modules.
