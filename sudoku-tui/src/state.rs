// state.rs: 应用状态

use sudoku_core::{Conflicts, Difficulty, Grid, Solution};

pub type PencilMarks = [[Vec<u8>; 9]; 9];

#[derive(Debug, Clone, Copy)]
pub struct Control {
    pub key: &'static str,
    pub label: &'static str,
}

#[derive(Clone)]
pub struct HistoryEntry {
    pub puzzle: Grid,
    pub pencil_marks: PencilMarks,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub mistakes: u8,
}

impl AppState {
    pub fn controls(&self) -> &'static [Control] {
        match self {
            AppState::Menu { .. } => MENU_CONTROLS,
            AppState::Playing { paused: true, .. } => PAUSED_CONTROLS,
            AppState::Playing {
                hint_mode: true, ..
            } => HINT_CONTROLS,
            AppState::Playing {
                pencil_mode: true, ..
            } => PENCIL_CONTROLS,
            AppState::Playing { .. } => NORMAL_CONTROLS,
            AppState::Won { .. } | AppState::Failed { .. } => END_CONTROLS,
        }
    }
}

const NORMAL_CONTROLS: &[Control] = &[
    Control {
        key: "←↑↓→",
        label: "Move",
    },
    Control {
        key: "1-9",
        label: "Place",
    },
    Control {
        key: "0/Del",
        label: "Erase",
    },
    Control {
        key: "u",
        label: "Undo",
    },
    Control {
        key: "Space",
        label: "Pause",
    },
    Control {
        key: "p",
        label: "Pencil",
    },
    Control {
        key: "h",
        label: "Hint",
    },
    Control {
        key: "q",
        label: "Quit",
    },
];

const PENCIL_CONTROLS: &[Control] = &[
    Control {
        key: "←↑↓→",
        label: "Move",
    },
    Control {
        key: "1-9",
        label: "Mark",
    },
    Control {
        key: "0/Del",
        label: "Clear",
    },
    Control {
        key: "p",
        label: "Exit",
    },
];

const HINT_CONTROLS: &[Control] = &[
    Control {
        key: "←↑↓→",
        label: "Move",
    },
    Control {
        key: "?",
        label: "Fill",
    },
    Control {
        key: "h",
        label: "Exit",
    },
];

const PAUSED_CONTROLS: &[Control] = &[
    Control {
        key: "Space",
        label: "Resume",
    },
    Control {
        key: "q",
        label: "Quit",
    },
];

const MENU_CONTROLS: &[Control] = &[
    Control {
        key: "↑↓",
        label: "Select",
    },
    Control {
        key: "Enter",
        label: "Start",
    },
    Control {
        key: "q",
        label: "Quit",
    },
];

const END_CONTROLS: &[Control] = &[Control {
    key: "q",
    label: "Menu",
}];

#[allow(clippy::large_enum_variant)]
pub enum AppState {
    Menu {
        difficulty: Difficulty,
    },
    Playing {
        puzzle: Grid,
        solution: Solution,
        pencil_marks: PencilMarks,
        pencil_mode: bool,
        hint_mode: bool,
        cursor_row: usize,
        cursor_col: usize,
        conflicts: Conflicts,
        difficulty: Difficulty,
        mistakes: u8,
        start_time: std::time::Instant,
        elapsed_secs: u64,
        paused: bool,
        history: Vec<HistoryEntry>,
    },
    Won {
        difficulty: Difficulty,
        elapsed_secs: u64,
    },
    Failed {
        difficulty: Difficulty,
        elapsed_secs: u64,
    },
}
