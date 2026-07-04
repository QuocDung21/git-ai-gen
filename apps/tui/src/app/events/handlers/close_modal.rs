use crate::app::App;

pub fn close_modal(app: &mut App) {
    app.active_modal = crate::models::ActiveModal::None;
}
