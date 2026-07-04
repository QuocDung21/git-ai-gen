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
    pub cmd_help_clear_trash: String,
    pub cmd_help_un: String,
    pub cmd_help_base: String,
    pub setting_auto_push: String,
    pub setting_auto_stage_all: String,
    pub setting_kilo_ai: String,
    pub github_close_all_folders: String,
    pub lang_stats_title: String,
    pub lang_stats_heading: String,
    pub lang_stats_empty: String,
    pub lang_stats_close: String,
    pub dev_modal_title: String,
    pub dev_modal_heading: String,
    pub dev_folder_label: String,
    pub dev_lang_label: String,
    pub dev_theme_label: String,
    pub dev_light_label: String,
    pub dev_changes_label: String,
    pub dev_scanned_label: String,
    pub dev_ide_label: String,
    pub dev_conflict_label: String,
    pub dev_actions_heading: String,
    pub dev_action_theme: String,
    pub dev_action_alert: String,
    pub dev_action_reset_stats: String,
    pub dev_action_diff: String,
    pub dev_action_ffi: String,
    pub dev_action_conflict: String,
    pub dev_action_lang: String,
    pub dev_modal_close: String,
    pub dev_diag_title: String,
    pub dev_playground_title: String,
    pub help_modal_header: String,
    pub help_group_navigation: String,
    pub help_nav_select: String,
    pub help_nav_scroll: String,
    pub help_group_git: String,
    pub help_git_stage: String,
    pub help_git_revert: String,
    pub help_git_stage_all: String,
    pub help_git_unstage_all: String,
    pub help_git_branch: String,
    pub help_git_timeline: String,
    pub help_git_fetch: String,
    pub help_git_pull: String,
    pub help_git_remote: String,
    pub help_git_copy_diff: String,
    pub help_git_ai_prompt: String,
    pub help_git_go: String,
    pub help_group_sys: String,
    pub help_sys_vscode: String,
    pub help_sys_workspace: String,
    pub help_sys_lang_stats: String,
    pub help_sys_lang: String,
    pub help_sys_theme: String,
    pub help_sys_reset: String,
    pub help_sys_quit: String,
    pub help_modal_close: String,
    pub settings_title: String,
    pub settings_header: String,
    pub settings_auto_push: String,
    pub settings_auto_stage: String,
    pub settings_kilo_ai: String,
    pub settings_splash: String,
    pub settings_editor_default: String,
    pub settings_editor_label: String,
    pub settings_path_header: String,
    pub settings_path_config: String,
    pub settings_path_history: String,
    pub settings_help_footer: String,
}

impl Locales {
    pub fn new(lang: &str) -> Self {
        let locale_str = if lang == "Vietnamese" || lang == "vi" {
            "vi"
        } else {
            "en"
        };
        rust_i18n::set_locale(locale_str);

        let yaml_content = if lang == "Vietnamese" || lang == "vi" {
            include_str!("../locales/vi.yml")
        } else {
            include_str!("../locales/en.yml")
        };

        serde_yaml::from_str(yaml_content)
            .unwrap_or_else(|e| panic!("Trục trặc khi parse file ngôn ngữ '{}': {}", lang, e))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    fn locale_keys(content: &str) -> BTreeSet<String> {
        let value: serde_yaml::Value = serde_yaml::from_str(content).unwrap();
        value
            .as_mapping()
            .unwrap()
            .keys()
            .map(|key| key.as_str().unwrap().to_string())
            .collect()
    }

    #[test]
    fn locale_files_have_matching_keys() {
        let en_keys = locale_keys(include_str!("../locales/en.yml"));
        let vi_keys = locale_keys(include_str!("../locales/vi.yml"));

        assert_eq!(en_keys, vi_keys);
    }

    fn source_t_macro_keys() -> BTreeSet<String> {
        let src_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut keys = BTreeSet::new();

        for entry in walkdir::WalkDir::new(src_dir)
            .into_iter()
            .filter_map(Result::ok)
        {
            if entry
                .path()
                .extension()
                .is_none_or(|extension| extension != "rs")
            {
                continue;
            }

            let content = std::fs::read_to_string(entry.path()).unwrap();
            for (index, _) in content.match_indices("t!(\"") {
                if index > 0 {
                    let previous = content[..index].chars().next_back().unwrap();
                    if previous.is_alphanumeric() || previous == '_' {
                        continue;
                    }
                }

                let part = &content[index + 4..];
                if let Some((key, _)) = part.split_once('"') {
                    keys.insert(key.to_string());
                }
            }
        }

        keys
    }

    #[test]
    fn literal_t_macro_keys_exist_in_locale_files() {
        let locale_keys = locale_keys(include_str!("../locales/en.yml"));
        let source_keys = source_t_macro_keys();
        let missing_keys = source_keys
            .difference(&locale_keys)
            .cloned()
            .collect::<Vec<_>>();

        assert!(missing_keys.is_empty(), "{missing_keys:?}");
    }
}
