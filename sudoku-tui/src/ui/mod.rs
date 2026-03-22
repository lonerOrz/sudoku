// ui/mod.rs: UI 模块入口

mod menu;
mod playing;

use crate::state::AppState;

pub fn draw(state: &AppState, f: &mut ratatui::prelude::Frame) {
    match state {
        AppState::Menu { difficulty } => menu::draw(f, *difficulty),
        AppState::Playing {
            puzzle,
            cursor_row,
            cursor_col,
            errors,
            ..
        } => playing::draw(f, puzzle, *cursor_row, *cursor_col, errors),
        AppState::Won { difficulty } => playing::draw_won(f, *difficulty),
    }
}
