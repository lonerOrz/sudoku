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
                                    state = AppState::Playing { puzzle, solution };
                                }
                            }
                            input::menu::Action::Back => break,
                        }
                    }
                }
                AppState::Playing { .. } => {
                    if let Some(input::playing::Action::Back) = input::playing::handle(key.code) {
                        state = AppState::Menu {
                            difficulty: sudoku_core::Difficulty::Easy,
                        };
                    }
                }
            }
        }
    }

    terminal::cleanup()
}
