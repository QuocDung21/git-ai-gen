pub mod components;
pub mod modals;

use crate::app::App;
use crate::models::ActiveModal;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Clear},
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    let theme = app.theme();
    let root_bg = Block::default().style(Style::default().bg(theme.bg));
    f.render_widget(root_bg, f.size());

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
            ActiveModal::WorkspacePathInput => modals::centered_rect(55, 30, f.size()),
            ActiveModal::ProjectLanguages => modals::centered_rect(55, 48, f.size()),
            ActiveModal::HandleTest => modals::centered_rect(75, 60, f.size()),
            ActiveModal::ViewPrompt => modals::centered_rect(80, 80, f.size()),
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
            ActiveModal::Settings => modals::centered_rect(55, 58, f.size()),
            ActiveModal::EditorSelect => modals::centered_rect(45, 36, f.size()),
            ActiveModal::None => f.size(),
        };

        if area.width > 2 && area.height > 1 {
            let shadow_area = ratatui::layout::Rect {
                x: area.x + 2,
                y: area.y + 1,
                width: area.width.saturating_sub(2),
                height: area.height.saturating_sub(1),
            };
            let shadow_bg = if theme.bg == Color::Rgb(248, 249, 250) {
                Color::Rgb(180, 185, 195)
            } else if theme.bg == Color::Rgb(46, 52, 64) {
                Color::Rgb(20, 22, 28)
            } else if theme.bg == Color::Rgb(40, 40, 40) {
                Color::Rgb(20, 20, 20)
            } else {
                Color::Rgb(10, 10, 15)
            };
            f.render_widget(
                Block::default().style(Style::default().bg(shadow_bg)),
                shadow_area,
            );
        }

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
            ActiveModal::WorkspacePathInput => modals::render_workspace_path_input(f, app, area),
            ActiveModal::ProjectLanguages => modals::render_project_languages(f, app, area),
            ActiveModal::HandleTest => modals::render_handle_test(f, app, area),
            ActiveModal::ViewPrompt => modals::render_view_prompt(f, app, area),
            ActiveModal::ManualCommit => modals::render_manual_commit(f, app, area),
            ActiveModal::GitMenu => modals::render_git_menu(f, app, area),
            ActiveModal::CommitTree => modals::render_commit_tree(f, app, area),
            ActiveModal::FeatureCommit => modals::render_feature_commit(f, app, area),
            ActiveModal::GithubDownloadUrlInput => {
                modals::render_github_download_url_input(f, app, area)
            }
            ActiveModal::GithubDownloadTree => modals::render_github_download_tree(f, app, area),
            ActiveModal::GithubDownloadTargetInput => {
                modals::render_github_download_target_input(f, app, area)
            }
            ActiveModal::GithubQuickView { path, name } => {
                modals::render_github_quick_view(f, app, area, path, name)
            }
            ActiveModal::GithubBranchSelect => modals::render_github_branch_select(f, app, area),
            ActiveModal::BranchDeleteConfirm(branch_name) => {
                modals::render_branch_delete_confirm(f, app, branch_name, area)
            }
            ActiveModal::Settings => modals::render_settings(f, app, area),
            ActiveModal::EditorSelect => modals::render_editor_select(f, app, area),
            ActiveModal::None => {}
        }
    }
}
