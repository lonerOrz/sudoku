// main.rs: 应用入口和状态管理

mod constants;
mod input;
mod state;
mod terminal;
mod ui;

use state::{AppState, HistoryEntry, PencilMarks};
use sudoku_core::{
    Cell, Conflicts, Difficulty, clear_peers, compute_conflicts, generate, has_empty,
};

fn main() -> std::io::Result<()> {
    let mut terminal = terminal::init()?;
    let mut state = AppState::Menu {
        difficulty: Difficulty::Easy,
    };

    loop {
        terminal.draw(|f| ui::draw(&state, f))?;

        if crossterm::event::poll(std::time::Duration::from_millis(500))? {
            let event = crossterm::event::read()?;
            if let crossterm::event::Event::Key(key) = event {
                match &state {
                    AppState::Menu { .. } => {
                        if let Some(action) = input::menu::handle(key.code) {
                            match action {
                                input::menu::Action::PrevDifficulty => {
                                    if let AppState::Menu { difficulty } = &mut state {
                                        *difficulty = constants::cycle(*difficulty, false);
                                    }
                                }
                                input::menu::Action::NextDifficulty => {
                                    if let AppState::Menu { difficulty } = &mut state {
                                        *difficulty = constants::cycle(*difficulty, true);
                                    }
                                }
                                input::menu::Action::Start => {
                                    if let AppState::Menu { difficulty } = &state {
                                        let (puzzle, solution) = generate(*difficulty);
                                        let conflicts = compute_conflicts(&puzzle);
                                        let pencil_marks: PencilMarks = std::array::from_fn(|_| {
                                            std::array::from_fn(|_| Vec::new())
                                        });
                                        state = AppState::Playing {
                                            puzzle,
                                            solution,
                                            pencil_marks,
                                            pencil_mode: false,
                                            hint_mode: false,
                                            cursor_row: 4,
                                            cursor_col: 4,
                                            conflicts,
                                            difficulty: *difficulty,
                                            mistakes: 0,
                                            hints_used: 0,
                                            undo_used: 0,
                                            start_time: std::time::Instant::now(),
                                            elapsed_secs: 0,
                                            paused: false,
                                            history: vec![],
                                        };
                                    }
                                }
                                input::menu::Action::Back => break,
                            }
                        }
                    }
                    AppState::Won { .. } | AppState::Failed { .. } => {
                        if let Some(input::playing::Action::Quit) = input::playing::handle(key.code)
                        {
                            state = AppState::Menu {
                                difficulty: Difficulty::Easy,
                            };
                        }
                    }
                    AppState::Playing {
                        paused, hint_mode, ..
                    } => {
                        if let Some(action) = input::playing::handle(key.code) {
                            match action {
                                input::playing::Action::Quit => {
                                    state = AppState::Menu {
                                        difficulty: Difficulty::Easy,
                                    };
                                }
                                input::playing::Action::Pause => {
                                    if let AppState::Playing {
                                        paused,
                                        start_time,
                                        elapsed_secs,
                                        ..
                                    } = &mut state
                                    {
                                        if *paused {
                                            *start_time = std::time::Instant::now()
                                                - std::time::Duration::from_secs(*elapsed_secs);
                                        } else {
                                            *elapsed_secs = start_time.elapsed().as_secs();
                                        }
                                        *paused = !*paused;
                                    }
                                }
                                input::playing::Action::Undo => {
                                    handle_undo(&mut state);
                                }
                                input::playing::Action::TogglePencilMode => {
                                    if let AppState::Playing { pencil_mode, .. } = &mut state {
                                        *pencil_mode = !*pencil_mode;
                                    }
                                }
                                input::playing::Action::ToggleHintMode => {
                                    if let AppState::Playing { hint_mode, .. } = &mut state {
                                        *hint_mode = !*hint_mode;
                                    }
                                }
                                input::playing::Action::PlaceHint => {
                                    if *hint_mode {
                                        handle_place_hint(&mut state);
                                    }
                                }
                                input::playing::Action::MoveLeft
                                | input::playing::Action::MoveRight
                                | input::playing::Action::MoveUp
                                | input::playing::Action::MoveDown => {
                                    if !*paused {
                                        handle_playing_action(&mut state, action);
                                    }
                                }
                                _ if !*paused && !*hint_mode => {
                                    handle_playing_action(&mut state, action);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    terminal::cleanup()
}

fn handle_place_hint(state: &mut AppState) {
    let r;
    let c;
    let v;
    let is_empty;

    {
        if let AppState::Playing {
            puzzle,
            solution,
            cursor_row,
            cursor_col,
            ..
        } = state
        {
            r = *cursor_row;
            c = *cursor_col;
            is_empty = matches!(puzzle[r][c], Cell::Empty);
            v = solution[r][c];
        } else {
            return;
        }
    }

    if is_empty {
        let history_entry = save_history(state);
        if let AppState::Playing {
            puzzle,
            pencil_marks,
            conflicts,
            history,
            hint_mode,
            hints_used,
            ..
        } = state
        {
            if let Some(entry) = history_entry {
                history.push(entry);
            }
            puzzle[r][c] = Cell::UserInput(v);
            pencil_marks[r][c].clear();
            clear_peers(pencil_marks, r, c, v);
            *conflicts = compute_conflicts(puzzle);
            *hints_used += 1;
            *hint_mode = false;
        }
    }
}

fn handle_undo(state: &mut AppState) {
    if let AppState::Playing {
        puzzle,
        pencil_marks,
        cursor_row,
        cursor_col,
        mistakes,
        undo_used,
        history,
        conflicts,
        ..
    } = state
        && let Some(entry) = history.pop()
    {
        *puzzle = entry.puzzle;
        *pencil_marks = entry.pencil_marks;
        *cursor_row = entry.cursor_row;
        *cursor_col = entry.cursor_col;
        *mistakes = entry.mistakes;
        *undo_used += 1;
        *conflicts = compute_conflicts(puzzle);
    }
}

fn save_history(state: &AppState) -> Option<HistoryEntry> {
    if let AppState::Playing {
        puzzle,
        pencil_marks,
        cursor_row,
        cursor_col,
        mistakes,
        ..
    } = state
    {
        Some(HistoryEntry {
            puzzle: *puzzle,
            pencil_marks: pencil_marks.clone(),
            cursor_row: *cursor_row,
            cursor_col: *cursor_col,
            mistakes: *mistakes,
        })
    } else {
        None
    }
}

fn has_conflicts(conflicts: &Conflicts) -> bool {
    conflicts
        .iter()
        .any(|row| row.iter().any(|ct| !ct.is_empty()))
}

fn handle_playing_action(state: &mut AppState, action: input::playing::Action) {
    match action {
        input::playing::Action::MoveLeft => {
            if let AppState::Playing { cursor_col, .. } = state
                && *cursor_col > 0
            {
                *cursor_col -= 1;
            }
        }
        input::playing::Action::MoveRight => {
            if let AppState::Playing { cursor_col, .. } = state
                && *cursor_col < 8
            {
                *cursor_col += 1;
            }
        }
        input::playing::Action::MoveUp => {
            if let AppState::Playing { cursor_row, .. } = state
                && *cursor_row > 0
            {
                *cursor_row -= 1;
            }
        }
        input::playing::Action::MoveDown => {
            if let AppState::Playing { cursor_row, .. } = state
                && *cursor_row < 8
            {
                *cursor_row += 1;
            }
        }
        input::playing::Action::PlaceNumber(n) => {
            let in_pencil_mode = if let AppState::Playing { pencil_mode, .. } = state {
                *pencil_mode
            } else {
                false
            };

            if in_pencil_mode {
                if let AppState::Playing {
                    puzzle,
                    pencil_marks,
                    cursor_row,
                    cursor_col,
                    ..
                } = state
                    && matches!(puzzle[*cursor_row][*cursor_col], Cell::Empty)
                {
                    let marks = &mut pencil_marks[*cursor_row][*cursor_col];
                    if marks.contains(&n) {
                        marks.retain(|&v| v != n);
                    } else {
                        marks.push(n);
                        marks.sort();
                    }
                }
            } else {
                let history_entry = save_history(state);
                if let AppState::Playing {
                    puzzle,
                    solution,
                    pencil_marks,
                    cursor_row,
                    cursor_col,
                    conflicts,
                    difficulty,
                    mistakes,
                    start_time,
                    history,
                    ..
                } = state
                {
                    let cell = &mut puzzle[*cursor_row][*cursor_col];
                    let already_has_n = matches!(cell, Cell::UserInput(v) if *v == n);
                    if !already_has_n && !matches!(cell, Cell::Given(_)) {
                        if let Some(entry) = history_entry {
                            history.push(entry);
                        }

                        *cell = Cell::UserInput(n);
                        pencil_marks[*cursor_row][*cursor_col].clear();
                        clear_peers(pencil_marks, *cursor_row, *cursor_col, n);
                        if solution[*cursor_row][*cursor_col] != n {
                            *mistakes += 1;
                        }
                        *conflicts = compute_conflicts(puzzle);
                        if *mistakes >= 5 {
                            *state = AppState::Failed {
                                difficulty: *difficulty,
                                elapsed_secs: start_time.elapsed().as_secs(),
                            };
                        } else if !has_conflicts(conflicts) && !has_empty(puzzle) {
                            *state = AppState::Won {
                                difficulty: *difficulty,
                                elapsed_secs: start_time.elapsed().as_secs(),
                            };
                        }
                    }
                }
            }
        }
        input::playing::Action::Erase => {
            let history_entry = save_history(state);
            if let AppState::Playing {
                puzzle,
                pencil_marks,
                cursor_row,
                cursor_col,
                conflicts,
                history,
                ..
            } = state
            {
                let cell = &mut puzzle[*cursor_row][*cursor_col];
                if matches!(cell, Cell::UserInput(_)) {
                    if let Some(entry) = history_entry {
                        history.push(entry);
                    }
                    *cell = Cell::Empty;
                    pencil_marks[*cursor_row][*cursor_col].clear();
                    *conflicts = compute_conflicts(puzzle);
                } else if !pencil_marks[*cursor_row][*cursor_col].is_empty() {
                    pencil_marks[*cursor_row][*cursor_col].clear();
                }
            }
        }
        _ => {}
    }
}
