use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render_settings(f: &mut Frame, app: &App, area: Rect) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let mut content = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            rust_i18n::t!("settings_header").to_string(),
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    let options = [
        (app.auto_push, rust_i18n::t!("settings_auto_push")),
        (app.auto_stage_all, rust_i18n::t!("settings_auto_stage")),
        (app.kilo_ai_enabled, rust_i18n::t!("settings_kilo_ai")),
        (app.splash_enabled, rust_i18n::t!("settings_splash")),
    ];

    for (i, (enabled, text)) in options.iter().enumerate() {
        let is_selected = i == app.selected_setting_index;
        let checkbox = if *enabled { "[X]" } else { "[ ]" };

        let prefix = if is_selected { " ➜ " } else { "   " };
        let style = if is_selected {
            Style::default()
                .fg(theme.select_fg)
                .bg(theme.select_bg)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg)
        };

        let box_color = if *enabled { theme.green } else { theme.red };

        content.push(Line::from(vec![
            Span::styled(
                prefix,
                Style::default()
                    .fg(theme.purple)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{} ", checkbox),
                Style::default().fg(box_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(text.to_string(), style),
        ]));
        content.push(Line::from(""));
    }

    let is_selected_4 = app.selected_setting_index == 4;
    let prefix_4 = if is_selected_4 { " ➜ " } else { "   " };
    let style_4 = if is_selected_4 {
        Style::default()
            .fg(theme.select_fg)
            .bg(theme.select_bg)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.fg)
    };
    let editor_friendly_name = match app.editor.as_str() {
        "code" => "VS Code".to_string(),
        "cursor" => "Cursor".to_string(),
        "zed" => "Zed".to_string(),
        "subl" => "Sublime Text".to_string(),
        _ => rust_i18n::t!("settings_editor_default").to_string(),
    };
    let editor_text =
        rust_i18n::t!("settings_editor_label", editor = &editor_friendly_name).to_string();

    content.push(Line::from(vec![
        Span::styled(
            prefix_4,
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "⚙️  ",
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(editor_text, style_4),
    ]));
    content.push(Line::from(""));

    content.push(Line::from(vec![Span::styled(
        rust_i18n::t!("settings_path_header").to_string(),
        Style::default()
            .fg(theme.purple)
            .add_modifier(Modifier::BOLD),
    )]));

    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| "~".to_string());

    content.push(Line::from(vec![
        Span::styled(
            rust_i18n::t!("settings_path_config").to_string(),
            Style::default().fg(theme.border),
        ),
        Span::styled(
            format!("{}/.gitconfig [git-ai]", home),
            Style::default().fg(theme.green),
        ),
    ]));
    content.push(Line::from(vec![
        Span::styled(
            rust_i18n::t!("settings_path_history").to_string(),
            Style::default().fg(theme.border),
        ),
        Span::styled(
            format!("{}/.git-chill/", home),
            Style::default().fg(theme.green),
        ),
    ]));
    content.push(Line::from(""));

    content.push(Line::from(vec![Span::styled(
        rust_i18n::t!("settings_help_footer").to_string(),
        Style::default().fg(theme.border),
    )]));

    let block = Block::default()
        .title(Span::styled(
            rust_i18n::t!("settings_title").to_string(),
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.purple))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}
