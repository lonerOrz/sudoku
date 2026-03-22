// game.rs: 游戏逻辑层

use crate::input::playing::Action;
use crate::state::{AppState, HistoryEntry, PencilMarks};
use sudoku_core::{Cell, Difficulty, clear_peers, compute_conflicts, generate, has_empty};

pub fn init_menu(difficulty: Difficulty) -> AppState {
    AppState::Menu { difficulty }
}

pub fn start_game(difficulty: Difficulty) -> AppState {
    AppState::Playing(Game::new(difficulty))
}

pub fn update(state: &mut AppState, action: Action) {
    match action {
        Action::Quit => {
            *state = AppState::Menu {
                difficulty: Difficulty::Easy,
            };
        }
        Action::Pause => {
            if let AppState::Playing(game) = state {
                game.toggle_pause();
            }
        }
        Action::TogglePencilMode => {
            if let AppState::Playing(game) = state {
                game.toggle_pencil_mode();
            }
        }
        Action::ToggleHintMode => {
            if let AppState::Playing(game) = state {
                game.toggle_hint_mode();
            }
        }
        Action::PlaceHint => {
            if let AppState::Playing(game) = state
                && game.is_hint_mode()
                && !game.is_paused()
                && let Some(new_state) = game.place_hint()
            {
                *state = new_state;
            }
        }
        Action::Undo => {
            if let AppState::Playing(game) = state
                && let Some(new_state) = game.undo()
            {
                *state = new_state;
            }
        }
        Action::MoveLeft => {
            if let AppState::Playing(game) = state {
                game.move_cursor(0, -1);
            }
        }
        Action::MoveRight => {
            if let AppState::Playing(game) = state {
                game.move_cursor(0, 1);
            }
        }
        Action::MoveUp => {
            if let AppState::Playing(game) = state {
                game.move_cursor(-1, 0);
            }
        }
        Action::MoveDown => {
            if let AppState::Playing(game) = state {
                game.move_cursor(1, 0);
            }
        }
        Action::PlaceNumber(n) => {
            if let AppState::Playing(game) = state
                && let Some(new_state) = game.place_number(n)
            {
                *state = new_state;
            }
        }
        Action::Erase => {
            if let AppState::Playing(game) = state {
                game.erase();
            }
        }
    }
}

pub struct Game {
    puzzle: sudoku_core::Grid,
    solution: sudoku_core::Solution,
    pencil_marks: PencilMarks,
    pencil_mode: bool,
    hint_mode: bool,
    cursor_row: usize,
    cursor_col: usize,
    conflicts: sudoku_core::Conflicts,
    difficulty: Difficulty,
    mistakes: u8,
    hints_used: u8,
    undo_used: u8,
    start_time: std::time::Instant,
    elapsed_secs: u64,
    paused: bool,
    history: Vec<HistoryEntry>,
}

