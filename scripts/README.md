# scripts

Release, packaging, and maintenance scripts should live here.

Keep scripts deterministic and runnable from the workspace root unless a script documents otherwise.

- `check.sh`: runs formatting, full/slim checks, tests, and clippy.
- `build-tui.sh`: builds the release `git-ai` binary.
- `build-ffi.sh`: builds the release FFI library artifacts.
- `clean-artifacts.sh`: removes local build/log/package artifacts.
