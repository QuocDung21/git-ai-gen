use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

use crate::app::App;

pub fn render_manual_commit(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let display_msg = if app.manual_commit_message.len() > 70 {
        format!("{}...", &app.manual_commit_message[..67])
    } else {
        app.manual_commit_message.clone()
    };

    let content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("manual_commit_title").to_string(),
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("manual_commit_prompt").to_string(),
            Style::default().fg(theme.border),
        )]),
        Line::from(vec![
            Span::styled("  ┌─── ", Style::default().fg(theme.green)),
            Span::styled(
                format!("{}_", display_msg),
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.bg)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("manual_commit_actions").to_string(),
            Style::default().fg(theme.border),
        )]),
    ];

    let block = Block::default()
        .title(Span::styled(
            " ✍️ MANUAL COMMIT ",
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.green))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}
