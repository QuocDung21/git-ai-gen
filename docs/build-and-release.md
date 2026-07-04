# Build and Release

Run commands from the workspace root.

## Verify

```bash
scripts/check.sh
```

## Build TUI

```bash
scripts/build-tui.sh
```

Output:

```text
target/release/git-ai
```

## Build FFI

```bash
scripts/build-ffi.sh
```

Outputs:

```text
target/release/libgit_ai_core.a
target/release/libgit_ai_core.dylib
```

On Linux the dynamic library uses `.so`.

## Build macOS App

```bash
scripts/build-macos-app.sh
```

This builds the FFI debug library first, then runs `swift build` and `swift test` in `apps/macos/LauncherApp`.

## Cleanup

```bash
scripts/clean-artifacts.sh
```
