// ui/playing.rs: 游戏界面

use crate::state::{Control, PencilMarks};
use ratatui::{
    prelude::{Alignment, Constraint, Frame, Layout, Line, Span, Style},
    style::Color,
    widgets::{Block, BorderType, Paragraph},
};
use sudoku_core::{Cell, Conflicts, Difficulty, Grid};

const GRID_HEIGHT: u16 = 37;

pub struct GameInfo {
    pub difficulty: Difficulty,
    pub mistakes: u8,
    pub hints_used: u8,
    pub undo_used: u8,
    pub elapsed_secs: u64,
    pub paused: bool,
    pub pencil_mode: bool,
    pub hint_mode: bool,
    pub technique_name: Option<String>,
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

        let mut lines = vec![
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
            Line::from(vec![Span::raw(format!("Undo used: {}", self.undo_used))]),
            Line::from(vec![Span::raw("")]),
            Line::from(vec![Span::raw("Mode: "), mode]),
        ];

        if let Some(ref technique) = self.technique_name {
            lines.push(Line::from(vec![Span::raw("")]));
            lines.push(Line::from(vec![Span::styled(
                format!("Technique: {}", technique),
                Style::default().fg(Color::Cyan),
            )]));
        }

        lines
    }
}

pub struct CellRenderParams<'a> {
    pub puzzle: &'a Grid,
    pub solution: &'a [[u8; 9]; 9],
    pub pencil_marks: &'a PencilMarks,
    pub pencil_mode: bool,
    pub hint_mode: bool,
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
    pub undo_used: u8,
    pub difficulty: Difficulty,
    pub elapsed_secs: u64,
    pub paused: bool,
    pub controls: &'a [Control],
    pub candidates: Option<&'a [bool; 9]>,
    pub technique_name: Option<String>,
}

pub fn draw(f: &mut Frame, params: &DrawParams) {
    let area = f.size();

    let main_chunks = Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).split(area);

    let cell_params = CellRenderParams {
        puzzle: params.puzzle,
        solution: params.solution,
        pencil_marks: params.pencil_marks,
        pencil_mode: params.pencil_mode,
        hint_mode: params.hint_mode,
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
        undo_used: params.undo_used,
        elapsed_secs: params.elapsed_secs,
        paused: params.paused,
        pencil_mode: params.pencil_mode,
        hint_mode: params.hint_mode,
        technique_name: params.technique_name.clone(),
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

    let (grid_rect, info_rect) = if params.candidates.is_some() {
        let layout = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(grid_height),
            Constraint::Length(5),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(side_chunks[0]);
        (layout[1], layout[2])
    } else {
        let layout = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(grid_height),
            Constraint::Min(0),
        ])
        .split(side_chunks[0]);
        (layout[1], layout[2])
    };

    let info_v = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(info_height),
        Constraint::Min(0),
    ])
    .split(side_chunks[1]);

    f.render_widget(Paragraph::new(grid).alignment(Alignment::Center), grid_rect);

    if let Some(candidates) = params.candidates {
        let candidate_bar = render_candidate_bar(candidates);
        f.render_widget(
            Paragraph::new(candidate_bar)
                .alignment(Alignment::Center)
                .wrap(ratatui::widgets::Wrap { trim: false }),
            info_rect,
        );
    }

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
        let popup = center_rect(30, 7, grid_rect);
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

fn render_candidate_bar(candidates: &[bool; 9]) -> Vec<Line<'static>> {
    let top: Vec<Span> = vec![Span::raw("         ")];

    let mut bottom = vec![Span::raw("  ")];
    for i in 1..=9 {
        let is_candidate = candidates[i - 1];
        let num_str = i.to_string();

        if is_candidate {
            bottom.push(Span::styled(
                format!("  [{}]  ", num_str),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            ));
        } else {
            bottom.push(Span::styled(
                format!("   {}   ", num_str),
                Style::default().fg(Color::DarkGray),
            ));
        }
    }
    bottom.push(Span::raw("  "));

    vec![Line::from(top), Line::from(bottom)]
}

