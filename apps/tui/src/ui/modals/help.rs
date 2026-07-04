use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render_help_modal(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            app.locales.help_modal_header.clone(),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    let shortcut_groups = vec![
        (
            app.locales.help_group_navigation.as_str(),
            vec![
                ("↑/↓ / j/k", app.locales.help_nav_select.as_str()),
                ("PgUp/PgDn", app.locales.help_nav_scroll.as_str()),
            ],
        ),
        (
            app.locales.help_group_git.as_str(),
            vec![
                ("Space", app.locales.help_git_stage.as_str()),
                ("Backspace", app.locales.help_git_revert.as_str()),
                ("a", app.locales.help_git_stage_all.as_str()),
                ("u", app.locales.help_git_unstage_all.as_str()),
                ("b", app.locales.help_git_branch.as_str()),
                ("v", app.locales.help_git_timeline.as_str()),
                ("f", app.locales.help_git_fetch.as_str()),
                ("p", app.locales.help_git_pull.as_str()),
                ("i", app.locales.help_git_remote.as_str()),
                ("d", app.locales.help_git_copy_diff.as_str()),
                ("x", app.locales.help_git_ai_prompt.as_str()),
                ("g", app.locales.help_git_go.as_str()),
            ],
        ),
        (
            app.locales.help_group_sys.as_str(),
            vec![
                ("o", app.locales.help_sys_vscode.as_str()),
                ("w", app.locales.help_sys_workspace.as_str()),
                ("e", app.locales.help_sys_lang_stats.as_str()),
                ("l", app.locales.help_sys_lang.as_str()),
                ("t", app.locales.help_sys_theme.as_str()),
                ("r", app.locales.help_sys_reset.as_str()),
                ("q / Esc", app.locales.help_sys_quit.as_str()),
            ],
        ),
    ];

    for (group_title, items) in shortcut_groups {
        content.push(Line::from(vec![Span::styled(
            format!("  ■ {}", group_title),
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )]));
        for (key, desc) in items {
            content.push(Line::from(vec![
                Span::styled("   ⚡ [", Style::default().fg(theme.border)),
                Span::styled(
                    key,
                    Style::default()
                        .fg(theme.green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("] : ", Style::default().fg(theme.border)),
                Span::styled(desc, Style::default().fg(theme.fg)),
            ]));
        }
        content.push(Line::from(""));
    }

    content.push(Line::from(vec![Span::styled(
        app.locales.help_modal_close.clone(),
        Style::default()
            .fg(theme.orange)
            .add_modifier(Modifier::BOLD),
    )]));

    let block = Block::default()
        .title(Span::styled(
            " SYSTEM MANUAL ",
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
