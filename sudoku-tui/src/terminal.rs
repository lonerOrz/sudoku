// terminal.rs: TUI 初始化与清理

use ratatui::{prelude::CrosstermBackend, terminal::Terminal};

pub type Tui = Terminal<CrosstermBackend<std::io::Stdout>>;

pub fn init() -> std::io::Result<Tui> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    Terminal::new(backend)
}

pub fn cleanup() -> std::io::Result<()> {
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()
}
