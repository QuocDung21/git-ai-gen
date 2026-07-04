use crate::app::App;
use crate::models::AmendStep;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

pub fn render_amend_commit(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let content = match &app.amend_step {
        AmendStep::Edit => {
            let display_msg = if app.amend_message.len() > 70 {
                format!("{}...", &app.amend_message[..67])
            } else {
                app.amend_message.clone()
            };
            vec![
                Line::from(""),
                Line::from(vec![Span::styled(
                    t!("amend_edit_heading").to_string(),
                    Style::default()
                        .fg(theme.orange)
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    t!("amend_edit_warning").to_string(),
                    Style::default()
                        .fg(theme.red)
                        .add_modifier(Modifier::ITALIC),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    t!("amend_edit_msg_label").to_string(),
                    Style::default().fg(theme.border),
                )]),
                Line::from(vec![
                    Span::styled("  ┌─── ", Style::default().fg(theme.orange)),
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
                    t!("amend_edit_type_hint").to_string(),
                    Style::default().fg(theme.border),
                )]),
            ]
        }
        AmendStep::Pushing => {
            vec![
                Line::from(""),
                Line::from(vec![Span::styled(
                    t!("amend_pushing_label").to_string(),
                    Style::default()
                        .fg(theme.yellow)
                        .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
                )]),
                Line::from(""),
            ]
        }
        AmendStep::Done(result) => {
            let color = if result.starts_with("✅") {
                theme.green
            } else {
                theme.red
            };
            vec![
                Line::from(""),
                Line::from(vec![Span::styled(
                    result.clone(),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    t!("amend_done_close_hint").to_string(),
                    Style::default().fg(theme.border),
                )]),
            ]
        }
    };

    let block = Block::default()
        .title(Span::styled(
            " ✏️  AMEND COMMIT ",
            Style::default()
                .fg(theme.orange)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.orange))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}
