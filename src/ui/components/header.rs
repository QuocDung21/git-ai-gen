use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use crate::app::App;

pub fn render_header(f: &mut Frame, app: &App, header_area: Rect, badge_area: Rect) {
    let is_vi = app.current_lang == "vi";

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
                .fg(Color::Rgb(189, 147, 249))
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            " ██╔════╝ ██║╚══██╔══╝██═══██╔════╝██║",
            Style::default()
                .fg(Color::Rgb(255, 121, 198))
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            " ██║  ███╗██║   ██║   ╚█████╔█████╗ ██║",
            Style::default()
                .fg(Color::Rgb(139, 233, 253))
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            " ██║   ██║██║   ██║   ██╔═══██╔═══╝ ██║",
            Style::default()
                .fg(Color::Rgb(80, 250, 123))
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            " ╚██████╔╝██║   ██║   ╚██████╔███████║",
            Style::default()
                .fg(Color::Rgb(241, 250, 140))
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "  ╚═════╝ ╚═╝   ╚═╝    ╚═════╝╚══════╝",
            Style::default()
                .fg(Color::Rgb(255, 184, 108))
                .add_modifier(Modifier::BOLD),
        )]),
    ];

    let brand_widget = Paragraph::new(brand_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(189, 147, 249)))
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
                .fg(Color::Rgb(189, 147, 249))
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled(
                " ⚡  AI Status: ",
                Style::default().fg(Color::Rgb(98, 114, 164)),
            ),
            Span::styled(
                "ONLINE",
                Style::default()
                    .fg(Color::Rgb(80, 250, 123))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                " 🌎  Language:  ",
                Style::default().fg(Color::Rgb(98, 114, 164)),
            ),
            Span::styled(
                lang_display,
                Style::default()
                    .fg(Color::Rgb(241, 250, 140))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                " 📦  Version:   ",
                Style::default().fg(Color::Rgb(98, 114, 164)),
            ),
            Span::styled("v3.0.0", Style::default().fg(Color::Rgb(139, 233, 253))),
        ]),
        Line::from(vec![
            Span::styled(
                " 💡  Help:      ",
                Style::default().fg(Color::Rgb(98, 114, 164)),
            ),
            Span::styled(
                "Press '?' or 'h'",
                Style::default()
                    .fg(Color::Rgb(255, 184, 108))
                    .add_modifier(Modifier::ITALIC),
            ),
        ]),
    ];

    let right_header_widget = Paragraph::new(right_header_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(139, 233, 253)))
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
                .fg(Color::Rgb(98, 114, 164))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            &app.current_dir,
            Style::default()
                .fg(Color::Rgb(248, 248, 242))
                .add_modifier(Modifier::BOLD),
        ),
    ]);
    let dir_block = Paragraph::new(dir_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(98, 114, 164)))
            .border_type(BorderType::Rounded),
    );
    f.render_widget(dir_block, badge_chunks[0]);

    // Right Panel: Git Branch & Changes breakdown stats
    let stats_text = if is_vi {
        Line::from(vec![
            Span::styled(" 🌿 ", Style::default().fg(Color::Rgb(80, 250, 123))),
            Span::styled(
                &app.current_branch,
                Style::default()
                    .fg(Color::Rgb(80, 250, 123))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  |  🟢 Đã Stage: ",
                Style::default().fg(Color::Rgb(248, 248, 242)),
            ),
            Span::styled(
                format!("{}", app.staged_count),
                Style::default()
                    .fg(Color::Rgb(80, 250, 123))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  🟡 Chưa Stage: ",
                Style::default().fg(Color::Rgb(248, 248, 242)),
            ),
            Span::styled(
                format!("{}", app.unstaged_count),
                Style::default()
                    .fg(Color::Rgb(241, 250, 140))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  🟣 Chưa theo dõi: ",
                Style::default().fg(Color::Rgb(248, 248, 242)),
            ),
            Span::styled(
                format!("{}", app.untracked_count),
                Style::default()
                    .fg(Color::Rgb(189, 147, 249))
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled(" 🌿 ", Style::default().fg(Color::Rgb(80, 250, 123))),
            Span::styled(
                &app.current_branch,
                Style::default()
                    .fg(Color::Rgb(80, 250, 123))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  |  🟢 Staged: ",
                Style::default().fg(Color::Rgb(248, 248, 242)),
            ),
            Span::styled(
                format!("{}", app.staged_count),
                Style::default()
                    .fg(Color::Rgb(80, 250, 123))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  🟡 Unstaged: ",
                Style::default().fg(Color::Rgb(248, 248, 242)),
            ),
            Span::styled(
                format!("{}", app.unstaged_count),
                Style::default()
                    .fg(Color::Rgb(241, 250, 140))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  🟣 Untracked: ",
                Style::default().fg(Color::Rgb(248, 248, 242)),
            ),
            Span::styled(
                format!("{}", app.untracked_count),
                Style::default()
                    .fg(Color::Rgb(189, 147, 249))
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    };

    let stats_widget = Paragraph::new(stats_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(139, 233, 253)))
            .border_type(BorderType::Rounded),
    );
    f.render_widget(stats_widget, badge_chunks[1]);
}
