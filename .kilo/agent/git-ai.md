# Git-AI Agent Instructions

This file provides context for AI agents to work efficiently on this Rust TUI codebase with minimal tokens.

## Project Overview

**git-ai** is a terminal UI (TUI) tool for generating Git commit messages with AI assistance. It provides an interactive dashboard for staging files, viewing diffs, committing, pushing, branch management, stash, and more — all with bilingual (Vietnamese/English) support.

**Version**: 3.0.0 | **Language**: Rust (2021) | **No async** — all sync `std::process::Command`

## Core Architecture

```
apps/tui/
├── src/
│   ├── main.rs      # Clap CLI router
│   ├── app/         # App state, config, event loop, key handlers
│   ├── ui/          # Layout, components, modal renderers
│   ├── git/         # Thin wrappers around git CLI commands
│   ├── cli/         # Non-TUI commands and shell install
│   ├── helper/      # Language/theme/history helpers
│   ├── models/      # Enums and shared data models
│   ├── ffi.rs       # Current C ABI exports
│   └── constant/    # Prompt and marker constants
└── tests/
bridge/ffi/          # Dedicated C ABI crate
core/git-ai-core/    # Pure/shared Rust logic and locales
```

## Critical Rules (Always Follow)

1. **Bilingual mandatory**: Every user-facing string must exist in both `core/git-ai-core/locales/en.yml` and `core/git-ai-core/locales/vi.yml`. Always use the `t!` macro from `rust_i18n` (via `use rust_i18n::t;`). Do not use manual `is_vi` or `app.current_lang == "vi"` checks.

2. **TUI is sacred**: All rendering goes through `ratatui`. Never use `println!` inside the dashboard. Only use `logger::*` for CLI commands (outside TUI).

3. **Git via Command only**: Every git operation is `Command::new("git").args([...]).output()?`. No libgit2. Handle errors with `if let Ok(...)` or `?`.

4. **State lives in App**: Add new UI state fields to `App` struct in `apps/tui/src/app/mod.rs`. Never use global statics or separate stores.

5. **Modals via ActiveModal enum**: New floating panels must be added as variants in `apps/tui/src/models/mod.rs:ActiveModal`, rendered in `apps/tui/src/ui/mod.rs`, and handled in `apps/tui/src/app/events/handlers/`.

6. **No comments in code** unless explicitly requested. Existing code has zero comments — keep it that way.

7. **Theme support**: Always use `app.theme()` for colors. Support both `is_light_theme` (Premium Light) and dark (Dracula).

8. **Event loop pattern**: 250ms poll in `apps/tui/src/app/events/mod.rs`. Long-running git ops (commit/push/amend) must be handled in the `if app.active_modal == ...` pre-poll blocks to avoid blocking the UI.

## Common Patterns

**Adding a new Git operation**:
- Add wrapper in `core/git-ai-core/src/git/<module>.rs`
- Call from focused event handlers or `apps/tui/src/cli/system/`
- Update `App` state if needed
- Add bilingual messages in locales

**Adding a new Modal**:
1. Add variant to `ActiveModal` enum (`apps/tui/src/models/mod.rs`)
2. Add state fields to `App` struct if required
3. Add render function in `apps/tui/src/ui/modals/<new>.rs` + export in `mod.rs`
4. Wire size + dispatch in `apps/tui/src/ui/mod.rs`
5. Handle keys in `apps/tui/src/app/events/handlers/`

**Adding a new CLI subcommand**:
- Add variant to `Commands` enum in `apps/tui/src/cli/args.rs`
- Implement handler in `apps/tui/src/cli/system/` or a new module
- Wire in `run()` match

**Reading/writing git config** (global):
```rust
Command::new("git").args(["config", "--global", "git-ai.<key>", value]).output()
```

**Bilingual strings using `t!`**:
```rust
use rust_i18n::t;
let label = t!("key").to_string();
```

**Theme colors** (never hardcode):
```rust
let theme = app.theme();
Style::default().fg(theme.green)
```

## What to Avoid

- Do not introduce async/await or tokio (project is sync-only)
- Do not use `println!`/`eprintln!` inside TUI event loop
- Do not skip locale files when adding user strings
- Do not add comments to source files
- Do not use external git libraries — stick to `std::process::Command`
- Do not break the 3-column layout (28% / 48% / 24%)
- Do not assume a file exists — always check `if let Ok(...)`

## Token-Saving Tips for Agents

- The App struct is intentionally large — adding 2-3 fields per feature is normal.
- Most "logic" is just calling git commands and updating string fields.
- Diff rendering truncates at 500 lines for untracked files.
- Clipboard usage: `arboard::Clipboard` — only for Diff and Go commands.
- When asked to "add feature X", first check if similar modal already exists and copy its pattern exactly.

## Quick Reference

| Task                    | Primary Files                              |
|-------------------------|--------------------------------------------|
| New TUI feature         | `apps/tui/src/app/`, `apps/tui/src/ui/` |
| New modal               | `apps/tui/src/models/mod.rs`, `apps/tui/src/ui/modals/*.rs` |
| Git wrapper             | `core/git-ai-core/src/git/*.rs` |
| Bilingual text          | `core/git-ai-core/locales/en.yml`, `core/git-ai-core/locales/vi.yml` |
| CLI command             | `apps/tui/src/cli/args.rs`, `apps/tui/src/cli/system/` |
| Theme colors            | `apps/tui/src/app/mod.rs:theme()` |
| Language detection      | `apps/tui/src/helper/mod.rs:get_ai_language()`, `get_ai_language_name()` |

Update this file when architecture changes significantly.
