# Linux GTK Cleanup UI

Reserved slot for a future Linux cleanup UI.

Expected direction:

- Call the shared Rust C ABI from `bridge/ffi/src/lib.rs`.
- Keep Linux UI code and build files inside this folder.
- Do not depend on Rust CLI/TUI modules.
