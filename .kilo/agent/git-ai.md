# Git-AI Agent Instructions

This file provides context for AI agents to work efficiently on this Rust TUI codebase with minimal tokens.

## Project Overview

**git-ai** is a terminal UI (TUI) tool for generating Git commit messages with AI assistance. It provides an interactive dashboard for staging files, viewing diffs, committing, pushing, branch management, stash, and more — all with bilingual (Vietnamese/English) support.

**Version**: 3.0.0 | **Language**: Rust (2021) | **No async** — all sync `std::process::Command`

## Core Architecture

```
src/
├── main.rs          # Clap CLI router (Diff, Go, Lang, Install, etc.)
├── app/
│   ├── mod.rs       # App struct (65+ state fields), refresh_git_status, theme()
│   ├── models.rs    # Enums: ActiveModal, GoStep, AmendStep, StashStep...
│   └── events.rs    # Event loop, key handling, modal intercepts (1275 lines)
├── ui/
│   ├── mod.rs       # Layout + modal dispatch
│   ├── components/  # header, changes, diff, legend
│   └── modals/      # help, confirm, branch, stash, gitlog, etc.
├── git/
│   ├── status.rs    # get_git_status, get_diff_*, stage_*, revert_file
│   ├── commit.rs    # commit, amend_commit, get_last_commit_subject
│   ├── branch.rs    # checkout, create, merge, get_branches
│   ├── remote.rs    # push, pull, fetch, ahead/behind
│   └── stash.rs     # push, pop, apply, drop
├── cli/
│   ├── system.rs    # handle_diff, handle_go, handle_lang, handle_restore
│   ├── install.rs   # shell alias setup
│   └── locales.rs   # en.yml / vi.yml via include_str + serde_yaml
├── helper/mod.rs    # get_ai_language, get_os_theme, get_locales
└── constant/mod.rs  # PROMPT_EXPERT (AI prompt prefix)
```

## Critical Rules (Always Follow)

1. **Bilingual mandatory**: Every user-facing string must exist in both `locales/en.yml` and `locales/vi.yml`. Use `app.current_lang == "vi"` checks. Never hardcode English-only strings.

2. **TUI is sacred**: All rendering goes through `ratatui`. Never use `println!` inside the dashboard. Only use `logger::*` for CLI commands (outside TUI).

3. **Git via Command only**: Every git operation is `Command::new("git").args([...]).output()?`. No libgit2. Handle errors with `if let Ok(...)` or `?`.

4. **State lives in App**: Add new UI state fields to `App` struct in `app/mod.rs`. Never use global statics or separate stores.

5. **Modals via ActiveModal enum**: New floating panels must be added as variants in `app/models.rs:ActiveModal`, rendered in `ui/mod.rs`, and handled in `events.rs`.

6. **No comments in code** unless explicitly requested. Existing code has zero comments — keep it that way.

7. **Theme support**: Always use `app.theme()` for colors. Support both `is_light_theme` (Premium Light) and dark (Dracula).

8. **Event loop pattern**: 250ms poll in `events.rs:run_app`. Long-running git ops (commit/push/amend) must be handled in the `if app.active_modal == ...` pre-poll blocks to avoid blocking the UI.

## Common Patterns

**Adding a new Git operation**:
- Add wrapper in `src/git/<module>.rs`
- Call from `events.rs` or `cli/system.rs`
- Update `App` state if needed
- Add bilingual messages in locales

**Adding a new Modal**:
1. Add variant to `ActiveModal` enum (`app/models.rs`)
2. Add state fields to `App` struct if required
3. Add render function in `ui/modals/<new>.rs` + export in `mod.rs`
4. Wire size + dispatch in `ui/mod.rs:ui()`
5. Handle keys in `events.rs` under `match &app.active_modal`

**Adding a new CLI subcommand**:
- Add variant to `Commands` enum in `main.rs`
- Implement handler in `cli/system.rs` or new module
- Wire in `run()` match

**Reading/writing git config** (global):
```rust
Command::new("git").args(["config", "--global", "git-ai.<key>", value]).output()
```

**Current language detection**:
```rust
let is_vi = app.current_lang == "vi";
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
| New TUI feature         | `app/mod.rs`, `app/events.rs`, `ui/mod.rs` |
| New modal               | `app/models.rs`, `ui/modals/*.rs`          |
| Git wrapper             | `src/git/*.rs`                             |
| Bilingual text          | `locales/en.yml`, `locales/vi.yml`         |
| CLI command             | `main.rs`, `cli/system.rs`                 |
| Theme colors            | `app/mod.rs:theme()`                       |
| Language detection      | `helper/mod.rs:get_ai_language()`          |

Update this file when architecture changes significantly.
