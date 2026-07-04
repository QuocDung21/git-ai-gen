use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

pub fn render_merge_confirm(f: &mut Frame, app: &App, branch_to_merge: &str, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("merge_confirm_heading").to_string(),
            Style::default()
                .fg(theme.orange)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                t!("merge_confirm_merge_verb").to_string(),
                Style::default().fg(theme.fg),
            ),
            Span::styled(
                format!("\"{}\"", branch_to_merge),
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                t!("merge_confirm_into").to_string(),
                Style::default().fg(theme.fg),
            ),
            Span::styled(
                format!("\"{}\"", app.current_branch),
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("merge_confirm_conflict_note1").to_string(),
            Style::default().fg(theme.red),
        )]),
        Line::from(vec![Span::styled(
            t!("merge_confirm_conflict_note2").to_string(),
            Style::default().fg(theme.red),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                t!("merge_confirm_confirm_btn").to_string(),
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("      ", Style::default()),
            Span::styled(
                t!("merge_confirm_cancel_btn").to_string(),
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.red)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let block = Block::default()
        .title(Span::styled(
            t!("merge_confirm_title").to_string(),
            Style::default()
                .fg(theme.orange)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.orange))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Center)
        .block(block);

    f.render_widget(paragraph, area);
}
