pub mod components;
pub mod modals;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::App;
use crate::app::models::ActiveModal;

pub fn ui(f: &mut Frame, app: &App) {
    let is_vi = app.current_lang == "vi";

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(8), // Upgraded Header / Banner to fit 6 lines of ASCII logo!
                Constraint::Length(3), // Workspace Badge Bar
                Constraint::Min(0),    // Main workspace area
                Constraint::Length(3), // Status message bar
            ]
            .as_ref(),
        )
        .split(f.size());

    // 1 & 2. RENDER HEADER & BADGE BAR
    components::render_header(f, app, chunks[0], chunks[1]);

    // Split the main content area into 3 columns horizontally
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(28), // Left: Changes list
                Constraint::Percentage(48), // Middle: Live Diff view
                Constraint::Percentage(24), // Right: Commands Legend
            ]
            .as_ref(),
        )
        .split(chunks[2]);

    // 3. LEFT COLUMN: 📂 CHANGES
    components::render_changes(f, app, main_chunks[0]);

    // 4. RENDER LIVE DIFF VIEW
    components::render_diff(f, app, main_chunks[1]);

    // 5. RENDER CONTROL LEGEND
    components::render_legend(f, app, main_chunks[2]);

    // 6. STATUS MESSAGE BAR
    let is_warning = app.status_message.starts_with("⚠️")
        || app.status_message.contains("CONFIRM")
        || app.status_message.contains("XÁC NHẬN");
    let is_error = app.status_message.starts_with("❌")
        || app.status_message.contains("Error")
        || app.status_message.contains("Lỗi");
    let is_success = app.status_message.starts_with("✅")
        || app.status_message.starts_with("🚀")
        || app.status_message.starts_with("⚡")
        || app.status_message.starts_with("✨");

    let status_color = if is_warning {
        Color::Rgb(241, 250, 140) // Yellow Warning
    } else if is_error {
        Color::Rgb(255, 85, 85) // Red Error
    } else if is_success {
        Color::Rgb(80, 250, 123) // Green Success
    } else {
        Color::Rgb(139, 233, 253) // Cyan Info
    };

    let status_text = Line::from(vec![
        Span::styled(
            if is_vi {
                "  🔔  THÔNG BÁO HỆ THỐNG  "
            } else {
                "  🔔  SYSTEM NOTIFICATION  "
            },
            Style::default()
                .fg(Color::Rgb(248, 248, 242))
                .bg(status_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            &app.status_message,
            Style::default()
                .fg(status_color)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let status_widget = Paragraph::new(status_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(status_color))
            .border_type(ratatui::widgets::BorderType::Rounded),
    );
    f.render_widget(status_widget, chunks[3]);

    // 7. RENDER FLOATING MODAL OVERLAYS (LAST IN CANVAS LAYERS)
    match &app.active_modal {
        ActiveModal::Help => {
            let area = modals::centered_rect(65, 75, f.size());
            modals::render_help_modal(f, app, area);
        }
        ActiveModal::LanguageSelect => {
            let area = modals::centered_rect(40, 25, f.size());
            modals::render_language_select(f, app, area);
        }
        ActiveModal::RevertConfirm(path) => {
            let area = modals::centered_rect(50, 30, f.size());
            modals::render_revert_confirm(f, app, path, area);
        }
        ActiveModal::GitLog => {
            let area = modals::centered_rect(75, 70, f.size());
            modals::render_git_log(f, app, area);
        }
        ActiveModal::BranchSelect => {
            let area = modals::centered_rect(50, 45, f.size());
            modals::render_branch_select(f, app, area);
        }
        ActiveModal::DiffResult => {
            let area = modals::centered_rect(72, 72, f.size());
            modals::render_diff_result(f, app, area);
        }
        ActiveModal::GoConfirm => {
            let area = modals::centered_rect(70, 70, f.size());
            modals::render_go_confirm(f, app, area);
        }
        ActiveModal::StashList => {
            let area = modals::centered_rect(70, 65, f.size());
            modals::render_stash_list(f, app, area);
        }
        ActiveModal::RemoteInfo => {
            let area = modals::centered_rect(65, 55, f.size());
            modals::render_remote_info(f, app, area);
        }
        ActiveModal::AmendCommit => {
            let area = modals::centered_rect(68, 50, f.size());
            modals::render_amend_commit(f, app, area);
        }
        ActiveModal::CommitDiff(hash) => {
            let area = modals::centered_rect(88, 88, f.size());
            modals::render_commit_diff(f, app, hash, area);
        }
        ActiveModal::MergeConfirm(selected_branch) => {
            let area = modals::centered_rect(55, 30, f.size());
            modals::render_merge_confirm(f, app, selected_branch, area);
        }
        ActiveModal::None => {}
    }
}
