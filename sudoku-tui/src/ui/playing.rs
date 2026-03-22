// ui/playing.rs: 游戏界面

use crate::state::{Control, PencilMarks};
use ratatui::{
    prelude::{Alignment, Constraint, Frame, Layout, Line, Span, Style},
    style::Color,
    widgets::{Block, BorderType, Paragraph},
};
use sudoku_core::{Cell, Conflicts, Difficulty, Grid};

const CELL_W: usize = 7;
const CELL_H: usize = 3;

pub struct GameInfo {
    pub difficulty: Difficulty,
    pub mistakes: u8,
    pub hints_used: u8,
    pub elapsed_secs: u64,
    pub paused: bool,
    pub pencil_mode: bool,
    pub hint_mode: bool,
}

impl GameInfo {
    pub fn render(&self) -> Vec<Line<'static>> {
        let time_str = format_time(self.elapsed_secs);
        let mode = if self.paused {
            Span::styled("PAUSED", Style::default().fg(Color::Yellow))
        } else if self.pencil_mode {
            Span::styled("PENCIL", Style::default().fg(Color::Green))
        } else if self.hint_mode {
            Span::styled("HINT", Style::default().fg(Color::Cyan))
        } else {
            Span::raw("Normal")
        };

        vec![
            Line::from(vec![Span::styled(
                "Info",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw(format!(
                "Difficulty: {}",
                self.difficulty.label()
            ))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw(format!("Time: {}", time_str))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw(format!("Mistakes: {}/5", self.mistakes))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw(format!("Hints used: {}", self.hints_used))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Mode: "), mode]),
        ]
    }
}

pub struct CellRenderParams<'a> {
    pub puzzle: &'a Grid,
    pub solution: &'a [[u8; 9]; 9],
    pub pencil_marks: &'a PencilMarks,
    pub pencil_mode: bool,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub conflicts: &'a Conflicts,
}

pub struct DrawParams<'a> {
    pub puzzle: &'a Grid,
    pub solution: &'a [[u8; 9]; 9],
    pub pencil_marks: &'a PencilMarks,
    pub pencil_mode: bool,
    pub hint_mode: bool,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub conflicts: &'a Conflicts,
    pub mistakes: u8,
    pub hints_used: u8,
    pub difficulty: Difficulty,
    pub elapsed_secs: u64,
    pub paused: bool,
    pub controls: &'a [Control],
}

pub fn draw(f: &mut Frame, params: &DrawParams) {
    let area = f.size();

    let main_chunks = Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).split(area);

    let cell_params = CellRenderParams {
        puzzle: params.puzzle,
        solution: params.solution,
        pencil_marks: params.pencil_marks,
        pencil_mode: params.pencil_mode,
        cursor_row: params.cursor_row,
        cursor_col: params.cursor_col,
        conflicts: params.conflicts,
    };
    let grid = render_grid(&cell_params);
    let grid_width = grid
        .iter()
        .map(|l| l.to_string().len() as u16)
        .max()
        .unwrap_or(0);
    let grid_height = grid.len() as u16;

    let info = GameInfo {
        difficulty: params.difficulty,
        mistakes: params.mistakes,
        hints_used: params.hints_used,
        elapsed_secs: params.elapsed_secs,
        paused: params.paused,
        pencil_mode: params.pencil_mode,
        hint_mode: params.hint_mode,
    }
    .render();
    let info_height = info.len() as u16;

    let total_width = grid_width + 20;

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

    let hints = render_controls(params.controls);
    let hints_rect = calc_hints_rect(main_chunks[1], &hints);
    f.render_widget(
        Paragraph::new(vec![hints])
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: false }),
        hints_rect,
    );

    if params.paused {
        let popup = center_rect(30, 7, grid_v[1]);
        f.render_widget(ratatui::widgets::Clear, popup);
        f.render_widget(render_pause_popup(params.elapsed_secs), popup);
    }
}

fn render_pause_popup(elapsed_secs: u64) -> Paragraph<'static> {
    let block = Block::bordered()
        .title(" Paused ")
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Yellow));

    Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "PAUSED",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::raw(format!(
            "Time: {}",
            format_time(elapsed_secs)
        ))]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Press "),
            Span::styled("Space", Style::default().fg(Color::Cyan)),
            Span::raw(" to resume"),
        ]),
    ])
    .block(block)
    .alignment(Alignment::Center)
}

pub fn draw_won(f: &mut Frame, difficulty: Difficulty, elapsed_secs: u64, controls: &[Control]) {
    let area = f.size();

    let label = difficulty.label();
    let time_str = format_time(elapsed_secs);

    let mut content = vec![
        Line::from(vec![Span::styled(
            "Congratulations!",
            Style::default()
                .fg(Color::Green)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::raw(format!("You completed {}!", label))]),
        Line::from(vec![Span::raw(format!("Time: {}", time_str))]),
        Line::from(vec![Span::raw("")]),
    ];

    content.push(render_controls_line(controls));

    let v_chunks = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(content.len() as u16),
        Constraint::Min(0),
    ])
    .split(area);

    let paragraph = Paragraph::new(content).alignment(Alignment::Center);

    f.render_widget(paragraph, v_chunks[1]);
}

