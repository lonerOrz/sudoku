// input/playing.rs: 游戏输入处理

use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    PlaceNumber(u8),
    Erase,
    Undo,
    Pause,
    TogglePencilMode,
    ToggleHintMode,
    PlaceHint,
    Quit,
}

pub fn handle(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Left => Some(Action::MoveLeft),
        KeyCode::Right => Some(Action::MoveRight),
        KeyCode::Up => Some(Action::MoveUp),
        KeyCode::Down => Some(Action::MoveDown),
        KeyCode::Char('0') | KeyCode::Delete | KeyCode::Backspace => Some(Action::Erase),
        KeyCode::Char(c) if c.is_ascii_digit() => {
            c.to_digit(10).map(|d| Action::PlaceNumber(d as u8))
        }
        KeyCode::Char('u') | KeyCode::Char('U') => Some(Action::Undo),
        KeyCode::Char(' ') => Some(Action::Pause),
        KeyCode::Char('p') | KeyCode::Char('P') => Some(Action::TogglePencilMode),
        KeyCode::Char('h') | KeyCode::Char('H') => Some(Action::ToggleHintMode),
        KeyCode::Char('?') => Some(Action::PlaceHint),
        KeyCode::Char('q') | KeyCode::Esc => Some(Action::Quit),
        _ => None,
    }
}
