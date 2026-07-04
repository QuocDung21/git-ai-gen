use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

pub fn render_workspace_path_input(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let display_msg = if app.workspace_path_input.len() > 70 {
        format!("{}...", &app.workspace_path_input[..67])
    } else {
        app.workspace_path_input.clone()
    };

    let content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("workspace_path_heading").to_string(),
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("workspace_path_enter_label").to_string(),
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
            t!("workspace_path_type_hint").to_string(),
            Style::default().fg(theme.border),
        )]),
    ];

    let block = Block::default()
        .title(Span::styled(
            t!("workspace_path_title").to_string(),
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
