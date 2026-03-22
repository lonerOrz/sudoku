// ui/playing.rs: 游戏界面

use ratatui::{
    prelude::{Alignment, Constraint, Frame, Layout, Line, Span, Style},
    style::Color,
    widgets::Paragraph,
};
use sudoku_core::{Cell, Difficulty, Grid};

const CELL_W: usize = 7;
const CELL_H: usize = 3;

pub fn draw(
    f: &mut Frame,
    puzzle: &Grid,
    cursor_row: usize,
    cursor_col: usize,
    errors: &[(usize, usize)],
    mistakes: u8,
) {
    let area = f.size();

    let main_chunks = Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).split(area);

    let grid = render_grid(puzzle, cursor_row, cursor_col, errors);
    let grid_width = grid
        .iter()
        .map(|l| l.to_string().len() as u16)
        .max()
        .unwrap_or(0);
    let grid_height = grid.len() as u16;

    let info = render_info(mistakes);
    let info_width = info
        .iter()
        .map(|l| l.to_string().len() as u16)
        .max()
        .unwrap_or(0);
    let info_height = info.len() as u16;

    let total_width = grid_width + info_width;

    let centered = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Length(total_width),
        Constraint::Min(0),
    ])
    .split(main_chunks[0]);

    let side_chunks =
        Layout::horizontal([Constraint::Ratio(8, 10), Constraint::Ratio(2, 10)]).split(centered[1]);

    let grid_v = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(grid_height),
        Constraint::Min(0),
    ])
    .split(side_chunks[0]);

    let info_v = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(info_height),
        Constraint::Min(0),
    ])
    .split(side_chunks[1]);

    f.render_widget(Paragraph::new(grid).alignment(Alignment::Center), grid_v[1]);
    f.render_widget(Paragraph::new(info).alignment(Alignment::Left), info_v[1]);

    let hints = render_controls();
    let hints_width = hints.to_string().len() as u16;
    let hints_h = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Length(hints_width),
        Constraint::Min(0),
    ])
    .split(main_chunks[1]);

    let hints_v = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Min(0),
    ])
    .split(hints_h[1]);

    f.render_widget(
        Paragraph::new(vec![hints.clone()])
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: false }),
        hints_v[1],
    );
}

pub fn draw_won(f: &mut Frame, difficulty: Difficulty) {
    let area = f.size();

    let label = difficulty.label();

    let content = vec![
        Line::from(vec![Span::styled(
            "Congratulations!",
            Style::default()
                .fg(Color::Green)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::raw(format!("You completed {}!", label))]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::styled(
            "Press q to return to menu",
            Style::default().fg(Color::Cyan),
        )]),
    ];

    let v_chunks = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(content.len() as u16),
        Constraint::Min(0),
    ])
    .split(area);

    let paragraph = Paragraph::new(content).alignment(Alignment::Center);

    f.render_widget(paragraph, v_chunks[1]);
}

pub fn draw_failed(f: &mut Frame, difficulty: Difficulty) {
    let area = f.size();

    let label = difficulty.label();

    let content = vec![
        Line::from(vec![Span::styled(
            "Game Over",
            Style::default()
                .fg(Color::Red)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::raw(format!("Too many mistakes on {}!", label))]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::styled(
            "Press q to return to menu",
            Style::default().fg(Color::Cyan),
        )]),
    ];

    let v_chunks = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(content.len() as u16),
        Constraint::Min(0),
    ])
    .split(area);

    let paragraph = Paragraph::new(content).alignment(Alignment::Center);

    f.render_widget(paragraph, v_chunks[1]);
}

fn render_info(mistakes: u8) -> Vec<Line<'static>> {
    vec![
        Line::from(vec![Span::styled(
            "Info",
            Style::default()
                .fg(Color::White)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::raw("Timer: 00:00")]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::raw(format!("Mistakes: {}/5", mistakes))]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::raw("Mode: Normal")]),
    ]
}

