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
├── main.rs
├── app/
│   ├── mod.rs                 # App struct + small core methods (keep < 400 LOC if possible)
│   ├── models.rs              # Data models & enums only (ActiveModal, Entry structs, Steps...)
│   ├── events.rs              # Main event loop + key routing (avoid putting heavy logic here)
│   ├── state/                 # (Preferred) Domain-specific state modules
│   └── handlers/              # (Preferred) Grouped action handlers
├── ui/
│   ├── mod.rs
│   ├── components/            # Reusable UI pieces (header, changes, legend, diff)
│   └── modals/                # ← Each modal should live in its own file
│       ├── mod.rs
│       ├── manual_commit.rs
│       ├── commit_tree.rs
│       ├── git_log.rs
│       ├── branch.rs
│       ├── github_download.rs  # Grouped Github Downloader, Tree, Quick View & Branch modal renderers
│       └── ...
├── git/                       # Pure Git command wrappers (excellent separation)
│   ├── mod.rs
│   ├── status.rs
│   ├── commit.rs
│   ├── branch.rs
│   ├── remote.rs
│   └── stash.rs
├── cli/
├── helper/
└── constant/
```

**Key Principle**: Keep files small and focused. One concept = one file when reasonable.

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

**Update this file** whenever the architecture changes significantly (especially when moving more modals out of `confirm.rs` or introducing new modules like `state/` or `handlers/`).

Last updated: 2026-05-23
