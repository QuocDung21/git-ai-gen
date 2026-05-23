use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Locales {
    pub help_title: String,
    pub help_desc: String,
    pub diff_success: String,
    pub error_prefix: String,
    pub press_enter: String,
    pub no_changes: String,
    pub status_clean: String,
    pub status_pending: String,
    pub status_fail: String,
    pub preview_heading: String,
    pub commit_content: String,
    pub confirm_deploy: String,
    pub pushing: String,
    pub push_success: String,
    pub deploy_cancel: String,
    pub reset_heading: String,
    pub reset_success: String,
    pub reset_info: String,
    pub reset_clean: String,
    pub confirm_remove_alias: String,
    pub keep_alias: String,
    pub lang_set: String,
    pub lang_auto: String,
    pub lang_invalid: String,
    pub cmd_help_diff: String,
    pub cmd_help_go: String,
    pub cmd_help_un: String,
    pub cmd_help_base: String,
    pub setting_auto_push: String,
    pub setting_auto_stage_all: String,
    pub setting_kilo_ai: String,
    pub github_close_all_folders: String,
}

impl Locales {
    pub fn new(lang: &str) -> Self {
        let yaml_content = if lang == "Vietnamese" || lang == "vi" {
            include_str!("../locales/vi.yml")
        } else {
            include_str!("../locales/en.yml")
        };

        serde_yaml::from_str(yaml_content)
            .unwrap_or_else(|e| panic!("Trục trặc khi parse file ngôn ngữ '{}': {}", lang, e))
    }
}
