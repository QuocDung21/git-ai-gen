use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};
use rust_i18n::t;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToastKind {
    Info,
    Success,
    Warning,
    Error,
}

impl ToastKind {
    pub fn from_message(message: &str) -> Self {
        if message.starts_with("❌")
            || message.contains("Error")
            || message.contains("error")
            || message.contains("Lỗi")
            || message.contains("failed")
            || message.contains("Failed")
        {
            Self::Error
        } else if message.starts_with("⚠️")
            || message.contains("CONFIRM")
            || message.contains("XÁC NHẬN")
            || message.contains("warning")
            || message.contains("Warning")
        {
            Self::Warning
        } else if message.starts_with("✅")
            || message.starts_with("🚀")
            || message.starts_with("⚡")
            || message.starts_with("✨")
            || message.contains("success")
            || message.contains("Success")
        {
            Self::Success
        } else {
            Self::Info
        }
    }
}

pub fn render_toast(f: &mut Frame, app: &App, area: Rect) {
    render_toast_message(f, app, area, &app.status_message);
}

pub fn render_toast_message(f: &mut Frame, app: &App, area: Rect, message: &str) {
    let theme = app.theme();
    let kind = ToastKind::from_message(message);
    let color = match kind {
        ToastKind::Info => theme.cyan,
        ToastKind::Success => theme.green,
        ToastKind::Warning => theme.yellow,
        ToastKind::Error => theme.red,
    };
    let label_bg = match kind {
        ToastKind::Info => theme.cyan,
        ToastKind::Success => theme.green,
        ToastKind::Warning => theme.yellow,
        ToastKind::Error => theme.red,
    };
    let label_fg = theme.bg;

    let toast_text = Line::from(vec![
        Span::styled(
            t!("ui_notification_label").to_string(),
            Style::default()
                .fg(label_fg)
                .bg(label_bg)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            message.to_string(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ),
    ]);

    let toast = Paragraph::new(toast_text).wrap(Wrap { trim: true }).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color))
            .border_type(BorderType::Rounded),
    );

    f.render_widget(toast, area);
}