impl Game {
    pub fn new(difficulty: Difficulty) -> Self {
        let (puzzle, solution) = generate(difficulty);
        let conflicts = compute_conflicts(&puzzle);
        let pencil_marks: PencilMarks =
            std::array::from_fn(|_| std::array::from_fn(|_| Vec::new()));

        Self {
            puzzle,
            solution,
            pencil_marks,
            pencil_mode: false,
            hint_mode: false,
            cursor_row: 4,
            cursor_col: 4,
            conflicts,
            difficulty,
            mistakes: 0,
            hints_used: 0,
            undo_used: 0,
            start_time: std::time::Instant::now(),
            elapsed_secs: 0,
            paused: false,
            history: vec![],
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn is_hint_mode(&self) -> bool {
        self.hint_mode
    }

    pub fn is_pencil_mode(&self) -> bool {
        self.pencil_mode
    }

    pub fn puzzle(&self) -> &sudoku_core::Grid {
        &self.puzzle
    }

    pub fn solution(&self) -> &sudoku_core::Solution {
        &self.solution
    }

    pub fn pencil_marks(&self) -> &PencilMarks {
        &self.pencil_marks
    }

    pub fn cursor_row(&self) -> usize {
        self.cursor_row
    }

    pub fn cursor_col(&self) -> usize {
        self.cursor_col
    }

    pub fn conflicts(&self) -> &sudoku_core::Conflicts {
        &self.conflicts
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn mistakes(&self) -> u8 {
        self.mistakes
    }

    pub fn hints_used(&self) -> u8 {
        self.hints_used
    }

    pub fn undo_used(&self) -> u8 {
        self.undo_used
    }

    pub fn elapsed_secs(&self) -> u64 {
        if self.paused {
            self.elapsed_secs
        } else {
            self.start_time.elapsed().as_secs()
        }
    }

    pub fn toggle_pause(&mut self) {
        if self.paused {
            self.start_time =
                std::time::Instant::now() - std::time::Duration::from_secs(self.elapsed_secs);
        } else {
            self.elapsed_secs = self.start_time.elapsed().as_secs();
        }
        self.paused = !self.paused;
    }

    pub fn toggle_pencil_mode(&mut self) {
        self.pencil_mode = !self.pencil_mode;
    }

    pub fn toggle_hint_mode(&mut self) {
        self.hint_mode = !self.hint_mode;
    }

    pub fn move_cursor(&mut self, dr: i32, dc: i32) {
        if self.paused {
            return;
        }
        self.cursor_row = ((self.cursor_row as i32) + dr).clamp(0, 8) as usize;
        self.cursor_col = ((self.cursor_col as i32) + dc).clamp(0, 8) as usize;
    }

    pub fn place_number(&mut self, n: u8) -> Option<AppState> {
        if self.paused || self.hint_mode {
            return None;
        }

        let r = self.cursor_row;
        let c = self.cursor_col;

        if self.pencil_mode {
            if matches!(self.puzzle[r][c], Cell::Empty) {
                let marks = &mut self.pencil_marks[r][c];
                if marks.contains(&n) {
                    marks.retain(|&v| v != n);
                } else {
                    marks.push(n);
                    marks.sort();
                }
            }
            return None;
        }

        let cell_value = self.puzzle[r][c];
        let already_has_n = matches!(cell_value, Cell::UserInput(v) if v == n);
        if already_has_n || matches!(cell_value, Cell::Given(_)) {
            return None;
        }

        let entry = HistoryEntry {
            puzzle: self.puzzle,
            pencil_marks: self.pencil_marks.clone(),
            cursor_row: self.cursor_row,
            cursor_col: self.cursor_col,
            mistakes: self.mistakes,
        };
        self.history.push(entry);

        self.puzzle[r][c] = Cell::UserInput(n);
        self.pencil_marks[r][c].clear();
        clear_peers(&mut self.pencil_marks, r, c, n);
        if self.solution[r][c] != n {
            self.mistakes += 1;
        }
        self.conflicts = compute_conflicts(&self.puzzle);

        if self.mistakes >= 5 {
            return Some(AppState::Failed {
                difficulty: self.difficulty,
                elapsed_secs: self.start_time.elapsed().as_secs(),
            });
        }

        if !has_conflicts(&self.conflicts) && !has_empty(&self.puzzle) {
            return Some(AppState::Won {
                difficulty: self.difficulty,
                elapsed_secs: self.start_time.elapsed().as_secs(),
            });
        }

        None
    }

    pub fn erase(&mut self) {
        if self.paused || self.hint_mode || self.pencil_mode {
            if !self.pencil_marks[self.cursor_row][self.cursor_col].is_empty() {
                self.pencil_marks[self.cursor_row][self.cursor_col].clear();
            }
            return;
        }

        let r = self.cursor_row;
        let c = self.cursor_col;

        if !matches!(self.puzzle[r][c], Cell::UserInput(_)) {
            return;
        }

        let entry = HistoryEntry {
            puzzle: self.puzzle,
            pencil_marks: self.pencil_marks.clone(),
            cursor_row: self.cursor_row,
            cursor_col: self.cursor_col,
            mistakes: self.mistakes,
        };
        self.history.push(entry);

        self.puzzle[r][c] = Cell::Empty;
        self.pencil_marks[r][c].clear();
        self.conflicts = compute_conflicts(&self.puzzle);
    }

    pub fn place_hint(&mut self) -> Option<AppState> {
        if !self.hint_mode || self.paused {
            return None;
        }

        let r = self.cursor_row;
        let c = self.cursor_col;

        if !matches!(self.puzzle[r][c], Cell::Empty) {
            return None;
        }

        let v = self.solution[r][c];

        let entry = HistoryEntry {
            puzzle: self.puzzle,
            pencil_marks: self.pencil_marks.clone(),
            cursor_row: self.cursor_row,
            cursor_col: self.cursor_col,
            mistakes: self.mistakes,
        };
        self.history.push(entry);

        self.puzzle[r][c] = Cell::UserInput(v);
        self.pencil_marks[r][c].clear();
        clear_peers(&mut self.pencil_marks, r, c, v);
        self.conflicts = compute_conflicts(&self.puzzle);
        self.hints_used += 1;
        self.hint_mode = false;

        None
    }

    pub fn undo(&mut self) -> Option<AppState> {
        let entry = self.history.pop()?;
        self.puzzle = entry.puzzle;
        self.pencil_marks = entry.pencil_marks;
        self.cursor_row = entry.cursor_row;
        self.cursor_col = entry.cursor_col;
        self.mistakes = entry.mistakes;
        self.undo_used += 1;
        self.conflicts = compute_conflicts(&self.puzzle);
        None
    }
}

fn has_conflicts(conflicts: &sudoku_core::Conflicts) -> bool {
    conflicts
        .iter()
        .any(|row| row.iter().any(|ct| !ct.is_empty()))
}
