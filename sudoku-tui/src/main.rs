// main.rs: 应用入口和事件循环

mod constants;
mod game;
mod input;
mod state;
mod terminal;
mod ui;

use crate::input::menu;
use crate::input::playing;
use crate::state::AppState;
use game::{init_menu, start_game, update};
use sudoku_core::Difficulty;

fn main() -> std::io::Result<()> {
    let (mut terminal, _guard) = terminal::init()?;
    let mut state = init_menu(Difficulty::Easy);

    loop {
        terminal.draw(|f| ui::draw(&state, f))?;

        if crossterm::event::poll(std::time::Duration::from_millis(500))? {
            let event = crossterm::event::read()?;
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
    match state {
        AppState::Menu { difficulty } => {
            if let Some(action) = menu::handle(key) {
                match action {
                    menu::Action::PrevDifficulty => {
                        *difficulty = difficulty.prev();
                    }
                    menu::Action::NextDifficulty => {
                        *difficulty = difficulty.next();
                    }
                    menu::Action::Start => {
                        *state = start_game(*difficulty);
                    }
                    menu::Action::Back => {
                        return false;
                    }
                }
            }
        }
        AppState::Playing(game) => {
            if let Some(action) = playing::handle(key)
                && is_action_allowed(&action, game.is_paused())
            {
                update(state, action);
            }
        }
        AppState::Won { .. } | AppState::Failed { .. } => {
            if let Some(playing::Action::Quit) = playing::handle(key) {
                *state = init_menu(Difficulty::Easy);
            }
        }
    }
    true
}

fn is_action_allowed(action: &playing::Action, paused: bool) -> bool {
    match action {
        playing::Action::Quit
        | playing::Action::Pause
        | playing::Action::TogglePencilMode
        | playing::Action::ToggleHintMode
        | playing::Action::PlaceHint
        | playing::Action::Undo => true,
        playing::Action::MoveLeft
        | playing::Action::MoveRight
        | playing::Action::MoveUp
        | playing::Action::MoveDown
        | playing::Action::PlaceNumber(_)
        | playing::Action::Erase => !paused,
    }
}
