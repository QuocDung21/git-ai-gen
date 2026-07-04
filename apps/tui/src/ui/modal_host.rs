use crate::app::App;
use crate::models::ActiveModal;
use crate::ui::modals;
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Clear},
    Frame,
};

pub fn render_active_modal(f: &mut Frame, app: &App) {
    if app.active_modal == ActiveModal::None {
        return;
    }

    let theme = app.theme();
    let area = modal_area(&app.active_modal, f.size());

    if area.width > 2 && area.height > 1 {
        let shadow_area = Rect {
            x: area.x + 2,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(1),
        };
        f.render_widget(
            Block::default().style(Style::default().bg(theme.shadow())),
            shadow_area,
        );
    }

    f.render_widget(Clear, area);
    f.render_widget(Block::default().style(Style::default().bg(theme.bg)), area);
    render_modal_content(f, app, area);
}

fn modal_area(active_modal: &ActiveModal, frame_area: Rect) -> Rect {
    match active_modal {
        ActiveModal::Help => modals::centered_rect(65, 75, frame_area),
        ActiveModal::LanguageSelect => modals::centered_rect(40, 25, frame_area),
        ActiveModal::RevertConfirm(_) => modals::centered_rect(50, 30, frame_area),
        ActiveModal::GitLog => modals::centered_rect(75, 70, frame_area),
        ActiveModal::BranchSelect => modals::centered_rect(50, 45, frame_area),
        ActiveModal::DiffResult => modals::centered_rect(75, 85, frame_area),
        ActiveModal::GoConfirm => modals::centered_rect(70, 70, frame_area),
        ActiveModal::StashList => modals::centered_rect(70, 65, frame_area),
        ActiveModal::RemoteInfo => modals::centered_rect(65, 62, frame_area),
        ActiveModal::AmendCommit => modals::centered_rect(68, 50, frame_area),
        ActiveModal::CommitDiff(_) => modals::centered_rect(88, 88, frame_area),
        ActiveModal::MergeConfirm(_) => modals::centered_rect(55, 30, frame_area),
        ActiveModal::NewBranchInput => modals::centered_rect(55, 30, frame_area),
        ActiveModal::WorkspaceHistory => modals::centered_rect(60, 50, frame_area),
        ActiveModal::WorkspacePathInput => modals::centered_rect(55, 30, frame_area),
        ActiveModal::ProjectLanguages => modals::centered_rect(55, 48, frame_area),
        ActiveModal::HandleTest => modals::centered_rect(75, 60, frame_area),
        ActiveModal::ViewPrompt => modals::centered_rect(80, 80, frame_area),
        ActiveModal::ManualCommit => modals::centered_rect(65, 38, frame_area),
        ActiveModal::GitMenu => modals::centered_rect(60, 70, frame_area),
        ActiveModal::CommitTree => modals::centered_rect(85, 80, frame_area),
        ActiveModal::FeatureCommit => modals::centered_rect(55, 50, frame_area),
        ActiveModal::GithubDownloadUrlInput => modals::centered_rect(80, 68, frame_area),
        ActiveModal::GithubDownloadTree => modals::centered_rect(85, 80, frame_area),
        ActiveModal::GithubDownloadTargetInput => modals::centered_rect(80, 45, frame_area),
        ActiveModal::GithubQuickView { .. } => modals::centered_rect(95, 90, frame_area),
        ActiveModal::GithubBranchSelect => modals::centered_rect(50, 45, frame_area),
        ActiveModal::BranchDeleteConfirm(_) => modals::centered_rect(55, 30, frame_area),
        ActiveModal::ClearTrashConfirm => modals::centered_rect(55, 28, frame_area),
        ActiveModal::Settings => modals::centered_rect(55, 58, frame_area),
        ActiveModal::EditorSelect => modals::centered_rect(45, 36, frame_area),
        ActiveModal::None => frame_area,
    }
}

fn render_modal_content(f: &mut Frame, app: &App, area: Rect) {
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
        ActiveModal::ClearTrashConfirm => modals::render_clear_trash_confirm(f, app, area),
        ActiveModal::Settings => modals::render_settings(f, app, area),
        ActiveModal::EditorSelect => modals::render_editor_select(f, app, area),
        ActiveModal::None => {}
    }
}
