use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use rust_i18n::t;

pub fn render_badge_bar(f: &mut Frame, app: &App, badge_area: Rect) {
    let theme = app.theme();

    let create_block = |color: Color| {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color))
            .border_type(BorderType::Rounded)
    };

    let [path_area, stats_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .areas(badge_area);

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

    let lbl_staged = t!("header_staged");
    let lbl_unstaged = t!("header_unstaged");
    let lbl_untracked = t!("header_untracked");

    let stats_text = Line::from(vec![
        Span::styled(
            format!("🌿 {} ", app.current_branch),
            Style::default()
                .fg(theme.green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(theme.border)),
        Span::styled(lbl_staged.as_ref(), Style::default().fg(theme.fg)),
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

pub fn render_splash_screen(f: &mut Frame, app: &App) {
    let theme = app.theme();

    let create_block = |color: Color| {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color))
            .border_type(BorderType::Rounded)
    };

    let welcome_title = t!("header_welcome");
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.purple))
        .border_type(BorderType::Rounded)
        .title(Span::styled(
            welcome_title.as_ref(),
            Style::default().fg(theme.purple).add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);

    let area = f.size();
    f.render_widget(outer_block.clone(), area);

    let inner_area = outer_block.inner(area);

    let (logo_area, info_area, prompt_area) = if inner_area.height < 25 {
        let splitted = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(inner_area);
        (splitted[0], splitted[1], splitted[2])
    } else {
        let splitted = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Length(6),
                Constraint::Percentage(5),
                Constraint::Length(10),
                Constraint::Min(0),
                Constraint::Length(3),
                Constraint::Percentage(10),
            ])
            .split(inner_area);
        (splitted[1], splitted[3], splitted[5])
    };

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
        Paragraph::new(brand_lines).alignment(Alignment::Center),
        logo_area,
    );

    let lang_display = t!("header_lang_display");
    let status_lbl = t!("header_status_online");
    let sys_info_title = t!("header_sys_info");

    let info_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.cyan))
        .border_type(BorderType::Rounded)
        .title(Span::styled(
            sys_info_title.as_ref(),
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);

    let info_inner_area = info_block.inner(info_area);
    f.render_widget(info_block, info_area);

    let info_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(info_inner_area);

    let left_lines = vec![
        Line::from(vec![
            Span::styled(" 🤖 Tool Name:  ", Style::default().fg(theme.border)),
            Span::styled("git-ai (GIT-CHILL)", Style::default().fg(theme.purple).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" ⚡ AI Agent:   ", Style::default().fg(theme.border)),
            Span::styled(status_lbl.as_ref(), Style::default().fg(theme.green).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 🌎 Language:   ", Style::default().fg(theme.border)),
            Span::styled(lang_display.as_ref(), Style::default().fg(theme.yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 📦 Version:    ", Style::default().fg(theme.border)),
            Span::styled("v3.0.0", Style::default().fg(theme.cyan)),
        ]),
    ];
    f.render_widget(Paragraph::new(left_lines), info_cols[0]);

    let lbl_staged = t!("header_staged");
    let lbl_unstaged = t!("header_unstaged");
    let lbl_untracked = t!("header_untracked");

    let right_lines = vec![
        Line::from(vec![
            Span::styled(" 📂 Workspace:  ", Style::default().fg(theme.border)),
            Span::styled(
                if app.current_dir.len() > 30 {
                    format!("...{}", &app.current_dir[app.current_dir.len()-27..])
                } else {
                    app.current_dir.clone()
                },
                Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(" 🌿 Branch:     ", Style::default().fg(theme.border)),
            Span::styled(&app.current_branch, Style::default().fg(theme.green).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(" 📊 Changes:    ", Style::default().fg(theme.border)),
            Span::styled(format!("{}{}", lbl_staged, app.staged_count), Style::default().fg(theme.green)),
            Span::styled(" | ", Style::default().fg(theme.border)),
            Span::styled(format!("{}{}", lbl_unstaged, app.unstaged_count), Style::default().fg(theme.yellow)),
        ]),
        Line::from(vec![
            Span::styled("                ", Style::default()),
            Span::styled(format!("{}{}", lbl_untracked, app.untracked_count), Style::default().fg(theme.purple)),
        ]),
    ];
    f.render_widget(Paragraph::new(right_lines), info_cols[1]);

    let start_prompt = t!("header_start_prompt");
    let prompt_p = Paragraph::new(Line::from(Span::styled(
        start_prompt.as_ref(),
        Style::default()
            .fg(theme.yellow)
            .add_modifier(Modifier::BOLD),
    )))
    .alignment(Alignment::Center)
    .block(create_block(theme.yellow));

    let prompt_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ])
        .split(prompt_area);

    f.render_widget(prompt_p, prompt_cols[1]);
}
