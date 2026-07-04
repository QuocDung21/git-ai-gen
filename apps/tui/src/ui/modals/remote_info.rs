use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

pub fn render_remote_info(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let ahead_color = if app.ahead_count > 0 {
        theme.green
    } else {
        theme.border
    };
    let behind_color = if app.behind_count > 0 {
        theme.red
    } else {
        theme.border
    };

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("remote_info_heading").to_string(),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  🌿 Branch:   ", Style::default().fg(theme.border)),
            Span::styled(
                app.current_branch.clone(),
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  🔗 Tracking: ", Style::default().fg(theme.border)),
            Span::styled(
                app.remote_tracking.clone(),
                Style::default()
                    .fg(theme.orange)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  📡 Remote:   ", Style::default().fg(theme.border)),
            Span::styled(app.remote_url.clone(), Style::default().fg(theme.purple)),
        ]),
    ];

    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        t!("remote_info_remotes_label").to_string(),
        Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
    )]));
    if app.remotes.is_empty() {
        content.push(Line::from(vec![Span::styled(
            t!("remote_info_no_remotes").to_string(),
            Style::default()
                .fg(theme.border)
                .add_modifier(Modifier::ITALIC),
        )]));
    } else {
        for remote in &app.remotes {
            content.push(Line::from(vec![
                Span::styled("    • ", Style::default().fg(theme.border)),
                Span::styled(
                    remote.name.clone(),
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" → ", Style::default().fg(theme.yellow)),
                Span::styled(remote.url.clone(), Style::default().fg(theme.purple)),
            ]));
        }
    }

    content.push(Line::from(""));
    content.push(Line::from(vec![
        Span::styled("  ↑ Ahead:  ", Style::default().fg(theme.border)),
        Span::styled(
            format!("{} commit(s) ahead of remote", app.ahead_count),
            Style::default()
                .fg(ahead_color)
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    content.push(Line::from(vec![
        Span::styled("  ↓ Behind: ", Style::default().fg(theme.border)),
        Span::styled(
            format!("{} commit(s) behind remote", app.behind_count),
            Style::default()
                .fg(behind_color)
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        if app.ahead_count > 0 && app.behind_count == 0 {
            t!("remote_info_can_push").to_string()
        } else if app.behind_count > 0 {
            t!("remote_info_pull_first").to_string()
        } else {
            t!("remote_info_in_sync").to_string()
        },
        Style::default()
            .fg(theme.yellow)
            .add_modifier(Modifier::ITALIC),
    )]));
    content.push(Line::from(""));
    content.push(Line::from(vec![Span::styled(
        t!("remote_info_close_hint").to_string(),
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            " REMOTE INFO ",
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
