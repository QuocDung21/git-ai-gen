# Windows WinUI Cleanup UI

Reserved slot for a future Windows cleanup UI.

Expected direction:

- Call the shared Rust C ABI from `src/ffi.rs`.
- Keep Windows UI code and build files inside this folder.
- Do not depend on Rust CLI/TUI modules.
