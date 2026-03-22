// input/menu.rs: 菜单输入处理

use crossterm::event::KeyCode;

use super::controls::Control;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    PrevDifficulty,
    NextDifficulty,
    Start,
    Back,
}

pub fn handle(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Left => Some(Action::PrevDifficulty),
        KeyCode::Right => Some(Action::NextDifficulty),
        KeyCode::Enter | KeyCode::Char(' ') => Some(Action::Start),
        KeyCode::Char('q') | KeyCode::Esc => Some(Action::Back),
        _ => None,
    }
}

pub fn controls() -> &'static [Control] {
    &[
        Control {
            key: "←/→",
            label: "Change",
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
