use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

use crate::app::App;

pub fn render_editor_select(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("editor_select_prompt").to_string(),
            Style::default().fg(theme.fg).add_modifier(Modifier::ITALIC),
        )]),
        Line::from(""),
    ];

    let items = vec![
        ("VS Code (code)".to_string(), 0),
        ("Cursor (cursor)".to_string(), 1),
        ("Zed (zed)".to_string(), 2),
        ("Sublime Text (subl)".to_string(), 3),
        (t!("editor_select_default").to_string(), 4),
    ];

    let current_selection = match app.editor.as_str() {
        "code" => 0,
        "cursor" => 1,
        "zed" => 2,
        "subl" => 3,
        _ => 4,
    };

    for (label, idx) in items {
        let is_hovered = idx == app.selected_editor_index;
        let is_currently_active = current_selection == idx;

        let cursor = if is_hovered {
            Span::styled(
                " ▶ ",
                Style::default()
                    .fg(theme.purple)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled("   ", Style::default())
        };

        let active_badge = if is_currently_active {
            Span::styled(
                " (Active) ",
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::ITALIC),
            )
        } else {
            Span::styled("", Style::default())
        };

        let item_style = if is_hovered {
            Style::default()
                .fg(theme.fg)
                .bg(theme.select_bg)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg)
        };

        content.push(Line::from(vec![
            cursor,
            Span::styled(
                format!(" [{}] ", idx + 1),
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled(label, item_style),
            active_badge,
        ]));
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        t!("editor_select_navigate").to_string(),
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            t!("editor_select_title").to_string(),
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.purple))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Center)
        .block(block);

    f.render_widget(paragraph, area);
}
