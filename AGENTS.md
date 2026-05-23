# Git-AI Agent Instructions

This file provides the **standard context** for AI agents to work on this Rust TUI project with **maximum efficiency and minimum tokens**.

## Project Overview

**git-ai** is a terminal UI (TUI) tool for generating Git commit messages with AI assistance. It provides a full interactive dashboard including staging, diff viewing, AI/manual commit, push, branch management, stash, remote operations, and bilingual (Vietnamese/English) support.

**Version**: 3.0.0  
**Language**: Rust (2021)  
**Hard Constraint**: **No async** — everything uses synchronous `std::process::Command`

## Current Recommended Architecture (2026)

```
src/
├── main.rs                    # Binary entry point (always builds full TUI)
├── lib.rs                     # Library root — uses Cargo feature "tui"
├── ffi.rs                     # C ABI layer (#[no_mangle] exports) — always compiled
│
├── constant/                  # Pure constants (prompts, markers) — always public
├── git/                       # Pure Git command wrappers — always public
│   ├── mod.rs
│   ├── status.rs
│   ├── commit.rs
│   ├── remote.rs
│   ├── branch.rs
│   └── stash.rs
├── helper/                    # Helper utilities (get_ai_language, get_locales, ...) — always public
├── locales.rs                 # i18n (struct Locales + new()) — top-level for FFI + helper
├── models/                    # All data models (BranchEntry, ActiveModal, ...) — top-level shared
├── theme/                     # Theme definitions (AppTheme, palettes) — always public
│
├── app/                       # Heavy TUI state & logic (only when "tui" feature)
│   ├── mod.rs
│   ├── events.rs
│   └── ...
├── ui/                        # Ratatui rendering + modals (only when "tui" feature)
├── cli/                       # Interactive CLI commands (only when "tui" feature)
```

**Cargo Feature Split (Critical for FFI)**

- `default = ["tui"]`
- `cargo build` (or for the binary) → full TUI included
- `cargo build --no-default-features` → **slim FFI library only** (constant + git + helper + locales + models + theme + ffi). No ratatui, no console, no interactive code.

This allows building a tiny staticlib/rlib for Swift, Kotlin, Node, etc. without pulling the entire dashboard.

**Key Principle**: Keep files small and focused. One concept = one file when reasonable. Pure/shared logic lives at the top level (`models/`, `locales.rs`, `git/`, `helper/`). Heavy interactive code lives behind the `tui` feature.

## Critical Rules (Always Follow)

1. **Bilingual mandatory**  
   Every user-facing string must exist in both `locales/en.yml` and `locales/vi.yml`. Always check `app.current_lang == "vi"`.

2. **TUI is sacred**  
   All rendering must go through `ratatui`. Never use `println!` / `eprintln!` inside the dashboard.

3. **Git via Command only**  
   Every Git operation uses `Command::new("git").args([...]).output()?`. No `libgit2`.

4. **State lives in App**  
   Add new UI state to the `App` struct in `app/mod.rs`. No global statics.

5. **Modals via ActiveModal**  
   New floating panels **must** be added as variants in `app/models.rs`, rendered via `ui/mod.rs`, and handled in `events.rs`.

6. **No comments in source code** (unless explicitly requested).

7. **Theme support**  
   Always use `app.theme()`. Support both light (`is_light_theme`) and dark themes.

8. **Long-running operations**  
   Handle in pre-poll blocks (`if app.active_modal == ...`) before `event::poll`.

9. **Safe Clipboard & Paste operations**  
   Use `Event::Paste(text)` via Bracketed Paste Mode (`EnableBracketedPaste`) globally in the main event loop to append text to inputs (e.g. `ManualCommit`, `NewBranchInput`), avoiding raw keystroke splitting and newline conflicts. In addition, support Ctrl+V via `arboard::Clipboard` as a fallback.

10. **Syntax Highlighting & Line Numbers in Viewers**  
    Quick view text modes should use the custom tokenizer (`highlight_line`) and prepend clean formatted line numbers (` 1 │`) for premium aesthetics and maximum readability.

11. **Directory Structure Preservation in Downloads**  
    Repository downloads must preserve hierarchical folder structures by using `entry.path` instead of `entry.name` when joining directories, and optimize performance by filtering out redundant child paths if their parent directory is already selected.

## How to Work Efficiently (Token-Saving Rules)

- **Read the smallest possible context** — prefer one focused file over large ones.
- **Copy existing patterns** — look for similar modals or handlers before creating new ones.
- **Prefer per-file modals** — do not add new logic into `confirm.rs`.
- **Use `app.theme()` everywhere** for colors.
- When adding a feature, first check if a similar pattern already exists in the codebase.

## Common Patterns

### Adding a New Modal (Preferred Modern Way)

1. Add variant to `ActiveModal` enum (`app/models.rs`)
2. (Optional) Add state fields to `App` struct
3. **Create new file**: `ui/modals/<name>.rs`
4. Export the render function in `ui/modals/mod.rs`
5. Add size + dispatch in `ui/mod.rs`
6. Handle keys in `events.rs` under `match &app.active_modal`

### Adding a Git Operation

- Add pure wrapper in `src/git/<module>.rs`
- Call it from `events.rs` or `cli/system.rs`
- Update relevant `App` state
- Add bilingual status messages

### Theme Colors

```rust
let theme = app.theme();
Style::default().fg(theme.green).add_modifier(Modifier::BOLD)
```

## What to Avoid

- Do not put multiple modals into one file (legacy `confirm.rs`).
- Do not create files larger than ~400 lines without strong reason.
- Do not introduce async/await or tokio.
- Do not hardcode colors or user-facing strings.
- Do not use external Git libraries.
- Do not skip bilingual support.

## Quick Reference Table

| Task                    | Primary Files                           | Notes                         |
| ----------------------- | --------------------------------------- | ----------------------------- |
| New Modal (recommended) | `app/models.rs` + `ui/modals/<name>.rs` | One file per modal            |
| New Git wrapper         | `src/git/<module>.rs`                   | Keep pure                     |
| Add UI state            | `app/mod.rs` (App struct)               | Keep struct manageable        |
| Handle keys for modal   | `events.rs`                             | Use `match &app.active_modal` |
| Theme colors            | `app.theme()`                           | Never hardcode                |
| Bilingual strings       | `locales/en.yml` + `locales/vi.yml`     | Always check `is_vi`          |

---

**Update this file** whenever the architecture changes significantly (especially when introducing top-level modules like `models/` / `locales.rs`, or changing the `tui` feature split).

### Working with FFI / Slim Library

When modifying code that must work for FFI consumers:

- Always test both:
  - `cargo check`                    (full TUI)
  - `cargo check --no-default-features` (slim FFI mode)
- New pure logic (Git wrappers, helpers, data types, i18n) **must** live in the always-compiled modules (`git/`, `helper/`, `models/`, `locales.rs`, `constant/`, `theme/`).
- Never add `use crate::app::...` or `use crate::ui::...` or `use crate::cli::...` from FFI-reachable code.
- The blanket `#![allow(dead_code)]` was removed from lib.rs root. TUI modules carry their own scoped allow when the feature is active.

Last updated: 2026-05-23
