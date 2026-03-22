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
            mistakes,
            start_time,
            paused,
            ..
        } => {
            playing::draw(
                f,
                puzzle,
                *cursor_row,
                *cursor_col,
                errors,
                *mistakes,
                *start_time,
                *paused,
            );
        }
        AppState::Won {
            difficulty,
            elapsed_secs,
        } => playing::draw_won(f, *difficulty, *elapsed_secs),
        AppState::Failed {
            difficulty,
            elapsed_secs,
        } => playing::draw_failed(f, *difficulty, *elapsed_secs),
    }
}
