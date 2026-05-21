use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use crate::app::App;

pub fn render_header(f: &mut Frame, app: &App, header_area: Rect, badge_area: Rect) {
    let is_vi = app.current_lang == "vi";
    let theme = app.theme();

    // 1. SPLIT HEADER / BANNER
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(70), // Title and Neon ASCII Brand
                Constraint::Percentage(30), // System Details
            ]
            .as_ref(),
        )
        .split(header_area);

    // Left Header: Neon HSL Gradient Block Letter ASCII Logo ("GIT-AI")
    let brand_lines = vec![
        Line::from(vec![Span::styled(
            "  ██████╗ ██╗████████╗      ███████╗██╗",
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            " ██╔════╝ ██║╚══██╔══╝██═══██╔════╝██║",
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            " ██║  ███╗██║   ██║   ╚█████╔█████╗ ██║",
            Style::default()
                .fg(theme.cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            " ██║   ██║██║   ██║   ██╔═══██╔═══╝ ██║",
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            " ╚██████╔╝██║   ██║   ╚██████╔███████║",
            Style::default()
                .fg(theme.yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "  ╚═════╝ ╚═╝   ╚═╝    ╚═════╝╚══════╝",
            Style::default()
                .fg(theme.orange)
                .add_modifier(Modifier::BOLD),
        )]),
    ];

    let brand_widget = Paragraph::new(brand_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.purple))
            .border_type(BorderType::Rounded),
    );
    f.render_widget(brand_widget, header_chunks[0]);

    // Right Header: System Settings details
    let lang_display = if app.current_lang == "vi" {
        "Tiếng Việt"
    } else {
        "English"
    };

    let right_header_text = vec![
        Line::from(vec![Span::styled(
            " 🤖  ULTIMATE GIT-AI SYSTEM ",
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled(
                " ⚡  AI Status: ",
                Style::default().fg(theme.border),
            ),
            Span::styled(
                "ONLINE",
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                " 🌎  Language:  ",
                Style::default().fg(theme.border),
            ),
            Span::styled(
                lang_display,
                Style::default()
                    .fg(theme.yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                " 📦  Version:   ",
                Style::default().fg(theme.border),
            ),
            Span::styled("v3.0.0", Style::default().fg(theme.cyan)),
        ]),
        Line::from(vec![
            Span::styled(
                " 💡  Help:      ",
                Style::default().fg(theme.border),
            ),
            Span::styled(
                "Press '?' or 'h'",
                Style::default()
                    .fg(theme.orange)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]),
    ];

    let right_header_widget = Paragraph::new(right_header_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.cyan))
            .border_type(BorderType::Rounded),
    );
    f.render_widget(right_header_widget, header_chunks[1]);

    // 2. WORKSPACE BADGE BAR (SPLIT IN TWO)
    let badge_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(55), // Workspace dir path
                Constraint::Percentage(45), // Branch name & counts
            ]
            .as_ref(),
        )
        .split(badge_area);

    // Left Panel: Workspace Directory Path
    let dir_text = Line::from(vec![
        Span::styled(
            "  📂  WORKSPACE: ",
            Style::default()
                .fg(theme.border)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            &app.current_dir,
            Style::default()
                .fg(theme.fg)
                .add_modifier(Modifier::BOLD),
        ),
    ]);
    let dir_block = Paragraph::new(dir_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .border_type(BorderType::Rounded),
    );
    f.render_widget(dir_block, badge_chunks[0]);

    // Right Panel: Git Branch & Changes breakdown stats
    let stats_text = if is_vi {
        Line::from(vec![
            Span::styled(" 🌿 ", Style::default().fg(theme.green)),
            Span::styled(
                &app.current_branch,
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  |  🟢 Đã Stage: ",
                Style::default().fg(theme.fg),
            ),
            Span::styled(
                format!("{}", app.staged_count),
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  🟡 Chưa Stage: ",
                Style::default().fg(theme.fg),
            ),
            Span::styled(
                format!("{}", app.unstaged_count),
                Style::default()
                    .fg(theme.yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  🟣 Chưa theo dõi: ",
                Style::default().fg(theme.fg),
            ),
            Span::styled(
                format!("{}", app.untracked_count),
                Style::default()
                    .fg(theme.purple)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled(" 🌿 ", Style::default().fg(theme.green)),
            Span::styled(
                &app.current_branch,
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  |  🟢 Staged: ",
                Style::default().fg(theme.fg),
            ),
            Span::styled(
                format!("{}", app.staged_count),
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  🟡 Unstaged: ",
                Style::default().fg(theme.fg),
            ),
            Span::styled(
                format!("{}", app.unstaged_count),
                Style::default()
                    .fg(theme.yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  🟣 Untracked: ",
                Style::default().fg(theme.fg),
            ),
            Span::styled(
                format!("{}", app.untracked_count),
                Style::default()
                    .fg(theme.purple)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    };

    let stats_widget = Paragraph::new(stats_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.cyan))
            .border_type(BorderType::Rounded),
    );
    f.render_widget(stats_widget, badge_chunks[1]);
}
