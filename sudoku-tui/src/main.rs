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
            match &mut state {
                AppState::Menu { difficulty } => {
                    if let Some(action) = input::menu::handle(key.code) {
                        match action {
                            input::menu::Action::PrevDifficulty => {
                                *difficulty = config::cycle(*difficulty, false);
                            }
                            input::menu::Action::NextDifficulty => {
                                *difficulty = config::cycle(*difficulty, true);
                            }
                            input::menu::Action::Start => {}
                            input::menu::Action::Back => break,
                        }
                    }
                }
            }
        }
    }

    terminal::cleanup()
}
