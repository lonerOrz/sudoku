// input/global.rs: 全局输入处理

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

pub enum Action {
    Quit,
}

pub fn handle_global(event: &Event) -> Option<Action> {
    if let Event::Key(KeyEvent {
        code: KeyCode::Char('c' | 'C'),
        modifiers,
        ..
    }) = event
    {
        if modifiers.contains(KeyModifiers::CONTROL) {
            return Some(Action::Quit);
        }
    }
    None
}

pub fn should_quit(event: &Event) -> bool {
    handle_global(event).is_some()
}
