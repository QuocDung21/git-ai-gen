use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

pub fn render_git_menu(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let ai_commit_label = if app.kilo_ai_enabled {
        t!("git_menu_ai_commit_enabled").to_string()
    } else {
        t!("git_menu_ai_commit_disabled").to_string()
    };

    let actions: Vec<(String, Vec<(String, char)>)> = vec![
        (
            t!("git_menu_group_commit").to_string(),
            vec![
                (ai_commit_label, 'g'),
                (t!("git_menu_manual_commit").to_string(), 'c'),
                (t!("git_menu_amend").to_string(), 'm'),
            ],
        ),
        (
            t!("git_menu_group_remote").to_string(),
            vec![
                (t!("git_menu_fetch").to_string(), 'f'),
                (t!("git_menu_pull").to_string(), 'p'),
                (t!("git_menu_push").to_string(), 'u'),
                (t!("git_menu_remote_info").to_string(), 'i'),
            ],
        ),
        (
            t!("git_menu_group_other").to_string(),
            vec![
                (t!("git_menu_branch").to_string(), 'b'),
                (t!("git_menu_stash").to_string(), 's'),
                (t!("git_menu_tree").to_string(), 't'),
                (t!("git_menu_history").to_string(), 'v'),
                (t!("git_menu_feature").to_string(), 'e'),
                (t!("git_menu_download").to_string(), 'n'),
                (t!("git_menu_clear_trash").to_string(), 'x'),
                (t!("git_menu_settings").to_string(), 'y'),
            ],
        ),
    ];

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            t!("git_menu_heading").to_string(),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    let mut idx = 0;
    for (group, items) in &actions {
        content.push(Line::from(vec![Span::styled(
            format!("  ■ {}", group),
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )]));

        for (name, key) in items {
            let is_selected = idx == app.selected_git_action;

            let prefix = if is_selected {
                Span::styled(
                    " ▶ ",
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("   ", Style::default())
            };

            let style = if is_selected {
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.select_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg)
            };

            content.push(Line::from(vec![
                prefix,
                Span::styled(format!("[{}] {}", key, name), style),
            ]));
            idx += 1;
        }
        content.push(Line::from(""));
    }

    content.push(Line::from(vec![Span::styled(
        t!("git_menu_navigate_hint").to_string(),
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            " 🛠️ GIT MENU ",
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
