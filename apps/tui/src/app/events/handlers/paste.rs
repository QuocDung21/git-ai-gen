use crate::app::App;
use crate::models::ActiveModal;

pub fn handle_paste(app: &mut App, text: &str) {
    match &app.active_modal {
        ActiveModal::ManualCommit => {
            app.manual_commit_message.push_str(text);
        }
        ActiveModal::GithubDownloadUrlInput => {
            app.github_download_url.push_str(text);
        }
        ActiveModal::NewBranchInput => {
            app.new_branch_name.push_str(text);
        }
        ActiveModal::GithubDownloadTargetInput => {
            app.github_download_target_path.push_str(text);
        }
        ActiveModal::GithubQuickView { .. } if app.github_quickview_searching => {
            app.github_quickview_search.push_str(text);
        }
        _ => {}
    }
}
