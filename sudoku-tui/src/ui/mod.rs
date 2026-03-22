mod menu;
mod playing;

use crate::state::AppState;
use playing::DrawParams;

pub fn draw(state: &AppState, f: &mut ratatui::prelude::Frame) {
    match state {
        AppState::Menu { difficulty } => menu::draw(f, *difficulty),
        AppState::Playing {
            puzzle,
            pencil_marks,
            pencil_mode,
            hint_mode,
            cursor_row,
            cursor_col,
            errors,
            mistakes,
            difficulty,
            start_time,
            elapsed_secs,
            paused,
            ..
        } => {
            let display_elapsed = if *paused {
                *elapsed_secs
            } else {
                start_time.elapsed().as_secs()
            };
            playing::draw(
                f,
                &DrawParams {
                    puzzle,
                    pencil_marks,
                    pencil_mode: *pencil_mode,
                    hint_mode: *hint_mode,
                    cursor_row: *cursor_row,
                    cursor_col: *cursor_col,
                    errors,
                    mistakes: *mistakes,
                    difficulty: *difficulty,
                    elapsed_secs: display_elapsed,
                    paused: *paused,
                    controls: state.controls(),
                },
            );
        }
        AppState::Won {
            difficulty,
            elapsed_secs,
        } => playing::draw_won(
            f,
            *difficulty,
            *elapsed_secs,
            &[crate::state::Control {
                key: "q",
                label: "Menu",
            }],
        ),
        AppState::Failed {
            difficulty,
            elapsed_secs,
        } => playing::draw_failed(
            f,
            *difficulty,
            *elapsed_secs,
            &[crate::state::Control {
                key: "q",
                label: "Menu",
            }],
        ),
    }
}
