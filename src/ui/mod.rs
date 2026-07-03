pub mod components;
pub mod modal_host;
pub mod modals;

use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::Block,
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    let theme = app.theme();
    f.render_widget(
        Block::default().style(Style::default().bg(theme.bg).fg(theme.fg)),
        f.size(),
    );

    if app.show_splash {
        components::render_splash_screen(f, app);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    components::render_badge_bar(f, app, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(28),
                Constraint::Percentage(48),
                Constraint::Percentage(24),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    // 3. LEFT COLUMN: 📂 CHANGES
    components::render_changes(f, app, main_chunks[0]);

    // 4. RENDER LIVE DIFF VIEW
    components::render_diff(f, app, main_chunks[1]);

    // 5. RENDER CONTROL LEGEND
    components::render_legend(f, app, main_chunks[2]);

    components::render_toast(f, app, chunks[2]);
    modal_host::render_active_modal(f, app);
}
