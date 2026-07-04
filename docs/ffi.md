# FFI Guide

The C ABI is owned by `bridge/ffi`.

## Build

```bash
cargo build -p git-ai-ffi
```

Release build:

```bash
cargo build --release -p git-ai-ffi
```

The library target is named `git_ai_core`, preserving the existing native artifact names:

- `target/{debug,release}/libgit_ai_core.a`
- `target/{debug,release}/libgit_ai_core.dylib` on macOS
- `target/{debug,release}/libgit_ai_core.so` on Linux

## Ownership

Functions returning `char *` allocate a C string. Callers must release it with:

```c
void git_ai_free_string(char *s);
```

## Error Contract

New JSON-returning cleanup APIs return structured errors:

```json
{
  "ok": false,
  "code": "invalid_path_pointer",
  "error": "Invalid path pointer",
  "message": "Invalid path pointer"
}
```

The legacy `error` field is retained for compatibility. Some older commit/push APIs still return plain strings such as `Success` or `Error: ...`; add v2 JSON APIs instead of changing those legacy functions in place.
