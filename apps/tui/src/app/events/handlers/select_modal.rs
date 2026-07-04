use crossterm::event::KeyEvent;

use crate::app::App;
use crate::models::ActiveModal;

pub fn handle_select_modal_keys(app: &mut App, key: &KeyEvent) {
    match &app.active_modal.clone() {
        ActiveModal::Help => super::basic_select::handle_help(app, key),
        ActiveModal::LanguageSelect => super::basic_select::handle_language_select(app, key),
        ActiveModal::Settings => super::basic_select::handle_settings(app, key),
        ActiveModal::EditorSelect => super::basic_select::handle_editor_select(app, key),
        ActiveModal::WorkspaceHistory => {
            super::workspace_select::handle_workspace_history(app, key)
        }
        ActiveModal::ProjectLanguages => super::basic_select::handle_project_languages(app, key),
        ActiveModal::HandleTest => super::basic_select::handle_handle_test(app, key),
        ActiveModal::GitLog => super::history_select::handle_git_log(app, key),
        ActiveModal::CommitDiff(_) => super::history_select::handle_commit_diff(app, key),
        ActiveModal::BranchSelect => super::branch_select::handle_branch_select(app, key),
        ActiveModal::MergeConfirm(branch_name) => {
            super::branch_select::handle_merge_confirm(app, key, branch_name);
        }
        ActiveModal::BranchDeleteConfirm(branch_name) => {
            super::branch_select::handle_branch_delete_confirm(app, key, branch_name);
        }
        ActiveModal::RemoteInfo => super::basic_select::handle_remote_info(app, key),
        ActiveModal::ViewPrompt => super::basic_select::handle_view_prompt(app, key),
        ActiveModal::CommitTree => super::history_select::handle_commit_tree(app, key),
        ActiveModal::FeatureCommit => super::history_select::handle_feature_commit(app, key),
        ActiveModal::GithubQuickView { .. } => {
            super::github_select::handle_github_quick_view(app, key);
        }
        ActiveModal::GithubBranchSelect => {
            super::github_select::handle_github_branch_select(app, key)
        }
        _ => {}
    }
}
