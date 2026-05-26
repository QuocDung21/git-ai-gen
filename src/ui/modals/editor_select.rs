use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_editor_select(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "Chọn trình biên dịch/ứng dụng mở mặc định:"
            } else {
                "Select default open application/editor:"
            },
            Style::default().fg(theme.fg).add_modifier(Modifier::ITALIC),
        )]),
        Line::from(""),
    ];

    let items = vec![
        ("VS Code (code)", 0),
        ("Cursor (cursor)", 1),
        ("Zed (zed)", 2),
        ("Sublime Text (subl)", 3),
        (if is_vi { "Mặc định hệ thống" } else { "System Default" }, 4),
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
        if is_vi {
            "Dùng ↑/↓ hoặc j/k để di chuyển, Enter để chọn."
        } else {
            "Use ↑/↓ or j/k to navigate, Enter to select."
        },
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
             if is_vi { " ⚙️ CHỌN TRÌNH BIÊN DỊCH MẶC ĐỊNH " } else { " ⚙️ SELECT DEFAULT EDITOR " },
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
