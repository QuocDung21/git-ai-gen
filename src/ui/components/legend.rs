use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use rust_i18n::t;

pub fn render_legend(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    let mut legend_lines = vec![Line::from("")];
    let nav_items = if app.focus_diff {
        vec![
            (
                "↑/↓ / j/k",
                t!("legend_nav_line_scroll"),
                theme.yellow,
            ),
            (
                "d / u",
                t!("legend_nav_page_scroll"),
                theme.yellow,
            ),
            (
                "Tab / Esc",
                t!("legend_nav_return"),
                theme.purple,
            ),
        ]
    } else {
        vec![
            (
                "↑/↓ / j/k",
                t!("legend_nav_select"),
                theme.purple,
            ),
            (
                "Tab / l / →",
                t!("legend_nav_focus_diff"),
                theme.yellow,
            ),
            (
                "[ / ]",
                t!("legend_nav_quick_scroll"),
                theme.cyan,
            ),
        ]
    };

    let groups = vec![
        ("Navigation", nav_items),
        (
            "Git Operations",
            vec![
                (
                    "Space",
                    t!("legend_git_stage_all"),
                    theme.green,
                ),
                (
                    "Backspace",
                    t!("legend_git_revert"),
                    theme.red,
                ),
                (
                    "A",
                    t!("legend_git_stage_all"),
                    theme.green,
                ),
                (
                    "U",
                    t!("legend_git_unstage_all"),
                    theme.red,
                ),
                (
                    "B",
                    t!("legend_git_branch"),
                    theme.cyan,
                ),
                (
                    "V",
                    t!("legend_git_history"),
                    theme.yellow,
                ),
                (
                    "F",
                    t!("legend_git_fetch"),
                    theme.cyan,
                ),
                (
                    "P",
                    t!("legend_git_pull"),
                    theme.cyan,
                ),
                (
                    "I",
                    t!("legend_git_remote"),
                    theme.cyan,
                ),
                (
                    "D",
                    t!("legend_git_view_prompt"),
                    theme.yellow,
                ),
                (
                    "X",
                    t!("legend_git_view_prompt"),
                    theme.yellow,
                ),
                (
                    "G",
                    t!("legend_git_stage_all"),
                    theme.green,
                ),
                (
                    "N",
                    t!("legend_git_download"),
                    theme.cyan,
                ),
            ],
        ),
        (
            "System",
            vec![
                (
                    "O",
                    t!("legend_sys_vscode"),
                    theme.purple,
                ),
                (
                    "W",
                    t!("legend_sys_workspace"),
                    theme.cyan,
                ),
                (
                    "E",
                    t!("legend_sys_lang_stats"),
                    theme.purple,
                ),
                (
                    "L",
                    t!("legend_sys_lang"),
                    theme.purple,
                ),
                (
                    "T",
                    t!("legend_sys_theme"),
                    theme.purple,
                ),
                (
                    ",",
                    t!("legend_sys_settings"),
                    theme.purple,
                ),
                (
                    "R",
                    t!("legend_sys_reset"),
                    theme.red,
                ),
                (
                    "? / H",
                    t!("legend_sys_manual"),
                    theme.cyan,
                ),
                (
                    "Q",
                    t!("legend_sys_quit"),
                    theme.border,
                ),
            ],
        ),
    ];

    for (group_title, items) in groups {
        legend_lines.push(Line::from(vec![Span::styled(
            format!("  ■ {}", group_title),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )]));
        for (key, desc, color) in items {
            legend_lines.push(Line::from(vec![
                Span::styled("   ⚡ [", Style::default().fg(theme.border)),
                Span::styled(key, Style::default().fg(color).add_modifier(Modifier::BOLD)),
                Span::styled("]  ", Style::default().fg(theme.border)),
                Span::styled(desc.to_string(), Style::default().fg(theme.fg)),
            ]));
        }
        legend_lines.push(Line::from(""));
    }

    let legend_title = t!("legend_title");
    let legend_widget = Paragraph::new(legend_lines).block(
        Block::default()
            .title(Span::styled(
                legend_title.as_ref(),
                Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.cyan))
            .border_type(BorderType::Rounded),
    );
    f.render_widget(legend_widget, area);
}