fn render_controls() -> Line<'static> {
    use crate::input::playing;

    let mut spans = Vec::new();
    for (i, ctrl) in playing::controls().iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("  "));
        }
        spans.push(Span::styled(ctrl.key, Style::default().fg(Color::Cyan)));
        spans.push(Span::styled(
            format!(" {}", ctrl.label),
            Style::default().fg(Color::White),
        ));
    }
    Line::from(spans)
}

fn render_grid(
    puzzle: &Grid,
    cursor_row: usize,
    cursor_col: usize,
    errors: &[(usize, usize)],
) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    lines.push(h_line(LineKind::Top));

    for cell_row in 0..9 {
        for inner_row in 0..CELL_H {
            lines.push(content_line(
                puzzle, cell_row, inner_row, cursor_row, cursor_col, errors,
            ));
        }

        if cell_row == 8 {
            lines.push(h_line(LineKind::Bottom));
        } else if (cell_row + 1) % 3 == 0 {
            lines.push(h_line(LineKind::Thick));
        } else {
            lines.push(h_line(LineKind::Thin));
        }
    }

    lines
}

enum LineKind {
    Top,
    Bottom,
    Thin,
    Thick,
}

fn h_line(kind: LineKind) -> Line<'static> {
    let mut s = String::new();

    let (left, right, fill) = match kind {
        LineKind::Top => ('┌', '┐', '─'),
        LineKind::Bottom => ('└', '┘', '─'),
        LineKind::Thin => ('├', '┤', '─'),
        LineKind::Thick => ('├', '┤', '═'),
    };

    s.push(left);

    for col in 0..9 {
        for _ in 0..CELL_W {
            s.push(fill);
        }

        if col < 8 {
            let ch = match kind {
                LineKind::Top => '┬',
                LineKind::Bottom => '┴',
                LineKind::Thin => '┼',
                LineKind::Thick => {
                    if (col + 1) % 3 == 0 {
                        '╬'
                    } else {
                        '╪'
                    }
                }
            };
            s.push(ch);
        }
    }

    s.push(right);
    Line::from(s)
}

fn content_line(
    puzzle: &Grid,
    cell_row: usize,
    inner_row: usize,
    cursor_row: usize,
    cursor_col: usize,
    errors: &[(usize, usize)],
) -> Line<'static> {
    let mut spans = Vec::new();

    let center_row = CELL_H / 2;

    for (cell_col, _) in puzzle[cell_row].iter().enumerate().take(9) {
        let is_cursor = cell_row == cursor_row && cell_col == cursor_col;
        let is_error = errors.contains(&(cell_row, cell_col));
        let cell = puzzle[cell_row][cell_col];
        let is_user_input_error = is_error && matches!(cell, Cell::UserInput(_));

        let bg = if is_cursor {
            Color::Blue
        } else if is_error && matches!(cell, Cell::Given(_)) {
            Color::Red
        } else {
            Color::Reset
        };

        let sep_char = if cell_col == 0 || cell_col % 3 == 0 {
            "┃"
        } else {
            "│"
        };
        let sep_fg = if cell_col == 0 || cell_col % 3 == 0 {
            Color::White
        } else {
            Color::DarkGray
        };
        spans.push(Span::styled(sep_char, Style::default().fg(sep_fg)));

        let content = if inner_row == center_row {
            match cell {
                Cell::Given(v) => format!("   {}   ", char::from_digit(v as u32, 10).unwrap()),
                Cell::UserInput(v) => format!("   {}   ", char::from_digit(v as u32, 10).unwrap()),
                Cell::Empty => {
                    if is_cursor {
                        "   ·   ".to_string()
                    } else {
                        "       ".to_string()
                    }
                }
            }
        } else {
            "       ".to_string()
        };

        let fg = if is_user_input_error {
            Color::Red
        } else {
            match cell {
                Cell::Given(_) => Color::White,
                Cell::UserInput(_) => Color::Cyan,
                Cell::Empty => Color::White,
            }
        };
        spans.push(Span::styled(content, Style::default().fg(fg).bg(bg)));
    }
    spans.push(Span::styled("┃", Style::default().fg(Color::White)));

    Line::from(spans)
}
