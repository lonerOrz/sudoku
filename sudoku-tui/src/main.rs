mod command;
mod constants;
mod game;
mod input;
mod state;
mod terminal;
mod ui;

use crate::command::Command;
use crate::input::global::should_quit;
use crate::input::menu;
use crate::input::playing;
use crate::state::AppState;
use game::{apply_command, init_menu};
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
    let quit = match state {
        AppState::Menu { .. } => {
            if let Some(cmd) = menu::handle(key) {
                if matches!(cmd, Command::Quit) {
                    true
                } else {
                    apply_command(state, cmd);
                    false
                }
            } else {
                false
            }
        }
        AppState::Playing(game) => {
            if game.is_paused() {
                if key == crossterm::event::KeyCode::Char(' ') {
                    apply_command(state, Command::Pause);
                } else if key == crossterm::event::KeyCode::Char('q')
                    || key == crossterm::event::KeyCode::Esc
                {
                    apply_command(state, Command::Quit);
                }
                false
            } else if let Some(cmd) = playing::handle(key) {
                apply_command(state, cmd);
                false
            } else {
                false
            }
        }
        AppState::Won { .. } | AppState::Failed { .. } => {
            if let Some(cmd) = playing::handle(key) {
                apply_command(state, cmd);
            }
            false
        }
    };
    !quit
}
