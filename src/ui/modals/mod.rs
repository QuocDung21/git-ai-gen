pub mod branch_delete;
pub mod clear_trash;
pub mod confirm;
pub mod editor_select;
pub mod feature_commit;
pub mod github_download;
pub mod handle_test;
pub mod help;
pub mod manual_commit;
pub mod project_languages;
pub mod settings;

pub use branch_delete::render_branch_delete_confirm;
pub use clear_trash::render_clear_trash_confirm;
pub use editor_select::render_editor_select;
pub use handle_test::render_handle_test;
pub use project_languages::render_project_languages;
pub use settings::render_settings;

pub use confirm::{
    render_amend_commit, render_branch_select, render_commit_diff, render_commit_tree,
    render_diff_result, render_git_log, render_git_menu, render_go_confirm, render_language_select,
    render_merge_confirm, render_new_branch_input, render_remote_info, render_revert_confirm,
    render_stash_list, render_view_prompt, render_workspace_history, render_workspace_path_input,
};
pub use feature_commit::render_feature_commit;
pub use github_download::{
    render_github_branch_select, render_github_download_target_input, render_github_download_tree,
    render_github_download_url_input, render_github_quick_view,
};
pub use help::render_help_modal;
pub use manual_commit::render_manual_commit;

use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
