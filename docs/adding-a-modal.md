# Adding a New Modal (Recommended Pattern)

This document describes the **preferred modern way** to add a new modal in git-ai.

## Why We Split Modals

- Large catch-all modal files are hard to review and expensive for agents to load.
- Each modal should live in its own file for better maintainability and lower token usage for AI agents.
- Follow the principle: **One concept = one file**.

## Step-by-Step Guide

### 1. Add the Modal Variant

Edit `core/git-ai-core/src/models/mod.rs`:

```rust
pub enum ActiveModal {
    // ... existing variants

    MyNewModal,           // Add your variant here
}
```

### 2. (Optional) Add State to App

If your modal needs state, add it in `apps/tui/src/app/mod.rs`:

```rust
pub struct App {
    // ... existing fields

    pub my_modal_input: String,
    pub my_modal_selected_index: usize,
}
```

Initialize it in `App::new()`.

### 3. Create the Modal File

Create a new file: `apps/tui/src/ui/modals/my_new_modal.rs`

The file should export a render function with this signature:

```rust
pub fn render_my_new_modal(f: &mut Frame, app: &App, area: Rect) {
    // Your rendering logic here
}
```

**Example structure**:

```rust
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;

use rust_i18n::t;

pub fn render_my_new_modal(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();

    f.render_widget(Clear, area);

    let content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("my_modal_title").to_string(),
            Style::default().fg(theme.green).add_modifier(Modifier::BOLD),
        )]),
        // ... more lines
    ];

    let block = Block::default()
        .title(" My Modal ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.green))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .block(block);

    f.render_widget(paragraph, area);
}
```

### 4. Export the Modal

Edit `apps/tui/src/ui/modals/mod.rs`:

```rust
pub mod my_new_modal;

pub use my_new_modal::render_my_new_modal;
```

### 5. Register Size and Dispatch

Edit `apps/tui/src/ui/mod.rs`:

```rust
// In the size calculation match
ActiveModal::MyNewModal => modals::centered_rect(60, 50, f.size()),

// In the render dispatch match
ActiveModal::MyNewModal => modals::render_my_new_modal(f, app, area),
```

### 6. Handle Keyboard Input

Create or update a focused handler under `apps/tui/src/app/events/handlers/`, then wire it in `apps/tui/src/app/events/handlers/mod.rs`:

```rust
pub fn handle_my_new_modal(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.active_modal = ActiveModal::None;
        }
        KeyCode::Enter => {
            // Handle confirm
            app.active_modal = ActiveModal::None;
        }
        KeyCode::Char(c) => {
            // Handle input
        }
        _ => {}
    }
}
```

## Naming Convention

- File: `snake_case.rs` (e.g. `commit_tree.rs`, `manual_commit.rs`)
- Render function: `render_xxx(f, app, area)`
- Modal variant: `PascalCase` (e.g. `CommitTree`)

## Checklist

- [ ] Added variant to `ActiveModal`
- [ ] Created `ui/modals/<name>.rs`
- [ ] Exported in `ui/modals/mod.rs`
- [ ] Registered size + dispatch in `ui/mod.rs`
- [ ] Added key handling in `apps/tui/src/app/events/handlers/`
- [ ] Wired handler dispatch in `apps/tui/src/app/events/handlers/mod.rs`
- [ ] Used `app.theme()` for all colors
- [ ] Added bilingual strings to locales and used `t!` macro
- [ ] No comments added to source code

## Example Modals to Copy From

- `manual_commit.rs` (simple text input)
- `commit_tree.rs` (list + diff preview)
- `github_quick_view.rs` (viewer with highlighting)

Copy the structure from these files when creating new ones.

---

**Last updated**: 2026-07-04
