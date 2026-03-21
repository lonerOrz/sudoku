// ui/mod.rs: UI 模块入口

mod menu;
mod playing;

pub fn draw(state: &crate::state::AppState, f: &mut ratatui::prelude::Frame) {
    match state {
        crate::state::AppState::Menu { difficulty } => menu::draw(f, *difficulty),
        crate::state::AppState::Playing { puzzle, .. } => playing::draw(f, puzzle),
    }
}
