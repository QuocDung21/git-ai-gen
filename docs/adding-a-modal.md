# Adding a New Modal (Recommended Pattern)

This document describes the **preferred modern way** to add a new modal in git-ai.

## Why We Split Modals

- `ui/modals/confirm.rs` has grown too large (>2500 lines).
- Each modal should live in its own file for better maintainability and lower token usage for AI agents.
- Follow the principle: **One concept = one file**.

## Step-by-Step Guide

### 1. Add the Modal Variant

Edit `src/app/models.rs`:

```rust
pub enum ActiveModal {
    // ... existing variants

    MyNewModal,           // Add your variant here
}
```

### 2. (Optional) Add State to App

If your modal needs state, add it in `src/app/mod.rs`:

```rust
pub struct App {
    // ... existing fields

    pub my_modal_input: String,
    pub my_modal_selected_index: usize,
}
```

Initialize it in `App::new()`.

### 3. Create the Modal File

Create a new file: `src/ui/modals/my_new_modal.rs`

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

pub fn render_my_new_modal(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";

    f.render_widget(Clear, area);

    let content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi { "Tiêu đề Modal" } else { "Modal Title" },
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

Edit `src/ui/modals/mod.rs`:

```rust
pub use confirm::{
    // ... existing exports
    render_my_new_modal,
};
```

### 5. Register Size and Dispatch

Edit `src/ui/mod.rs`:

```rust
// In the size calculation match
ActiveModal::MyNewModal => modals::centered_rect(60, 50, f.size()),

// In the render dispatch match
ActiveModal::MyNewModal => modals::render_my_new_modal(f, app, area),
```

### 6. Handle Keyboard Input

Edit `src/app/events.rs` inside the big `match &app.active_modal`:

```rust
ActiveModal::MyNewModal => {
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
    continue;
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
- [ ] Added key handling in `events.rs`
- [ ] Used `app.theme()` for all colors
- [ ] Added bilingual strings (`is_vi`)
- [ ] No comments added to source code

## Example Modals to Copy From

- `manual_commit.rs` (simple text input)
- `commit_tree.rs` (list + diff preview)
- `kilo_model_select.rs` (searchable list)

Copy the structure from these files when creating new ones.

---

**Last updated**: 2026-05-22
