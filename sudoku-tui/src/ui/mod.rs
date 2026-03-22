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
            ..
        } => playing::draw(
            f,
            puzzle,
            *cursor_row,
            *cursor_col,
            errors,
            *mistakes,
            *start_time,
        ),
        AppState::Paused {
            puzzle,
            cursor_row,
            cursor_col,
            errors,
            mistakes,
            elapsed_secs,
            ..
        } => {
            playing::draw_paused(
                f,
                puzzle,
                *cursor_row,
                *cursor_col,
                errors,
                *mistakes,
                *elapsed_secs,
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
