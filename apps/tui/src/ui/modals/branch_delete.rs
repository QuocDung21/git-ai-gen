use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

use crate::app::App;

pub fn render_branch_delete_confirm(f: &mut Frame, app: &App, branch_name: &str, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("branch_delete_warning_title").to_string(),
            Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("branch_delete_question").to_string(),
            Style::default().fg(theme.fg),
        )]),
        Line::from(vec![Span::styled(
            format!("👉 {} ", branch_name),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("branch_delete_irreversible").to_string(),
            Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                t!("branch_delete_confirm_btn").to_string(),
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("      ", Style::default()),
            Span::styled(
                t!("branch_delete_cancel_btn").to_string(),
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.red)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let block = Block::default()
        .title(Span::styled(
            t!("branch_delete_title").to_string(),
            Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.red))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Center)
        .block(block);

    f.render_widget(paragraph, area);
}
