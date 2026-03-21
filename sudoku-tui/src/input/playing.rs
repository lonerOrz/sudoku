// input/playing.rs: 游戏输入处理

use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Action {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    PlaceNumber(u8),
    Erase,
    Undo,
    Quit,
}

pub fn handle(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Left => Some(Action::MoveLeft),
        KeyCode::Right => Some(Action::MoveRight),
        KeyCode::Up => Some(Action::MoveUp),
        KeyCode::Down => Some(Action::MoveDown),
        KeyCode::Char(c) if c.is_ascii_digit() => {
            c.to_digit(10).map(|d| Action::PlaceNumber(d as u8))
        }
        KeyCode::Char('0') | KeyCode::Delete | KeyCode::Backspace => Some(Action::Erase),
        KeyCode::Char('u') | KeyCode::Char('U') => Some(Action::Undo),
        KeyCode::Char('q') | KeyCode::Esc => Some(Action::Quit),
        _ => None,
    }
}

pub struct Control {
    pub key: &'static str,
    pub label: &'static str,
}

pub fn controls() -> &'static [Control] {
    &[
        Control {
            key: "←↑↓→",
            label: "Move",
        },
        Control {
            key: "1-9",
            label: "Place",
        },
        Control {
            key: "0/Del",
            label: "Erase",
        },
        Control {
            key: "u",
            label: "Undo",
        },
        Control {
            key: "q",
            label: "Quit",
        },
    ]
}
