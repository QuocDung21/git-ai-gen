use std::process::Command;

use super::App;

impl App {
    pub fn try_generate_with_kilo(&mut self, full_diff: &str) -> Result<String, String> {
        let ai_lang = crate::helper::Helper::get_ai_language_name();
        let prompt = format!(
            "{} {}.\n\nDiff:\n\n{}",
            crate::constant::Constant::PROMPT_EXPERT,
            ai_lang,
            full_diff
        );

        let mut cmd = Command::new("kilo");
        cmd.args(["run", "--pure", "--auto"]);

        let model_to_use = if !self.current_kilo_model.is_empty() {
            self.current_kilo_model.clone()
        } else if let Ok(env_model) = std::env::var("KILO_MODEL") {
            env_model
        } else {
            String::new()
        };

        if !model_to_use.trim().is_empty() {
            cmd.args(["--model", &model_to_use]);
        }

        cmd.arg(prompt);

        let output = match cmd.output() {
            Ok(o) => o,
            Err(e) => {
                return Err(if self.current_lang == "vi" {
                    format!("Không tìm thấy lệnh 'kilo'. Hãy cài @kilocode/cli (npm i -g @kilocode/cli) hoặc đảm bảo 'kilo' có trong PATH. Lỗi: {}", e)
                } else {
                    format!("'kilo' command not found. Please install @kilocode/cli (npm i -g @kilocode/cli) and ensure it is in PATH. Error: {}", e)
                });
            }
        };

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(if self.current_lang == "vi" {
                format!("kilo run thất bại: {}", stderr)
            } else {
                format!("kilo run failed: {}", stderr)
            });
        }

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if stdout.is_empty() {
            return Err(if self.current_lang == "vi" {
                "kilo trả về kết quả rỗng.".to_string()
            } else {
                "kilo returned empty output.".to_string()
            });
        }

        Ok(stdout)
    }

    pub fn fetch_kilo_models(&mut self) {
        self.kilo_models.clear();
        if let Ok(output) = Command::new("kilo").arg("models").output() {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if line.starts_with("kilo/") || line.contains('/') {
                    self.kilo_models.push(line.to_string());
                }
            }
        }
        if self.selected_kilo_model_index >= self.kilo_models.len() {
            self.selected_kilo_model_index = 0;
        }
    }
}