fn render_grid(params: &CellRenderParams) -> Vec<Line<'static>> {
    let mut lines = Vec::with_capacity(GRID_HEIGHT as usize);

    for visual_row in 0..GRID_HEIGHT {
        let row_kind = classify_row(visual_row);

        match row_kind {
            RowKind::ThickBorder(border_idx) => {
                lines.push(thick_horizontal_line(border_idx));
            }
            RowKind::ThinBorder => {
                lines.push(thin_horizontal_line());
            }
            RowKind::CellRow(grid_row, sub_row) => {
                lines.push(render_cell_row(params, grid_row, sub_row));
            }
        }
    }

    lines
}

#[derive(Debug)]
enum RowKind {
    ThickBorder(u8),
    ThinBorder,
    CellRow(usize, usize),
}

fn classify_row(visual: u16) -> RowKind {
    match visual {
        0 => RowKind::ThickBorder(0),
        12 => RowKind::ThickBorder(1),
        24 => RowKind::ThickBorder(2),
        36 => RowKind::ThickBorder(3),
        4 | 8 | 16 | 20 | 28 | 32 => RowKind::ThinBorder,
        _ => {
            let section = (visual / 12) as usize;
            let offset = visual % 12;
            let (cell_in_section, sub_row) = if offset <= 3 {
                (0, (offset - 1) as usize)
            } else if offset <= 7 {
                (1, (offset - 5) as usize)
            } else {
                (2, (offset - 9) as usize)
            };
            let grid_row = section * 3 + cell_in_section;
            RowKind::CellRow(grid_row, sub_row)
        }
    }
}

enum ColKind {
    ThickBorder,
    ThinBorder,
    Cell(usize),
}

fn classify_col(seg: usize) -> ColKind {
    match seg {
        0 | 6 | 12 | 18 => ColKind::ThickBorder,
        2 | 4 | 8 | 10 | 14 | 16 => ColKind::ThinBorder,
        1 => ColKind::Cell(0),
        3 => ColKind::Cell(1),
        5 => ColKind::Cell(2),
        7 => ColKind::Cell(3),
        9 => ColKind::Cell(4),
        11 => ColKind::Cell(5),
        13 => ColKind::Cell(6),
        15 => ColKind::Cell(7),
        17 => ColKind::Cell(8),
        _ => ColKind::ThinBorder,
    }
}

fn thick_horizontal_line(border_idx: u8) -> Line<'static> {
    let (left, thick_cross, thin_cross, right) = match border_idx {
        0 => ('╔', '╦', '╤', '╗'),
        3 => ('╚', '╩', '╧', '╝'),
        _ => ('╠', '╬', '╪', '╣'),
    };

    let mut s = String::with_capacity(80);
    s.push(left);
    for box_idx in 0..3 {
        for cell_idx in 0..3 {
            s.push_str("═══════");
            if cell_idx < 2 {
                s.push(thin_cross);
            }
        }
        if box_idx < 2 {
            s.push(thick_cross);
        }
    }
    s.push(right);

    Line::from(Span::styled(s, Style::default().fg(Color::White)))
}

fn thin_horizontal_line() -> Line<'static> {
    let mut s = String::with_capacity(80);
    s.push('║');
    for box_idx in 0..3 {
        for cell_idx in 0..3 {
            s.push_str("───────");
            if cell_idx < 2 {
                s.push('┼');
            }
        }
        if box_idx < 2 {
            s.push('║');
        }
    }
    s.push('║');

    Line::from(Span::styled(s, Style::default().fg(Color::DarkGray)))
}

fn render_cell_row(params: &CellRenderParams, grid_row: usize, sub_row: usize) -> Line<'static> {
    let mut spans = Vec::with_capacity(19);
    for seg in 0..19 {
        let col_kind = classify_col(seg);
        match col_kind {
            ColKind::ThickBorder => {
                spans.push(Span::styled("║", Style::default().fg(Color::White)));
            }
            ColKind::ThinBorder => {
                spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
            }
            ColKind::Cell(grid_col) => {
                spans.push(render_grid_cell(params, grid_row, grid_col, sub_row));
            }
        }
    }
    Line::from(spans)
}

