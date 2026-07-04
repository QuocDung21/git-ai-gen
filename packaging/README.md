# packaging

Packaging and release artifacts live here.

- `homebrew/`: Homebrew formula/cask material.
- `macos/`: macOS static library or app artifacts.
- `dist/`: built binary artifacts.

Generated binary artifacts are ignored by Git. Keep `.gitkeep` placeholders in artifact directories and produce fresh artifacts with:

```bash
scripts/build-tui.sh
scripts/build-ffi.sh
```

The `homebrew/git-ai.rb` file is a placeholder until release metadata and checksums are generated.
