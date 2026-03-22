use crate::command::Command;
use crate::state::Control;
use crossterm::event::KeyCode;

pub fn handle(key: KeyCode) -> Option<Command> {
    match key {
        KeyCode::Left => Some(Command::PrevDifficulty),
        KeyCode::Right => Some(Command::NextDifficulty),
        KeyCode::Enter => Some(Command::StartGame),
        KeyCode::Char('q') | KeyCode::Esc => Some(Command::Quit),
        _ => None,
    }
}

pub fn controls() -> Vec<Control> {
    vec![
        Control {
            key: "←/→",
            label: "Select",
        },
        Control {
            key: "Enter",
            label: "Start",
        },
        Control {
            key: "q",
            label: "Quit",
        },
    ]
}
