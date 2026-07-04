# bridge/ffi

Dedicated C ABI crate for native consumers.

Build from the workspace root:

```bash
cargo build -p git-ai-ffi
```

The library target is named `git_ai_core` to preserve the existing `libgit_ai_core` artifact name expected by Swift/Kotlin/Node consumers.
