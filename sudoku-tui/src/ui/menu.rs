use ratatui::{
    prelude::{Alignment, Constraint, Frame, Layout, Line, Modifier, Rect, Span, Style},
    style::Color,
    widgets::Paragraph,
};
use sudoku_core::Difficulty;

use crate::config;

const WIDTH: u16 = 55;
const HEIGHT: u16 = 18;

pub fn draw(f: &mut Frame, difficulty: Difficulty) {
    let area = center(WIDTH, HEIGHT, f.size());

    let chunks = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(8),
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Min(0),
    ])
    .split(area);

    f.render_widget(title(), chunks[1]);
    f.render_widget(selector(difficulty), chunks[2]);
    f.render_widget(controls(), chunks[3]);
}

fn center(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}

fn title() -> Paragraph<'static> {
    let lines = vec![
        Line::from(Span::styled(
            r"  ███████╗██╗   ██╗██████╗  ██████╗ ██╗  ██╗██╗   ██╗",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            r"  ██╔════╝██║   ██║██╔══██╗██╔═══██╗██║ ██╔╝██║   ██║",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            r"  ███████╗██║   ██║██║  ██║██║   ██║█████╔╝ ██║   ██║",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            r"  ╚════██║██║   ██║██║  ██║██║   ██║██╔═██╗ ██║   ██║",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            r"  ███████║╚██████╔╝██████╔╝╚██████╔╝██║  ██╗╚██████╔╝",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            r"  ╚══════╝ ╚═════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═╝ ╚═════╝",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
    ];
    Paragraph::new(lines).alignment(Alignment::Center)
}

fn selector(difficulty: Difficulty) -> Paragraph<'static> {
    let label = config::label(difficulty);
    let color = config::color(difficulty);

    let content = vec![
        Line::from(Span::raw("Select Difficulty")),
        Line::from(vec![
            Span::styled("◄  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("  {}  ", label),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::styled("  ►", Style::default().fg(Color::DarkGray)),
        ]),
    ];
    Paragraph::new(content).alignment(Alignment::Center)
}

fn controls() -> Paragraph<'static> {
    let content = vec![Line::from(vec![
        Span::styled("←/→", Style::default().fg(Color::Cyan)),
        Span::styled(" Change  ", Style::default().fg(Color::DarkGray)),
        Span::styled("Enter", Style::default().fg(Color::Cyan)),
        Span::styled(" Start  ", Style::default().fg(Color::DarkGray)),
        Span::styled("q", Style::default().fg(Color::Cyan)),
        Span::styled(" Quit", Style::default().fg(Color::DarkGray)),
    ])];
    Paragraph::new(content).alignment(Alignment::Center)
}
