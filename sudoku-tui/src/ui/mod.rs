// ui/mod.rs: UI 模块入口

mod menu;
mod playing;

pub fn draw(state: &crate::state::AppState, f: &mut ratatui::prelude::Frame) {
    match state {
        crate::state::AppState::Menu { difficulty } => menu::draw(f, *difficulty),
        crate::state::AppState::Playing {
            puzzle,
            cursor_row,
            cursor_col,
            errors,
            ..
        } => playing::draw(f, puzzle, *cursor_row, *cursor_col, errors),
        crate::state::AppState::Won { difficulty } => playing::draw_won(f, *difficulty),
    }
}
