mod config;
mod constants;
mod input;
mod state;
mod terminal;
mod ui;

use state::AppState;

fn main() -> std::io::Result<()> {
    let mut terminal = terminal::init()?;
    let mut state = AppState::Menu {
        difficulty: sudoku_core::Difficulty::Easy,
    };

    loop {
        terminal.draw(|f| ui::draw(&state, f))?;

        let event = crossterm::event::read()?;
        if let crossterm::event::Event::Key(key) = event {
            match &state {
                AppState::Menu { .. } => {
                    if let Some(action) = input::menu::handle(key.code) {
                        match action {
                            input::menu::Action::PrevDifficulty => {
                                if let AppState::Menu { difficulty } = &mut state {
                                    *difficulty = config::cycle(*difficulty, false);
                                }
                            }
                            input::menu::Action::NextDifficulty => {
                                if let AppState::Menu { difficulty } = &mut state {
                                    *difficulty = config::cycle(*difficulty, true);
                                }
                            }
                            input::menu::Action::Start => {
                                if let AppState::Menu { difficulty } = &state {
                                    let (puzzle, solution) = sudoku_core::generate(*difficulty);
                                    let errors = sudoku_core::find_errors(&puzzle);
                                    state = AppState::Playing {
                                        puzzle,
                                        solution,
                                        cursor_row: 4,
                                        cursor_col: 4,
                                        errors,
                                    };
                                }
                            }
                            input::menu::Action::Back => break,
                        }
                    }
                }
                AppState::Playing { .. } => {
                    if let Some(action) = input::playing::handle(key.code) {
                        match action {
                            input::playing::Action::Quit => {
                                state = AppState::Menu {
                                    difficulty: sudoku_core::Difficulty::Easy,
                                };
                            }
                            input::playing::Action::MoveLeft => {
                                if let AppState::Playing { cursor_col, .. } = &mut state
                                    && *cursor_col > 0
                                {
                                    *cursor_col -= 1;
                                }
                            }
                            input::playing::Action::MoveRight => {
                                if let AppState::Playing { cursor_col, .. } = &mut state
                                    && *cursor_col < 8
                                {
                                    *cursor_col += 1;
                                }
                            }
                            input::playing::Action::MoveUp => {
                                if let AppState::Playing { cursor_row, .. } = &mut state
                                    && *cursor_row > 0
                                {
                                    *cursor_row -= 1;
                                }
                            }
                            input::playing::Action::MoveDown => {
                                if let AppState::Playing { cursor_row, .. } = &mut state
                                    && *cursor_row < 8
                                {
                                    *cursor_row += 1;
                                }
                            }
                            input::playing::Action::PlaceNumber(n) => {
                                if let AppState::Playing {
                                    puzzle,
                                    cursor_row,
                                    cursor_col,
                                    errors,
                                    ..
                                } = &mut state
                                {
                                    let cell = &mut puzzle[*cursor_row][*cursor_col];
                                    let already_has_n =
                                        matches!(cell, sudoku_core::Cell::UserInput(v) if *v == n);
                                    if !already_has_n
                                        && !matches!(cell, sudoku_core::Cell::Given(_))
                                    {
                                        *cell = sudoku_core::Cell::UserInput(n);
                                        *errors = sudoku_core::find_errors(puzzle);
                                    }
                                }
                            }
                            input::playing::Action::Erase => {
                                if let AppState::Playing {
                                    puzzle,
                                    cursor_row,
                                    cursor_col,
                                    errors,
                                    ..
                                } = &mut state
                                {
                                    let cell = &mut puzzle[*cursor_row][*cursor_col];
                                    if matches!(cell, sudoku_core::Cell::UserInput(_)) {
                                        *cell = sudoku_core::Cell::Empty;
                                        *errors = sudoku_core::find_errors(puzzle);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    terminal::cleanup()
}
