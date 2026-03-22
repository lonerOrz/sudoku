// main.rs: 应用入口和状态管理

mod constants;
mod input;
mod state;
mod terminal;
mod ui;

use state::{AppState, HistoryEntry};
use sudoku_core::{Cell, Difficulty, find_errors, generate, has_empty};

fn main() -> std::io::Result<()> {
    let mut terminal = terminal::init()?;
    let mut state = AppState::Menu {
        difficulty: Difficulty::Easy,
    };

    loop {
        terminal.draw(|f| ui::draw(&state, f))?;

        if crossterm::event::poll(std::time::Duration::from_millis(500))? {
            let event = crossterm::event::read()?;
            if let crossterm::event::Event::Key(key) = event {
                match &state {
                    AppState::Menu { .. } => {
                        if let Some(action) = input::menu::handle(key.code) {
                            match action {
                                input::menu::Action::PrevDifficulty => {
                                    if let AppState::Menu { difficulty } = &mut state {
                                        *difficulty = constants::cycle(*difficulty, false);
                                    }
                                }
                                input::menu::Action::NextDifficulty => {
                                    if let AppState::Menu { difficulty } = &mut state {
                                        *difficulty = constants::cycle(*difficulty, true);
                                    }
                                }
                                input::menu::Action::Start => {
                                    if let AppState::Menu { difficulty } = &state {
                                        let (puzzle, _solution) = generate(*difficulty);
                                        let errors = error_vec_to_array(find_errors(&puzzle));
                                        state = AppState::Playing {
                                            puzzle,
                                            cursor_row: 4,
                                            cursor_col: 4,
                                            errors,
                                            difficulty: *difficulty,
                                            mistakes: 0,
                                            start_time: std::time::Instant::now(),
                                            elapsed_secs: 0,
                                            paused: false,
                                            history: vec![],
                                        };
                                    }
                                }
                                input::menu::Action::Back => break,
                            }
                        }
                    }
                    AppState::Won { .. } | AppState::Failed { .. } => {
                        if let Some(input::playing::Action::Quit) = input::playing::handle(key.code)
                        {
                            state = AppState::Menu {
                                difficulty: Difficulty::Easy,
                            };
                        }
                    }
                    AppState::Playing { paused, .. } => {
                        if let Some(action) = input::playing::handle(key.code) {
                            match action {
                                input::playing::Action::Quit => {
                                    state = AppState::Menu {
                                        difficulty: Difficulty::Easy,
                                    };
                                }
                                input::playing::Action::Pause => {
                                    if let AppState::Playing {
                                        paused,
                                        start_time,
                                        elapsed_secs,
                                        ..
                                    } = &mut state
                                    {
                                        if *paused {
                                            *start_time = std::time::Instant::now()
                                                - std::time::Duration::from_secs(*elapsed_secs);
                                        } else {
                                            *elapsed_secs = start_time.elapsed().as_secs();
                                        }
                                        *paused = !*paused;
                                    }
                                }
                                input::playing::Action::Undo => {
                                    handle_undo(&mut state);
                                }
                                _ if !*paused => {
                                    handle_playing_action(&mut state, action);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    terminal::cleanup()
}

fn error_vec_to_array(errors: Vec<(usize, usize)>) -> [bool; 81] {
    let mut arr = [false; 81];
    for (r, c) in errors {
        arr[r * 9 + c] = true;
    }
    arr
}

fn handle_undo(state: &mut AppState) {
    if let AppState::Playing {
        puzzle,
        cursor_row,
        cursor_col,
        errors,
        mistakes,
        history,
        ..
    } = state
        && let Some(entry) = history.pop()
    {
        *puzzle = entry.puzzle;
        *cursor_row = entry.cursor_row;
        *cursor_col = entry.cursor_col;
        *mistakes = entry.mistakes;
        *errors = error_vec_to_array(find_errors(puzzle));
    }
}

fn save_history(state: &AppState) -> Option<HistoryEntry> {
    if let AppState::Playing {
        puzzle,
        cursor_row,
        cursor_col,
        mistakes,
        ..
    } = state
    {
        Some(HistoryEntry {
            puzzle: *puzzle,
            cursor_row: *cursor_row,
            cursor_col: *cursor_col,
            mistakes: *mistakes,
        })
    } else {
        None
    }
}

fn handle_playing_action(state: &mut AppState, action: input::playing::Action) {
    match action {
        input::playing::Action::MoveLeft => {
            if let AppState::Playing { cursor_col, .. } = state
                && *cursor_col > 0
            {
                *cursor_col -= 1;
            }
        }
        input::playing::Action::MoveRight => {
            if let AppState::Playing { cursor_col, .. } = state
                && *cursor_col < 8
            {
                *cursor_col += 1;
            }
        }
        input::playing::Action::MoveUp => {
            if let AppState::Playing { cursor_row, .. } = state
                && *cursor_row > 0
            {
                *cursor_row -= 1;
            }
        }
        input::playing::Action::MoveDown => {
            if let AppState::Playing { cursor_row, .. } = state
                && *cursor_row < 8
            {
                *cursor_row += 1;
            }
        }
        input::playing::Action::PlaceNumber(n) => {
            let history_entry = save_history(state);
            if let AppState::Playing {
                puzzle,
                cursor_row,
                cursor_col,
                errors,
                difficulty,
                mistakes,
                start_time,
                history,
                ..
            } = state
            {
                let cell = &mut puzzle[*cursor_row][*cursor_col];
                let already_has_n = matches!(cell, Cell::UserInput(v) if *v == n);
                if !already_has_n && !matches!(cell, Cell::Given(_)) {
                    if let Some(entry) = history_entry {
                        history.push(entry);
                    }

                    *cell = Cell::UserInput(n);
                    *errors = error_vec_to_array(find_errors(puzzle));
                    let cursor_idx = *cursor_row * 9 + *cursor_col;
                    if errors[cursor_idx] {
                        *mistakes += 1;
                        if *mistakes >= 5 {
                            *state = AppState::Failed {
                                difficulty: *difficulty,
                                elapsed_secs: start_time.elapsed().as_secs(),
                            };
                        }
                    } else if errors.iter().all(|&e| !e) && !has_empty(puzzle) {
                        *state = AppState::Won {
                            difficulty: *difficulty,
                            elapsed_secs: start_time.elapsed().as_secs(),
                        };
                    }
                }
            }
        }
        input::playing::Action::Erase => {
            let history_entry = save_history(state);
            if let AppState::Playing {
                puzzle,
                cursor_row,
                cursor_col,
                errors,
                history,
                ..
            } = state
            {
                let cell = &mut puzzle[*cursor_row][*cursor_col];
                if matches!(cell, Cell::UserInput(_)) {
                    if let Some(entry) = history_entry {
                        history.push(entry);
                    }
                    *cell = Cell::Empty;
                    *errors = error_vec_to_array(find_errors(puzzle));
                }
            }
        }
        _ => {}
    }
}
