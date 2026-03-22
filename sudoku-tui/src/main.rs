mod command;
mod constants;
mod game;
mod input;
mod state;
mod terminal;
mod ui;

use crate::command::Command;
use crate::input::global::should_quit;
use crate::state::AppState;
use game::{handle_input, init_menu};
use sudoku_core::Difficulty;

fn main() -> std::io::Result<()> {
    let (mut terminal, _guard) = terminal::init()?;
    let mut state = init_menu(Difficulty::Easy);

    loop {
        terminal.draw(|f| ui::draw(&state, f))?;

        if crossterm::event::poll(std::time::Duration::from_millis(500))? {
            let event = crossterm::event::read()?;
            if should_quit(&event) {
                break;
            }
            if let crossterm::event::Event::Key(key) = event
                && !handle_key(&mut state, key.code)
            {
                break;
            }
        }
    }

    Ok(())
}

fn handle_key(state: &mut AppState, key: crossterm::event::KeyCode) -> bool {
    if let AppState::Playing(game) = state
        && game.is_paused()
    {
        if key == crossterm::event::KeyCode::Char(' ')
            || key == crossterm::event::KeyCode::Char('q')
            || key == crossterm::event::KeyCode::Esc
        {
            handle_input(state, key);
            return true;
        }
        return true;
    }

    if let AppState::Menu { .. } = state
        && let Some(cmd) = crate::input::menu::handle(key)
        && matches!(cmd, Command::Quit)
    {
        return false;
    }

    handle_input(state, key);
    true
}
