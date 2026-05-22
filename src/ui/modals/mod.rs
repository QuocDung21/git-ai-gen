pub mod help;
pub mod confirm;
pub mod manual_commit;

pub use help::render_help_modal;
pub use confirm::{
    render_language_select,
    render_revert_confirm,
    render_git_log,
    render_branch_select,
    render_diff_result,
    render_go_confirm,
    render_stash_list,
    render_remote_info,
    render_amend_commit,
    render_commit_diff,
    render_merge_confirm,
    render_new_branch_input,
    render_theme_select,
    render_workspace_history,
    render_view_prompt,
    render_kilo_model_select,
    render_git_menu,
    render_commit_tree,
};
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