fn render_grid_cell(
    params: &CellRenderParams,
    grid_row: usize,
    grid_col: usize,
    sub_row: usize,
) -> Span<'static> {
    let cell = params.puzzle[grid_row][grid_col];
    let is_cursor = grid_row == params.cursor_row && grid_col == params.cursor_col;
    let conflict_type = params.conflicts[grid_row][grid_col];
    let has_conflict = !conflict_type.is_empty();
    let selected_value = params.puzzle[params.cursor_row][params.cursor_col].value();
    let is_same_value = cell.value().is_some() && cell.value() == selected_value && !is_cursor;

    let is_wrong = if let Cell::UserInput(v) = cell {
        params.solution[grid_row][grid_col] != v
    } else {
        false
    };

    let bg = if is_cursor {
        if params.pencil_mode {
            Color::Green
        } else if params.hint_mode {
            Color::Cyan
        } else {
            Color::Blue
        }
    } else if has_conflict && !is_wrong {
        Color::Red
    } else if is_same_value {
        Color::DarkGray
    } else {
        Color::Reset
    };

    render_cell_content(
        cell,
        &params.pencil_marks[grid_row][grid_col],
        sub_row,
        bg,
        is_cursor,
        is_wrong,
    )
}

fn render_cell_content(
    cell: Cell,
    pencil_marks: &[u8],
    sub_row: usize,
    bg: Color,
    is_cursor: bool,
    is_wrong: bool,
) -> Span<'static> {
    if is_wrong {
        let content = if sub_row == 1 {
            if let Cell::UserInput(v) = cell {
                format!("   {}   ", char::from_digit(v as u32, 10).unwrap_or('?'))
            } else {
                "       ".to_string()
            }
        } else {
            "       ".to_string()
        };
        return Span::styled(content, Style::default().fg(Color::Red).bg(bg));
    }

    let fg_for_bg = if bg == Color::Blue || bg == Color::Green {
        Color::Black
    } else if bg == Color::Magenta {
        Color::White
    } else {
        Color::Reset
    };

    let fg = match cell {
        Cell::Given(_) => Color::White,
        Cell::UserInput(_) => Color::LightGreen,
        Cell::Empty => {
            if is_cursor && !pencil_marks.is_empty() {
                Color::Cyan
            } else {
                Color::DarkGray
            }
        }
    };

    let final_fg = if is_cursor {
        if bg == Color::Green || bg == Color::Cyan {
            Color::Black
        } else {
            fg
        }
    } else if fg_for_bg != Color::Reset {
        fg_for_bg
    } else {
        fg
    };

    let content = match cell {
        Cell::Given(v) | Cell::UserInput(v) => {
            if sub_row == 1 {
                format!("   {}   ", char::from_digit(v as u32, 10).unwrap_or('?'))
            } else {
                "       ".to_string()
            }
        }
        Cell::Empty => {
            if pencil_marks.is_empty() {
                if is_cursor && sub_row == 1 {
                    "   ·   ".to_string()
                } else {
                    "       ".to_string()
                }
            } else {
                let base = (sub_row * 3 + 1) as u8;
                let c0 = if pencil_marks.contains(&base) {
                    (b'0' + base) as char
                } else {
                    ' '
                };
                let c1 = if pencil_marks.contains(&(base + 1)) {
                    (b'0' + base + 1) as char
                } else {
                    ' '
                };
                let c2 = if pencil_marks.contains(&(base + 2)) {
                    (b'0' + base + 2) as char
                } else {
                    ' '
                };
                format!(" {} {} {} ", c0, c1, c2)
            }
        }
    };

    Span::styled(content, Style::default().fg(final_fg).bg(bg))
}
