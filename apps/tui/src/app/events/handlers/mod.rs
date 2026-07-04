use crossterm::event::KeyEvent;

use crate::app::App;
use crate::models::ActiveModal;

mod basic_select;
mod branch_select;
mod clear_trash;
pub(crate) mod close_modal;
mod confirm;
pub(crate) mod diff_result;
pub(crate) mod git_menu;
pub(crate) mod github;
mod github_select;
pub(crate) mod go_confirm;
pub(crate) mod handle_browser;
mod history_select;
pub(crate) mod input_modal;
pub(crate) mod navigation;
pub(crate) mod paste;
pub(crate) mod select_modal;
pub(crate) mod stash;
mod workspace_select;

// Re-export all handler functions for crate-level access
// These are consumed indirectly via pub(crate) modules in dashboard.rs
#[allow(unused_imports)]
pub use clear_trash::handle_clear_trash_confirm;
#[allow(unused_imports)]
pub use confirm::handle_revert_confirm;
#[allow(unused_imports)]
pub use diff_result::handle_diff_result;
#[allow(unused_imports)]
pub use git_menu::handle_git_menu;
#[allow(unused_imports)]
pub use github::{handle_download_tree, handle_github_cloning};
#[allow(unused_imports)]
pub use go_confirm::{
    handle_amend_edit, handle_amend_pushing, handle_go_confirm, handle_go_pushing,
};
#[allow(unused_imports)]
pub use input_modal::handle_input_modal_keys;
#[allow(unused_imports)]
pub use navigation::handle_standard_keys;
#[allow(unused_imports)]
pub use paste::handle_paste;
#[allow(unused_imports)]
pub use select_modal::handle_select_modal_keys;
#[allow(unused_imports)]
pub use stash::handle_stash;

/// Dispatch key event to the active modal handler.
/// Returns true if the key was handled by a modal handler.
pub fn dispatch_modal_key(app: &mut App, key: &KeyEvent) -> bool {
    match &app.active_modal {
        ActiveModal::Help
        | ActiveModal::LanguageSelect
        | ActiveModal::Settings
        | ActiveModal::EditorSelect
        | ActiveModal::BranchSelect
        | ActiveModal::GitLog
        | ActiveModal::CommitTree
        | ActiveModal::FeatureCommit
        | ActiveModal::WorkspaceHistory
        | ActiveModal::RemoteInfo
        | ActiveModal::ViewPrompt
        | ActiveModal::CommitDiff(_)
        | ActiveModal::MergeConfirm(_)
        | ActiveModal::BranchDeleteConfirm(_)
        | ActiveModal::GithubBranchSelect
        | ActiveModal::ProjectLanguages
        | ActiveModal::HandleTest
        | ActiveModal::GithubQuickView { .. } => {
            select_modal::handle_select_modal_keys(app, key);
            true
        }
        ActiveModal::RevertConfirm(_) => {
            confirm::handle_revert_confirm(app, key);
            true
        }
        ActiveModal::ClearTrashConfirm => {
            clear_trash::handle_clear_trash_confirm(app, key);
            true
        }
        ActiveModal::DiffResult => {
            diff_result::handle_diff_result(app, key);
            true
        }
        ActiveModal::ManualCommit
        | ActiveModal::NewBranchInput
        | ActiveModal::GithubDownloadUrlInput
        | ActiveModal::GithubDownloadTargetInput
        | ActiveModal::WorkspacePathInput => {
            input_modal::handle_input_modal_keys(app, key);
            true
        }
        ActiveModal::GitMenu => {
            git_menu::handle_git_menu(app, key);
            true
        }
        ActiveModal::GoConfirm => {
            go_confirm::handle_go_confirm(app, key);
            true
        }
        ActiveModal::StashList => {
            stash::handle_stash(app, key);
            true
        }
        ActiveModal::AmendCommit => {
            go_confirm::handle_amend_edit(app, key);
            true
        }
        ActiveModal::GithubDownloadTree => {
            github::handle_download_tree(app, key);
            true
        }
        ActiveModal::None => false,
    }
}
