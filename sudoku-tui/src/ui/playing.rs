// ui/playing.rs: 游戏界面

use ratatui::{
    prelude::{Alignment, Constraint, Frame, Layout, Line, Span, Style},
    style::Color,
    widgets::Paragraph,
};
use sudoku_core::Cell;

const CELL_W: usize = 6;
const CELL_H: usize = 3;

pub fn draw(f: &mut Frame, puzzle: &sudoku_core::Grid) {
    let area = f.size();

    let main_chunks = Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).split(area);

    let grid = render_grid(puzzle);
    let grid_width = grid
        .iter()
        .map(|l| l.to_string().len() as u16)
        .max()
        .unwrap_or(0);
    let grid_height = grid.len() as u16;

    let info = render_info();
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

fn render_info() -> Vec<Line<'static>> {
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
        Line::from(vec![Span::raw("Mistakes: 0/5")]),
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

fn render_grid(puzzle: &sudoku_core::Grid) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    lines.push(h_line(LineKind::Top));

    for cell_row in 0..9 {
        for inner_row in 0..CELL_H {
            lines.push(content_line(puzzle, cell_row, inner_row));
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

fn content_line(puzzle: &sudoku_core::Grid, cell_row: usize, inner_row: usize) -> Line<'static> {
    let mut s = String::new();

    for (cell_col, _) in puzzle[cell_row].iter().enumerate() {
        let sep = if cell_col == 0 || cell_col % 3 == 0 {
            '┃'
        } else {
            '│'
        };
        s.push(sep);

        let cell = puzzle[cell_row][cell_col];
        let center_row = CELL_H / 2;
        let center_col = CELL_W / 2;

        for inner_col in 0..CELL_W {
            let ch = match cell {
                Cell::Given(v) | Cell::UserInput(v) => {
                    if inner_row == center_row && inner_col == center_col {
                        char::from_digit(v as u32, 10).unwrap()
                    } else {
                        ' '
                    }
                }
                Cell::Empty => ' ',
            };
            s.push(ch);
        }
    }
    s.push('┃');

    Line::from(s)
}
