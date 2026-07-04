pub use git_ai_core_shared::{cleanup, constant, git, helper, locales, models, theme};

rust_i18n::i18n!("../../core/git-ai-core/locales");

#[cfg(feature = "tui")]
#[allow(dead_code)]
mod app;

#[cfg(feature = "tui")]
#[allow(dead_code)]
mod cli;

#[cfg(feature = "tui")]
#[allow(dead_code)]
mod ui;
