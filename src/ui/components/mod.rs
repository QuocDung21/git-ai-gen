pub mod header;
pub mod diff;
pub mod legend;
pub mod changes;

pub use header::{render_badge_bar, render_splash_screen};
pub use diff::render_diff;
pub use legend::render_legend;
pub use changes::render_changes;
