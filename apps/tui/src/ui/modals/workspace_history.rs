use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

pub fn render_workspace_history(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("workspace_history_heading").to_string(),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.workspace_history.is_empty() {
        content.push(Line::from(vec![Span::styled(
            t!("workspace_history_empty").to_string(),
            Style::default()
                .fg(theme.border)
                .add_modifier(Modifier::ITALIC),
        )]));
    } else {
        for (i, path) in app.workspace_history.iter().enumerate() {
            let is_selected = i == app.selected_workspace_index;
            let is_active = *path == app.current_dir;

            let cursor = if is_selected {
                Span::styled(
                    " ▶ ",
                    Style::default()
                        .fg(theme.purple)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("   ", Style::default())
            };

            let icon = if is_active { "★ " } else { "☆ " };
            let icon_span = Span::styled(
                icon,
                if is_active {
                    Style::default().fg(theme.green)
                } else {
                    Style::default().fg(theme.border)
                },
            );

            let display_path = {
                let parts: Vec<&str> = path.rsplitn(3, '/').collect();
                if parts.len() >= 2 {
                    format!(
                        ".../{}",
                        parts
                            .iter()
                            .rev()
                            .skip(1)
                            .cloned()
                            .collect::<Vec<&str>>()
                            .join("/")
                    )
                } else {
                    path.clone()
                }
            };

            let path_style = if is_selected {
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.select_bg)
                    .add_modifier(Modifier::BOLD)
            } else if is_active {
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg)
            };

            let active_badge = if is_active {
                Span::styled(
                    t!("workspace_history_active_badge").to_string(),
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::ITALIC),
                )
            } else {
                Span::styled("", Style::default())
            };

            content.push(Line::from(vec![
                cursor,
                icon_span,
                Span::styled(display_path, path_style),
                active_badge,
            ]));
        }
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        t!("workspace_history_actions_hint").to_string(),
        Style::default()
            .fg(theme.orange)
            .add_modifier(Modifier::BOLD),
    )]));

    let block = Block::default()
        .title(Span::styled(
            t!("workspace_history_title").to_string(),
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
