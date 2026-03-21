mod menu;

pub fn draw(state: &crate::state::AppState, f: &mut ratatui::prelude::Frame) {
    match state {
        crate::state::AppState::Menu { difficulty } => menu::draw(f, *difficulty),
    }
}
