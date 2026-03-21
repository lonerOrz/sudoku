mod config;
mod state;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{prelude::CrosstermBackend, terminal::Terminal};

use state::AppState;

fn main() -> std::io::Result<()> {
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;
    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen)?;

    let mut state = AppState::Menu {
        difficulty: sudoku_core::Difficulty::Easy,
    };

    loop {
        terminal.draw(|f| ui::draw(&state, f))?;

        if let Event::Key(key) = event::read()? {
            match (&mut state, key.code) {
                (AppState::Menu { difficulty }, KeyCode::Left) => {
                    *difficulty = config::cycle(*difficulty, false);
                }
                (AppState::Menu { difficulty }, KeyCode::Right) => {
                    *difficulty = config::cycle(*difficulty, true);
                }
                (AppState::Menu { .. }, KeyCode::Enter | KeyCode::Char(' ')) => {}
                (_, KeyCode::Char('q') | KeyCode::Esc) => {
                    break;
                }
                _ => {}
            }
        }
    }

    execute!(std::io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
