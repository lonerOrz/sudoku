// state.rs: 应用状态

use crate::game::Game;
use sudoku_core::Difficulty;

pub type PencilMarks = [[Vec<u8>; 9]; 9];

#[derive(Debug, Clone, Copy)]
pub struct Control {
    pub key: &'static str,
    pub label: &'static str,
}

#[derive(Clone)]
pub struct HistoryEntry {
    pub puzzle: sudoku_core::Grid,
    pub pencil_marks: PencilMarks,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub mistakes: u8,
}

impl AppState {
    pub fn controls(&self) -> &'static [Control] {
        match self {
            AppState::Menu { .. } => MENU_CONTROLS,
            AppState::Playing(game) if game.is_paused() => PAUSED_CONTROLS,
            AppState::Playing(game) if game.is_hint_mode() => HINT_CONTROLS,
            AppState::Playing(game) if game.is_pencil_mode() => PENCIL_CONTROLS,
            AppState::Playing(_) => NORMAL_CONTROLS,
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
        key: "n",
        label: "Numbers",
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
    Control {
        key: "n",
        label: "Numbers",
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
    Control {
        key: "n",
        label: "Numbers",
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
    Playing(Game),
    Won {
        difficulty: Difficulty,
        elapsed_secs: u64,
    },
    Failed {
        difficulty: Difficulty,
        elapsed_secs: u64,
    },
}
