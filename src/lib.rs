// ============================================================================
// git-ai Library Root (for FFI consumers + pure core)
// ============================================================================
//
// Public API exposed to FFI / other Rust crates (always available):
//   - constant, git, helper, locales, models, theme
//
// Heavy interactive TUI code (app, cli, ui) is only compiled when the
// "tui" feature is enabled (default for the binary, optional for library users).
//
// This structure lets us build a slim staticlib/rlib for Swift/Kotlin/etc.
// without pulling in ratatui, console, crossterm, etc.
//
// Default: cargo build          → includes TUI (for the git-ai binary)
//          cargo build --no-default-features → slim FFI-only library

pub mod constant;
pub mod git;
pub mod helper;
pub mod locales;
pub mod models;
pub mod theme;

rust_i18n::i18n!("locales");


// FFI C ABI layer (always present so the staticlib exports the symbols)
mod ffi;

// ---------------------------------------------------------------------------
// Heavy TUI / interactive layers – gated behind the "tui" feature
// ---------------------------------------------------------------------------
// TUI layers are compiled with dead_code allowed because many items
// (renderers, event handlers, App methods) are only called at runtime
// via the binary or through feature-gated paths.
#[cfg(feature = "tui")]
#[allow(dead_code)]
mod app;

#[cfg(feature = "tui")]
#[allow(dead_code)]
mod cli;

#[cfg(feature = "tui")]
#[allow(dead_code)]
mod ui;
