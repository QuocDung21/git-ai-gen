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

## Cleanup

```bash
scripts/clean-artifacts.sh
```