pub fn draw_failed(f: &mut Frame, difficulty: Difficulty, elapsed_secs: u64, controls: &[Control]) {
    let area = f.size();

    let label = difficulty.label();
    let time_str = format_time(elapsed_secs);

    let mut content = vec![
        Line::from(vec![Span::styled(
            "Game Over",
            Style::default()
                .fg(Color::Red)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw("")]),
        Line::from(vec![Span::raw(format!("Too many mistakes on {}!", label))]),
        Line::from(vec![Span::raw(format!("Time: {}", time_str))]),
        Line::from(vec![Span::raw("")]),
    ];

    content.push(render_controls_line(controls));

    let v_chunks = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(content.len() as u16),
        Constraint::Min(0),
    ])
    .split(area);

    let paragraph = Paragraph::new(content).alignment(Alignment::Center);

    f.render_widget(paragraph, v_chunks[1]);
}

fn render_controls_line(controls: &[Control]) -> Line<'static> {
    let mut spans = Vec::new();
    for (i, ctrl) in controls.iter().enumerate() {
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

fn center_rect(width: u16, height: u16, area: ratatui::prelude::Rect) -> ratatui::prelude::Rect {
    let vert = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(height),
        Constraint::Min(0),
    ])
    .split(area);

    let horiz = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Length(width),
        Constraint::Min(0),
    ])
    .split(vert[1]);

    horiz[1]
}

fn calc_hints_rect(
    bottom_area: ratatui::prelude::Rect,
    hints: &Line<'_>,
) -> ratatui::prelude::Rect {
    let width = hints.width() as u16;
    let horiz = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Length(width),
        Constraint::Min(0),
    ])
    .split(bottom_area);

    let vert = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Min(0),
    ])
    .split(horiz[1]);

    vert[1]
}

fn format_time(total_secs: u64) -> String {
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;
    format!("{:02}:{:02}:{:02}", hours, mins, secs)
}

fn render_controls(controls: &[Control]) -> Line<'static> {
    let mut spans = Vec::new();
    for (i, ctrl) in controls.iter().enumerate() {
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

fn render_grid(params: &CellRenderParams) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    lines.push(h_line(LineKind::Top));

    for cell_row in 0..9 {
        for inner_row in 0..CELL_H {
            lines.push(content_line(params, cell_row, inner_row));
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

fn content_line(params: &CellRenderParams, cell_row: usize, inner_row: usize) -> Line<'static> {
    let mut spans = Vec::new();
    let puzzle = params.puzzle;
    let solution = params.solution;
    let pencil_marks = params.pencil_marks;
    let pencil_mode = params.pencil_mode;
    let cursor_row = params.cursor_row;
    let cursor_col = params.cursor_col;
    let conflicts = params.conflicts;

    for (cell_col, _) in puzzle[cell_row].iter().enumerate().take(9) {
        let is_cursor = cell_row == cursor_row && cell_col == cursor_col;
        let cell = puzzle[cell_row][cell_col];
        let conflict_type = conflicts[cell_row][cell_col];
        let has_conflict = !conflict_type.is_empty();

        let is_wrong = if let Cell::UserInput(v) = cell {
            solution[cell_row][cell_col] != v
        } else {
            false
        };

        let bg = if is_cursor {
            if pencil_mode {
                Color::Green
            } else {
                Color::Blue
            }
        } else if has_conflict && !is_wrong {
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

        let content = if inner_row == 1 {
            match cell {
                Cell::Given(v) | Cell::UserInput(v) => {
                    format!("   {}   ", char::from_digit(v as u32, 10).unwrap())
                }
                Cell::Empty => {
                    let marks = &pencil_marks[cell_row][cell_col];
                    if marks.is_empty() {
                        if is_cursor {
                            "   ·   ".to_string()
                        } else {
                            "       ".to_string()
                        }
                    } else {
                        let base = (inner_row * 3 + 1) as u8;
                        let c0 = if marks.contains(&base) {
                            (b'0' + base) as char
                        } else {
                            ' '
                        };
                        let c1 = if marks.contains(&(base + 1)) {
                            (b'0' + base + 1) as char
                        } else {
                            ' '
                        };
                        let c2 = if marks.contains(&(base + 2)) {
                            (b'0' + base + 2) as char
                        } else {
                            ' '
                        };
                        format!(" {} {} {} ", c0, c1, c2)
                    }
                }
            }
        } else {
            let marks = &pencil_marks[cell_row][cell_col];
            if marks.is_empty() {
                "       ".to_string()
            } else {
                let base = (inner_row * 3 + 1) as u8;
                let c0 = if marks.contains(&base) {
                    (b'0' + base) as char
                } else {
                    ' '
                };
                let c1 = if marks.contains(&(base + 1)) {
                    (b'0' + base + 1) as char
                } else {
                    ' '
                };
                let c2 = if marks.contains(&(base + 2)) {
                    (b'0' + base + 2) as char
                } else {
                    ' '
                };
                format!(" {} {} {} ", c0, c1, c2)
            }
        };

        let fg = if is_wrong {
            Color::Red
        } else {
            match cell {
                Cell::Given(_) => Color::White,
                Cell::UserInput(_) => Color::Cyan,
                Cell::Empty => {
                    if is_cursor && !pencil_marks[cell_row][cell_col].is_empty() {
                        Color::Cyan
                    } else {
                        Color::DarkGray
                    }
                }
            }
        };
        spans.push(Span::styled(content, Style::default().fg(fg).bg(bg)));
    }
    spans.push(Span::styled("┃", Style::default().fg(Color::White)));

    Line::from(spans)
}
