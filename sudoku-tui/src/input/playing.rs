// input/playing.rs: 游戏输入处理

use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Back,
}

pub fn handle(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => Some(Action::Back),
        _ => None,
    }
}
