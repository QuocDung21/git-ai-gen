use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render_header(f: &mut Frame, app: &App, header_area: Rect, badge_area: Rect) {
    let is_vi = app.current_lang == "vi";
    let theme = app.theme();

    // --- TỐI ƯU 1: Helper tạo Block để không phải lặp lại code ---
    let create_block = |color: Color| {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color))
            .border_type(BorderType::Rounded)
    };

    let [left_header, right_header] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .areas(header_area);

    // 🎯 ĐÃ ĐỔI: Chữ ASCII Art được thiết kế lại thành "GIT-CHILL"
    let brand_lines = vec![
        Line::from(Span::styled(
            "  ██████  ██ ████████       ██████  ██   ██ ██ ██      ██      ",
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            " ██       ██    ██         ██       ██   ██ ██ ██      ██      ",
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            " ██   ███ ██    ██   ████  ██       ███████ ██ ██      ██      ",
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            " ██    ██ ██    ██         ██       ██   ██ ██ ██      ██      ",
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "  ██████  ██    ██          ██████  ██   ██ ██ ███████ ███████ ",
            Style::default()
                .fg(theme.yellow)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    f.render_widget(
        Paragraph::new(brand_lines)
            .alignment(Alignment::Center)
            .block(create_block(theme.purple)),
        left_header,
    );

    // --- RIGHT: SYSTEM SETTINGS ---
    let (lang_display, help_hint) = if is_vi {
        ("Tiếng Việt", "Nhấn '?' hoặc 'h'")
    } else {
        ("English", "Press '?' or 'h'")
    };

    let right_header_text = vec![
        // 🎯 ĐÃ ĐỔI: Chữ tiêu đề hệ thống góc phải thành GIT-CHILL
        Line::from(Span::styled(
            " 🤖 ULTIMATE GIT-CHILL ",
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::styled(" ⚡ AI: ", Style::default().fg(theme.border)),
            Span::styled(
                "ONLINE",
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(" 🌎 Lang: ", Style::default().fg(theme.border)),
            Span::styled(
                lang_display,
                Style::default()
                    .fg(theme.yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(" 📦 Ver:  ", Style::default().fg(theme.border)),
            Span::styled("v3.0.0", Style::default().fg(theme.cyan)),
        ]),
        Line::from(vec![
            Span::styled(" 💡 Help: ", Style::default().fg(theme.border)),
            Span::styled(
                help_hint,
                Style::default()
                    .fg(theme.orange)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]),
    ];

    f.render_widget(
        Paragraph::new(right_header_text).block(create_block(theme.cyan)),
        right_header,
    );

    // 2. WORKSPACE BADGE BAR
    let [path_area, stats_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .areas(badge_area);

    // Left: Workspace Path
    let dir_text = Line::from(vec![
        Span::styled(
            " 📂 WORKSPACE: ",
            Style::default()
                .fg(theme.border)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            &app.current_dir,
            Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
        ),
    ]);

    f.render_widget(
        Paragraph::new(dir_text).block(create_block(theme.border)),
        path_area,
    );

    // Right: Git Stats (Localized Labels)
    let (lbl_staged, lbl_unstaged, lbl_untracked) = if is_vi {
        ("🟢 Đã Stage: ", "🟡 Chưa Stage: ", "🟣 Chưa theo dõi: ")
    } else {
        ("🟢 Staged: ", "🟡 Unstaged: ", "🟣 Untracked: ")
    };

    let stats_text = Line::from(vec![
        Span::styled(
            format!("🌿 {} ", app.current_branch),
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(theme.border)),
        Span::styled(lbl_staged, Style::default().fg(theme.fg)),
        Span::styled(
            app.staged_count.to_string(),
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(format!("  {}", lbl_unstaged), Style::default().fg(theme.fg)),
        Span::styled(
            app.unstaged_count.to_string(),
            Style::default()
                .fg(theme.yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("  {}", lbl_untracked),
            Style::default().fg(theme.fg),
        ),
        Span::styled(
            app.untracked_count.to_string(),
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    f.render_widget(
        Paragraph::new(stats_text)
            .alignment(Alignment::Center)
            .block(create_block(theme.cyan)),
        stats_area,
    );
}
