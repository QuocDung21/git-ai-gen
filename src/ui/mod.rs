pub mod components;
pub mod modals;

use crate::app::models::ActiveModal;
use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    let theme = app.theme();
    let root_bg = Block::default().style(Style::default().bg(theme.bg));
    f.render_widget(root_bg, f.size());

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
    let theme = app.theme();
    let is_warning = app.status_message.starts_with("⚠️")
        || app.status_message.contains("CONFIRM")
        || app.status_message.contains("XÁC NHẬN");
    let is_error = app.status_message.starts_with("❌")
        || app.status_message.contains("Error")
        || app.status_message.contains("Lỗi");
    let is_loading = app.status_message.starts_with("⏳");
    let is_success = app.status_message.starts_with("✅")
        || app.status_message.starts_with("🚀")
        || app.status_message.starts_with("⚡")
        || app.status_message.starts_with("✨");

    let status_color = if is_warning {
        theme.yellow
    } else if is_error {
        theme.red
    } else if is_success {
        theme.green
    } else if is_loading {
        theme.cyan
    } else {
        theme.cyan
    };

    let status_text = Line::from(vec![
        Span::styled(
            if is_vi {
                "  🔔  THÔNG BÁO HỆ THỐNG  "
            } else {
                "  🔔  SYSTEM NOTIFICATION  "
            },
            Style::default()
                .fg(theme.bg)
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
    if app.active_modal != ActiveModal::None {
        let area = match &app.active_modal {
            ActiveModal::Help => modals::centered_rect(65, 75, f.size()),
            ActiveModal::LanguageSelect => modals::centered_rect(40, 25, f.size()),
            ActiveModal::RevertConfirm(_) => modals::centered_rect(50, 30, f.size()),
            ActiveModal::GitLog => modals::centered_rect(75, 70, f.size()),
            ActiveModal::BranchSelect => modals::centered_rect(50, 45, f.size()),
            ActiveModal::DiffResult => modals::centered_rect(75, 85, f.size()),
            ActiveModal::GoConfirm => modals::centered_rect(70, 70, f.size()),
            ActiveModal::StashList => modals::centered_rect(70, 65, f.size()),
            ActiveModal::RemoteInfo => modals::centered_rect(65, 62, f.size()),
            ActiveModal::AmendCommit => modals::centered_rect(68, 50, f.size()),
            ActiveModal::CommitDiff(_) => modals::centered_rect(88, 88, f.size()),
            ActiveModal::MergeConfirm(_) => modals::centered_rect(55, 30, f.size()),
            ActiveModal::NewBranchInput => modals::centered_rect(55, 30, f.size()),
            ActiveModal::ThemeSelect => modals::centered_rect(45, 35, f.size()),
            ActiveModal::WorkspaceHistory => modals::centered_rect(60, 50, f.size()),
            ActiveModal::ViewPrompt => modals::centered_rect(80, 80, f.size()),
            ActiveModal::KiloModelSelect => modals::centered_rect(70, 70, f.size()),
            ActiveModal::ManualCommit => modals::centered_rect(65, 38, f.size()),
            ActiveModal::GitMenu => modals::centered_rect(60, 70, f.size()),
            ActiveModal::CommitTree => modals::centered_rect(85, 80, f.size()),
            ActiveModal::FeatureCommit => modals::centered_rect(55, 50, f.size()),
            ActiveModal::GithubDownloadUrlInput => modals::centered_rect(80, 68, f.size()),
            ActiveModal::GithubDownloadTree => modals::centered_rect(85, 80, f.size()),
            ActiveModal::GithubDownloadTargetInput => modals::centered_rect(80, 45, f.size()),
            ActiveModal::GithubQuickView { .. } => modals::centered_rect(95, 90, f.size()),
            ActiveModal::GithubBranchSelect => modals::centered_rect(50, 45, f.size()),
            ActiveModal::BranchDeleteConfirm(_) => modals::centered_rect(55, 30, f.size()),
            ActiveModal::Settings => modals::centered_rect(55, 38, f.size()),
            ActiveModal::None => f.size(),
        };

        f.render_widget(Clear, area);
        f.render_widget(Block::default().style(Style::default().bg(theme.bg)), area);

        match &app.active_modal {
            ActiveModal::Help => modals::render_help_modal(f, app, area),
            ActiveModal::LanguageSelect => modals::render_language_select(f, app, area),
            ActiveModal::RevertConfirm(path) => modals::render_revert_confirm(f, app, path, area),
            ActiveModal::GitLog => modals::render_git_log(f, app, area),
            ActiveModal::BranchSelect => modals::render_branch_select(f, app, area),
            ActiveModal::DiffResult => modals::render_diff_result(f, app, area),
            ActiveModal::GoConfirm => modals::render_go_confirm(f, app, area),
            ActiveModal::StashList => modals::render_stash_list(f, app, area),
            ActiveModal::RemoteInfo => modals::render_remote_info(f, app, area),
            ActiveModal::AmendCommit => modals::render_amend_commit(f, app, area),
            ActiveModal::CommitDiff(hash) => modals::render_commit_diff(f, app, hash, area),
            ActiveModal::MergeConfirm(selected_branch) => {
                modals::render_merge_confirm(f, app, selected_branch, area)
            }
            ActiveModal::NewBranchInput => modals::render_new_branch_input(f, app, area),
            ActiveModal::ThemeSelect => modals::render_theme_select(f, app, area),
            ActiveModal::WorkspaceHistory => modals::render_workspace_history(f, app, area),
            ActiveModal::ViewPrompt => modals::render_view_prompt(f, app, area),
            ActiveModal::KiloModelSelect => modals::render_kilo_model_select(f, app, area),
            ActiveModal::ManualCommit => modals::render_manual_commit(f, app, area),
            ActiveModal::GitMenu => modals::render_git_menu(f, app, area),
            ActiveModal::CommitTree => modals::render_commit_tree(f, app, area),
            ActiveModal::FeatureCommit => modals::render_feature_commit(f, app, area),
            ActiveModal::GithubDownloadUrlInput => modals::render_github_download_url_input(f, app, area),
            ActiveModal::GithubDownloadTree => modals::render_github_download_tree(f, app, area),
            ActiveModal::GithubDownloadTargetInput => modals::render_github_download_target_input(f, app, area),
            ActiveModal::GithubQuickView { path, name } => modals::render_github_quick_view(f, app, area, path, name),
            ActiveModal::GithubBranchSelect => modals::render_github_branch_select(f, app, area),
            ActiveModal::BranchDeleteConfirm(branch_name) => modals::render_branch_delete_confirm(f, app, &branch_name, area),
            ActiveModal::Settings => modals::render_settings(f, app, area),
            ActiveModal::None => {}
        }
    }
}
