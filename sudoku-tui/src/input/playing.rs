use crate::command::Command;
use crossterm::event::KeyCode;

pub fn handle(key: KeyCode) -> Option<Command> {
    match key {
        KeyCode::Left => Some(Command::MoveLeft),
        KeyCode::Right => Some(Command::MoveRight),
        KeyCode::Up => Some(Command::MoveUp),
        KeyCode::Down => Some(Command::MoveDown),
        KeyCode::Char('0') | KeyCode::Delete | KeyCode::Backspace => Some(Command::Erase),
        KeyCode::Char(c) if c.is_ascii_digit() => {
            c.to_digit(10).map(|d| Command::PlaceNumber(d as u8))
        }
        KeyCode::Char('u') | KeyCode::Char('U') => Some(Command::Undo),
        KeyCode::Char(' ') => Some(Command::Pause),
        KeyCode::Char('p') | KeyCode::Char('P') => Some(Command::TogglePencilMode),
        KeyCode::Char('h') | KeyCode::Char('H') => Some(Command::ToggleHintMode),
        KeyCode::Char('?') => Some(Command::PlaceHint),
        KeyCode::Char('q') | KeyCode::Esc => Some(Command::Quit),
        _ => None,
    }
}
