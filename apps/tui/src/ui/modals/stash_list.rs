use crate::app::App;
use crate::models::{StashAction, StashStep};
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

pub fn render_stash_list(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("stash_list_heading").to_string(),
            Style::default()
                .fg(theme.orange)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    match &app.stash_step {
        StashStep::List => {
            if app.stash_entries.is_empty() {
                content.push(Line::from(vec![Span::styled(
                    t!("stash_list_empty").to_string(),
                    Style::default()
                        .fg(theme.border)
                        .add_modifier(Modifier::ITALIC),
                )]));
                content.push(Line::from(""));
                content.push(Line::from(vec![Span::styled(
                    t!("stash_list_new_stash_hint").to_string(),
                    Style::default().fg(theme.cyan),
                )]));
            } else {
                for (i, entry) in app.stash_entries.iter().enumerate() {
                    let is_sel = i == app.selected_stash_index;
                    let cursor = if is_sel { " ▶ " } else { "   " };
                    let row_style = if is_sel {
                        Style::default()
                            .fg(theme.fg)
                            .bg(theme.select_bg)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(theme.fg)
                    };
                    content.push(Line::from(vec![
                        Span::styled(
                            cursor,
                            Style::default()
                                .fg(theme.orange)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("[{}] ", entry.index),
                            Style::default()
                                .fg(theme.purple)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("({}) ", entry.branch),
                            Style::default().fg(theme.cyan),
                        ),
                        Span::styled(entry.message.clone(), row_style),
                    ]));
                }
                content.push(Line::from(""));
                content.push(Line::from(vec![Span::styled(
                    t!("stash_list_actions_hint").to_string(),
                    Style::default()
                        .fg(theme.orange)
                        .add_modifier(Modifier::BOLD),
                )]));
            }
        }
        StashStep::Confirm(idx, action) => {
            let action_str = match action {
                StashAction::Pop => t!("stash_confirm_pop_action").to_string(),
                StashAction::Apply => t!("stash_confirm_apply_action").to_string(),
                StashAction::Drop => t!("stash_confirm_drop_action").to_string(),
            };
            let action_color = match action {
                StashAction::Drop => theme.red,
                _ => theme.green,
            };
            content.push(Line::from(vec![Span::styled(
                format!("  ⚠️  Xác nhận {} stash@{{{}}}?", action_str, idx),
                Style::default()
                    .fg(action_color)
                    .add_modifier(Modifier::BOLD),
            )]));
            content.push(Line::from(""));
            content.push(Line::from(vec![
                Span::styled(
                    t!("stash_confirm_confirm_btn").to_string(),
                    Style::default()
                        .fg(theme.bg)
                        .bg(action_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("    ", Style::default()),
                Span::styled(
                    t!("stash_confirm_cancel_btn").to_string(),
                    Style::default()
                        .fg(theme.fg)
                        .bg(theme.select_bg)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        }
    }

    let block = Block::default()
        .title(Span::styled(
            " 📦 STASH MANAGER ",
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
