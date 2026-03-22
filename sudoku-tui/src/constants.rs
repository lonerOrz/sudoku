// constants.rs: UI 常量

use ratatui::{
    prelude::{Alignment, Line, Modifier, Span, Style},
    style::Color,
    widgets::Paragraph,
};
use sudoku_core::Difficulty;

pub const MENU_WIDTH: u16 = 55;
pub const MENU_HEIGHT: u16 = 18;

pub const TITLE: &str = r"  ███████╗██╗   ██╗██████╗  ██████╗ ██╗  ██╗██╗   ██╗
  ██╔════╝██║   ██║██╔══██╗██╔═══██╗██║ ██╔╝██║   ██║
  ███████╗██║   ██║██║  ██║██║   ██║█████╔╝ ██║   ██║
  ╚════██║██║   ██║██║  ██║██║   ██║██╔═██╗ ██║   ██║
  ███████║╚██████╔╝██████╔╝╚██████╔╝██║  ██╗╚██████╔╝
  ╚══════╝ ╚═════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═╝ ╚═════╝";

pub fn title_widget() -> Paragraph<'static> {
    let lines: Vec<Line> = TITLE
        .lines()
        .map(|line| {
            Line::from(Span::styled(
                line,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ))
        })
        .collect();
    Paragraph::new(lines).alignment(Alignment::Center)
}

pub fn difficulty_color(d: Difficulty) -> Color {
    match d {
        Difficulty::Easy => Color::Green,
        Difficulty::Medium => Color::Yellow,
        Difficulty::Hard => Color::Red,
        Difficulty::Expert => Color::Magenta,
    }
}
