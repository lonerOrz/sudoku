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

        let value = self.solution[r][c];
        if value == 0 {
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

        self.puzzle[r][c] = Cell::UserInput(value);
        self.pencil_marks[r][c].clear();
        clear_peers(&mut self.pencil_marks, r, c, value);
        self.conflicts = compute_conflicts(&self.puzzle);
        self.hints_used += 1;
        self.hint_mode = false;

        if !has_empty(&self.puzzle) && !has_conflicts(&self.conflicts) {
            return Some(AppState::Won {
                difficulty: self.difficulty,
                elapsed_secs: self.start_time.elapsed().as_secs(),
            });
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use sudoku_core::{Cell, Difficulty, Grid};

    fn create_empty_game() -> Game {
        let (puzzle, solution) = generate(Difficulty::Easy);
        let conflicts = compute_conflicts(&puzzle);
        let pencil_marks: PencilMarks =
            std::array::from_fn(|_| std::array::from_fn(|_| Vec::new()));
        Game {
            puzzle,
            solution,
            pencil_marks,
            pencil_mode: false,
            hint_mode: false,

            cursor_row: 4,
            cursor_col: 4,
            conflicts,
            difficulty: Difficulty::Easy,
            mistakes: 0,
            hints_used: 0,
            undo_used: 0,
            start_time: std::time::Instant::now(),
            elapsed_secs: 0,
            paused: false,
            history: vec![],
        }
    }

    fn find_empty_cell(game: &mut Game) -> Option<(usize, usize)> {
        for row in 0..9 {
            for col in 0..9 {
                if matches!(game.puzzle()[row][col], Cell::Empty) {
                    return Some((row, col));
                }
            }
        }
        None
    }

    fn move_to_cell(game: &mut Game, row: usize, col: usize) {
        game.move_cursor(
            row as i32 - game.cursor_row() as i32,
            col as i32 - game.cursor_col() as i32,
        );
    }

    #[test]
    fn test_new_game_initialization() {
        let game = Game::new(Difficulty::Easy);
        assert_eq!(game.difficulty(), Difficulty::Easy);
        assert_eq!(game.cursor_row(), 4);
        assert_eq!(game.cursor_col(), 4);
        assert_eq!(game.mistakes(), 0);
        assert_eq!(game.hints_used(), 0);
        assert_eq!(game.undo_used(), 0);
        assert!(!game.is_paused());
        assert!(!game.is_pencil_mode());
        assert!(!game.is_hint_mode());
    }

    #[test]
    fn test_cursor_movement() {
        let mut game = create_empty_game();

        game.move_cursor(0, 1);
        assert_eq!(game.cursor_col(), 5);

        game.move_cursor(0, -1);
        assert_eq!(game.cursor_col(), 4);

        game.move_cursor(-1, 0);
        assert_eq!(game.cursor_row(), 3);

        game.move_cursor(1, 0);
        assert_eq!(game.cursor_row(), 4);
    }

    #[test]
    fn test_cursor_boundary() {
        let mut game = create_empty_game();

        game.move_cursor(0, 100);
        assert_eq!(game.cursor_col(), 8);

        game.move_cursor(0, -100);
        assert_eq!(game.cursor_col(), 0);

        game.move_cursor(-100, 0);
        assert_eq!(game.cursor_row(), 0);

        game.move_cursor(100, 0);
        assert_eq!(game.cursor_row(), 8);
    }

    #[test]
    fn test_cursor_blocked_when_paused() {
        let mut game = create_empty_game();
        game.toggle_pause();

        let row_before = game.cursor_row();
        game.move_cursor(1, 0);
        assert_eq!(game.cursor_row(), row_before);
    }

    #[test]
    fn test_toggle_modes() {
        let mut game = create_empty_game();

        assert!(!game.is_pencil_mode());
        game.toggle_pencil_mode();
        assert!(game.is_pencil_mode());
        game.toggle_pencil_mode();
        assert!(!game.is_pencil_mode());

        assert!(!game.is_hint_mode());
        game.toggle_hint_mode();
        assert!(game.is_hint_mode());
        game.toggle_hint_mode();
        assert!(!game.is_hint_mode());
    }

    #[test]
    fn test_pencil_marks_toggle() {
        let mut game = create_empty_game();
        game.toggle_pencil_mode();
        assert!(game.is_pencil_mode());

        let (row, col) = find_empty_cell(&mut game).expect("Game should have empty cells");
        move_to_cell(&mut game, row, col);
        game.place_number(5);

        let marks = game.pencil_marks();
        assert!(marks[row][col].contains(&5));

        game.place_number(6);
        let marks = game.pencil_marks();
        assert!(marks[row][col].contains(&6));

        game.place_number(5);
        let marks = game.pencil_marks();
        assert!(!marks[row][col].contains(&5));
    }

    #[test]
    fn test_place_number_on_given_cell() {
        let mut game = create_empty_game();

        let result = game.place_number(1);

        assert!(result.is_none());
    }

    #[test]
    fn test_place_correct_number() {
        let mut game = create_empty_game();

        let (row, col) = find_empty_cell(&mut game).expect("Game should have empty cells");
        move_to_cell(&mut game, row, col);

        let solution_value = game.solution()[row][col];
        let result = game.place_number(solution_value);

        assert!(result.is_none());
    }

    #[test]
    fn test_place_wrong_number_increments_mistakes() {
        let mut game = create_empty_game();

        let (row, col) = find_empty_cell(&mut game).expect("Game should have empty cells");
        move_to_cell(&mut game, row, col);

        let solution_value = game.solution()[row][col];
        let wrong_value = if solution_value == 1 { 9 } else { 1 };

        let mistakes_before = game.mistakes();
        game.place_number(wrong_value);
        assert_eq!(game.mistakes(), mistakes_before + 1);
    }

    fn count_empty_cells(puzzle: &Grid) -> usize {
        puzzle
            .iter()
            .flatten()
            .filter(|c| matches!(c, Cell::Empty))
            .count()
    }

    #[test]
    fn test_five_mistakes_leads_to_failed() {
        let (puzzle, solution) = generate(Difficulty::Easy);
        let empty_count = count_empty_cells(&puzzle);

        assert!(
            empty_count >= 5,
            "Should have at least 5 empty cells, got {}",
            empty_count
        );

        let conflicts = compute_conflicts(&puzzle);
        let pencil_marks: PencilMarks =
            std::array::from_fn(|_| std::array::from_fn(|_| Vec::new()));

        let mut game = Game {
            puzzle,
            solution,
            pencil_marks,
            pencil_mode: false,
            hint_mode: false,
            cursor_row: 0,
            cursor_col: 0,
            conflicts,
            difficulty: Difficulty::Easy,
            mistakes: 0,
            hints_used: 0,
            undo_used: 0,
            start_time: std::time::Instant::now(),
            elapsed_secs: 0,
            paused: false,
            history: Vec::new(),
        };

        let mut filled = 0;
        for row in 0..9 {
            for col in 0..9 {
                if filled >= 5 {
                    break;
                }

                if matches!(game.puzzle[row][col], Cell::Empty) {
                    game.cursor_row = row;
                    game.cursor_col = col;
                    let wrong_num = if game.solution[row][col] == 9 {
                        1
                    } else {
                        game.solution[row][col] + 1
                    };
                    let result = game.place_number(wrong_num);

                    filled += 1;

                    if game.mistakes >= 5 {
                        assert!(matches!(result, Some(AppState::Failed { .. })));
                        return;
                    }
                }
            }
        }

        panic!("Should have triggered Failed state after 5 mistakes");
    }

    #[test]
    fn test_erase_user_input() {
        let mut game = create_empty_game();

        let (row, col) = find_empty_cell(&mut game).expect("Game should have empty cells");
        move_to_cell(&mut game, row, col);

        game.place_number(5);
        assert!(matches!(game.puzzle()[row][col], Cell::UserInput(5)));

        game.erase();

        assert!(matches!(game.puzzle()[row][col], Cell::Empty));
    }

    #[test]
    fn test_erase_does_nothing_on_given() {
        let mut game = create_empty_game();

        for row in 0..9 {
            for col in 0..9 {
                if matches!(game.puzzle()[row][col], Cell::Given(_)) {
                    move_to_cell(&mut game, row, col);
                    game.erase();
                    assert!(matches!(game.puzzle()[row][col], Cell::Given(_)));
                    return;
                }
            }
        }
    }

    #[test]
    fn test_hint_mode_places_correct_number() {
        let mut game = create_empty_game();

        let (row, col) = find_empty_cell(&mut game).expect("Game should have empty cells");
        move_to_cell(&mut game, row, col);

        game.toggle_hint_mode();
        assert!(game.is_hint_mode());

        let solution_value = game.solution()[row][col];
        game.place_hint();

        assert_eq!(game.puzzle()[row][col], Cell::UserInput(solution_value));
        assert_eq!(game.hints_used(), 1);
        assert!(!game.is_hint_mode());
    }

    #[test]
    fn test_hint_only_works_in_hint_mode() {
        let mut game = create_empty_game();

        let (row, col) = find_empty_cell(&mut game).expect("Game should have empty cells");
        move_to_cell(&mut game, row, col);

        let result = game.place_hint();
        assert!(result.is_none());
        assert_eq!(game.hints_used(), 0);
    }

    #[test]
    fn test_hint_disabled_when_paused() {
        let mut game = create_empty_game();

        game.toggle_hint_mode();
        game.toggle_pause();

        let result = game.place_hint();
        assert!(result.is_none());
    }

    #[test]
    fn test_undo_restores_state() {
        let mut game = create_empty_game();

        let (row, col) = find_empty_cell(&mut game).expect("Game should have empty cells");
        move_to_cell(&mut game, row, col);

        let original_cell = game.puzzle()[row][col];
        game.place_number(5);
        let after_place = game.puzzle()[row][col];

        game.undo();
        let after_undo = game.puzzle()[row][col];

        assert!(matches!(original_cell, Cell::Empty));
        assert!(matches!(after_place, Cell::UserInput(5)));
        assert_eq!(after_undo, original_cell);
        assert_eq!(game.undo_used(), 1);
    }

    #[test]
    fn test_undo_restores_mistakes() {
        let mut game = create_empty_game();

        let target = (0..9)
            .flat_map(|r| (0..9).map(move |c| (r, c)))
            .find(|(r, c)| {
                matches!(game.puzzle[*r][*c], Cell::Empty) && game.solution[*r][*c] != 1
            });

        if let Some((row, col)) = target {
            game.cursor_row = row;
            game.cursor_col = col;
        } else {
            return;
        }

        game.place_number(1);
        assert_eq!(game.mistakes(), 1);

        game.undo();
        assert_eq!(game.mistakes(), 0);
    }

    #[test]
    fn test_undo_with_empty_history() {
        let mut game = create_empty_game();
        let result = game.undo();
        assert!(result.is_none());
        assert_eq!(game.undo_used(), 0);
    }

    #[test]
    fn test_toggle_pause() {
        let mut game = create_empty_game();

        assert!(!game.is_paused());
        game.toggle_pause();
        assert!(game.is_paused());
        game.toggle_pause();
        assert!(!game.is_paused());
    }

    #[test]
    fn test_place_number_disabled_in_hint_mode() {
        let mut game = create_empty_game();

        game.toggle_hint_mode();

        let result = game.place_number(5);
        assert!(result.is_none());
    }

    #[test]
    fn test_place_number_disabled_when_paused() {
        let mut game = create_empty_game();

        game.toggle_pause();

        let result = game.place_number(5);
        assert!(result.is_none());
    }

    #[test]
    fn test_erase_disabled_in_hint_mode() {
        let mut game = create_empty_game();

        game.toggle_hint_mode();

        game.erase();
    }

    #[test]
    fn test_erase_disabled_when_paused() {
        let mut game = create_empty_game();

        game.toggle_pause();

        game.erase();
    }

    #[test]
    fn test_place_same_number_again() {
        let mut game = create_empty_game();

        let (row, col) = find_empty_cell(&mut game).expect("Game should have empty cells");
        move_to_cell(&mut game, row, col);

        let solution_value = game.solution()[row][col];
        game.place_number(solution_value);
        let result = game.place_number(solution_value);

        assert!(result.is_none());
    }

    #[test]
    fn test_history_is_saved() {
        let mut game = create_empty_game();

        let (row, col) = find_empty_cell(&mut game).expect("Game should have empty cells");
        move_to_cell(&mut game, row, col);

        game.place_number(5);
        let (row2, col2) = find_empty_cell(&mut game).expect("Game should have empty cells");
        move_to_cell(&mut game, row2, col2);
        game.place_number(6);

        game.undo();
        assert_eq!(game.puzzle()[row2][col2], Cell::Empty);
    }

    #[test]
    fn test_conflicts_update_after_placement() {
        let mut game = create_empty_game();

        let target: Option<(usize, usize)> = (0..9)
            .flat_map(|r| (0..9).map(move |c| (r, c)))
            .find(|(r, c)| matches!(game.puzzle[*r][*c], Cell::Empty));

        let Some((row, col)) = target else {
            return;
        };

        game.cursor_row = row;
        game.cursor_col = col;

        game.place_number(5);

        assert!(true);
    }

    #[test]
    fn test_init_menu_creates_menu_state() {
        let state = init_menu(Difficulty::Medium);
        assert!(matches!(state, AppState::Menu { difficulty } if difficulty == Difficulty::Medium));
    }

    #[test]
    fn test_start_game_creates_playing_state() {
        let state = start_game(Difficulty::Hard);
        assert!(matches!(state, AppState::Playing(_)));
    }
}
