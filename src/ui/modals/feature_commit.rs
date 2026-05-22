use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_feature_commit(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let is_vi = app.current_lang == "vi";
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            if is_vi {
                "🧩 COMMIT THEO FEATURE"
            } else {
                "🧩 FEATURE COMMIT"
            },
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.feature_groups.is_empty() {
        content.push(Line::from(vec![Span::styled(
            if is_vi {
                "  Không có thay đổi để nhóm thành feature."
            } else {
                "  No changes to group into features."
            },
            Style::default().fg(theme.border),
        )]));
    } else {
        for (i, group) in app.feature_groups.iter().enumerate() {
            let is_selected = i == app.selected_feature_index;

            let cursor = if is_selected {
                Span::styled(" ▶ ", Style::default().fg(theme.green).add_modifier(Modifier::BOLD))
            } else {
                Span::styled("   ", Style::default())
            };

            let style = if is_selected {
                Style::default()
                    .fg(theme.select_fg)
                    .bg(theme.select_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg)
            };

            let line_text = format!("{} ({} files)", group.name, group.file_count);

            content.push(Line::from(vec![
                cursor,
                Span::styled(line_text, style),
            ]));
        }
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        if is_vi {
            "  [↑/↓] Chọn  [Enter] Stage feature  [Esc] Đóng"
        } else {
            "  [↑/↓] Select  [Enter] Stage feature  [Esc] Close"
        },
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            if is_vi {
                " 🧩 FEATURE COMMIT "
            } else {
                " 🧩 FEATURE COMMIT "
            },
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.cyan))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}
