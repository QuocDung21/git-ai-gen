use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

pub fn render_new_branch_input(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let display_msg = if app.new_branch_name.len() > 70 {
        format!("{}...", &app.new_branch_name[..67])
    } else {
        app.new_branch_name.clone()
    };

    let content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("new_branch_heading").to_string(),
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("new_branch_enter_label").to_string(),
            Style::default().fg(theme.border),
        )]),
        Line::from(vec![
            Span::styled("  ┌─── ", Style::default().fg(theme.purple)),
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
            t!("new_branch_type_hint").to_string(),
            Style::default().fg(theme.border),
        )]),
    ];

    let block = Block::default()
        .title(Span::styled(
            t!("new_branch_title").to_string(),
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.purple))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}
