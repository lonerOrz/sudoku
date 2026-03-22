// terminal.rs: TUI 初始化与清理

use ratatui::{prelude::CrosstermBackend, terminal::Terminal};

pub type Tui = Terminal<CrosstermBackend<std::io::Stdout>>;

pub struct TerminalGuard;

pub fn init() -> std::io::Result<(Tui, TerminalGuard)> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;
    Ok((terminal, TerminalGuard))
}

pub fn cleanup() -> std::io::Result<()> {
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = cleanup();
    }
}
