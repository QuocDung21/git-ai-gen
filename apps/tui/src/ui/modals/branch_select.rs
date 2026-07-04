use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

pub fn render_branch_select(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("branch_select_heading").to_string(),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.branches.is_empty() {
        content.push(Line::from(vec![Span::styled(
            t!("branch_select_empty").to_string(),
            Style::default()
                .fg(theme.border)
                .add_modifier(Modifier::ITALIC),
        )]));
    } else {
        for (i, branch) in app.branches.iter().enumerate() {
            let is_selected = i == app.selected_branch_index;
            let is_active = !branch.is_remote && branch.name == app.current_branch;
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

            let branch_style = if is_selected {
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.select_bg)
                    .add_modifier(Modifier::BOLD)
            } else if is_active {
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD)
            } else if branch.is_remote {
                Style::default().fg(theme.orange)
            } else {
                Style::default().fg(theme.fg)
            };

            let active_badge = if is_active {
                Span::styled(
                    t!("branch_select_active_badge").to_string(),
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::ITALIC),
                )
            } else if branch.is_remote {
                Span::styled(
                    " (Remote) ",
                    Style::default()
                        .fg(theme.orange)
                        .add_modifier(Modifier::ITALIC),
                )
            } else {
                Span::styled("", Style::default())
            };

            let prefix = if branch.is_remote {
                "🌍 "
            } else if is_active {
                "★ "
            } else {
                "☆ "
            };

            let prefix_span = Span::styled(
                prefix,
                if branch.is_remote {
                    Style::default().fg(theme.orange)
                } else if is_active {
                    Style::default().fg(theme.green)
                } else {
                    Style::default().fg(theme.border)
                },
            );

            content.push(Line::from(vec![
                cursor,
                prefix_span,
                Span::styled(branch.name.clone(), branch_style),
                active_badge,
            ]));
        }
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        t!("branch_select_navigate").to_string(),
        Style::default()
            .fg(theme.orange)
            .add_modifier(Modifier::BOLD),
    )]));
    content.push(Line::from(vec![Span::styled(
        t!("branch_select_merge_hint").to_string(),
        Style::default()
            .fg(theme.green)
            .add_modifier(Modifier::BOLD),
    )]));
    content.push(Line::from(vec![Span::styled(
        t!("branch_select_create_hint").to_string(),
        Style::default()
            .fg(theme.purple)
            .add_modifier(Modifier::BOLD),
    )]));
    content.push(Line::from(vec![Span::styled(
        t!("branch_select_delete_hint").to_string(),
        Style::default().fg(theme.red).add_modifier(Modifier::BOLD),
    )]));
    content.push(Line::from(vec![Span::styled(
        t!("branch_select_cancel_hint").to_string(),
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            t!("branch_select_title").to_string(),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.cyan))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Center)
        .block(block);

    f.render_widget(paragraph, area);
}
