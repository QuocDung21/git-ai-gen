use anyhow::{bail, Context, Result};
use rust_i18n::t;
use std::process::Command;

use crate::cli::{ask_confirm_default_no, logger};

pub fn handle_enable_touch_id_sudo() -> Result<()> {
    #[cfg(not(target_os = "macos"))]
    {
        logger::warn(t!("touch_id_sudo_unsupported").as_ref());
        return Ok(());
    }

    #[cfg(target_os = "macos")]
    {
        logger::heading(t!("touch_id_sudo_heading").as_ref());

        if sudo_touch_id_configured() {
            logger::success(t!("touch_id_sudo_already_enabled").as_ref());
            return Ok(());
        }

        logger::warn(t!("touch_id_sudo_warning").as_ref());
        if !ask_confirm_default_no(t!("touch_id_sudo_confirm").as_ref())? {
            logger::success(t!("touch_id_sudo_cancelled").as_ref());
            return Ok(());
        }

        enable_touch_id_for_sudo()?;
        logger::success(t!("touch_id_sudo_enabled").as_ref());
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn sudo_touch_id_configured() -> bool {
    ["/etc/pam.d/sudo_local", "/etc/pam.d/sudo"]
        .iter()
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .flat_map(|content| {
            content
                .lines()
                .map(str::trim)
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .any(|line| !line.starts_with('#') && line.contains("pam_tid.so"))
}

#[cfg(target_os = "macos")]
fn enable_touch_id_for_sudo() -> Result<()> {
    let script = r#"set -eu
target="/etc/pam.d/sudo_local"
template="/etc/pam.d/sudo_local.template"

if [ ! -f "$target" ]; then
  if [ -f "$template" ]; then
    cp "$template" "$target"
  else
    touch "$target"
  fi
fi

if grep -Eq '^[[:space:]]*auth[[:space:]]+sufficient[[:space:]]+pam_tid\.so' "$target"; then
  exit 0
fi

if grep -Eq '^[[:space:]]*#[[:space:]]*auth[[:space:]]+sufficient[[:space:]]+pam_tid\.so' "$target"; then
  sed -i '' -E 's/^[[:space:]]*#[[:space:]]*(auth[[:space:]]+sufficient[[:space:]]+pam_tid\.so.*)$/\1/' "$target"
else
  printf '%s\n' 'auth       sufficient     pam_tid.so' | cat - "$target" > "$target.tmp"
  mv "$target.tmp" "$target"
fi
"#;

    let status = Command::new("sudo")
        .arg("sh")
        .arg("-c")
        .arg(script)
        .status()
        .context(t!("touch_id_sudo_enable_failed").to_string())?;

    if !status.success() {
        bail!("{}", t!("touch_id_sudo_enable_failed"));
    }

    Ok(())
}
