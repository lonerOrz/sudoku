mod menu;
mod playing;

use crate::state::AppState;
use playing::DrawParams;

pub fn draw(state: &AppState, f: &mut ratatui::prelude::Frame) {
    match state {
        AppState::Menu { difficulty } => menu::draw(f, *difficulty),
        AppState::Playing(game) => {
            playing::draw(
                f,
                &DrawParams {
                    puzzle: game.puzzle(),
                    solution: game.solution(),
                    pencil_marks: game.pencil_marks(),
                    pencil_mode: game.is_pencil_mode(),
                    hint_mode: game.is_hint_mode(),
                    cursor_row: game.cursor_row(),
                    cursor_col: game.cursor_col(),
                    conflicts: game.conflicts(),
                    mistakes: game.mistakes(),
                    hints_used: game.hints_used(),
                    undo_used: game.undo_used(),
                    difficulty: game.difficulty(),
                    elapsed_secs: game.elapsed_secs(),
                    paused: game.is_paused(),
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
