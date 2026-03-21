// ui/mod.rs: UI 模块入口

mod menu;

pub fn draw(state: &crate::state::AppState, f: &mut ratatui::prelude::Frame) {
    match state {
        crate::state::AppState::Menu { difficulty } => menu::draw(f, *difficulty),
        crate::state::AppState::Playing { .. } => {
            use ratatui::prelude::Alignment;
            use ratatui::widgets::Paragraph;
            f.render_widget(
                Paragraph::new("Playing...").alignment(Alignment::Center),
                f.size(),
            );
        }
    }
}
