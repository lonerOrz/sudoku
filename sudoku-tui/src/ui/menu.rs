// ui/menu.rs: 菜单 UI

use ratatui::{
    prelude::{Alignment, Constraint, Frame, Layout, Line, Modifier, Rect, Span, Style},
    style::Color,
    widgets::Paragraph,
};
use sudoku_core::Difficulty;

use crate::config;
use crate::constants::{self, MENU_HEIGHT, MENU_WIDTH};

pub fn draw(f: &mut Frame, difficulty: Difficulty) {
    let area = center(MENU_WIDTH, MENU_HEIGHT, f.size());

    let chunks = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(8),
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Min(0),
    ])
    .split(area);

    f.render_widget(constants::title_widget(), chunks[1]);
    f.render_widget(selector(difficulty), chunks[2]);
    f.render_widget(controls(), chunks[3]);
}

fn center(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}

fn selector(difficulty: Difficulty) -> Paragraph<'static> {
    let label = difficulty.label();
    let color = config::color(difficulty);
    let (min, max) = difficulty.givens_range();

    let content = vec![
        Line::from(Span::raw("Select Difficulty")),
        Line::from(vec![
            Span::styled("◄  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("  {}  ", label),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("({}-{} clues)", min, max),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled("  ►", Style::default().fg(Color::DarkGray)),
        ]),
    ];
    Paragraph::new(content).alignment(Alignment::Center)
}

fn controls() -> Paragraph<'static> {
    use crate::input::menu;

    let mut spans = Vec::new();
    for (i, ctrl) in menu::controls().iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("  "));
        }
        spans.push(Span::styled(ctrl.key, Style::default().fg(Color::Cyan)));
        spans.push(Span::styled(
            format!(" {}", ctrl.label),
            Style::default().fg(Color::DarkGray),
        ));
    }
    Paragraph::new(vec![Line::from(spans)]).alignment(Alignment::Center)
}
