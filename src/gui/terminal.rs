use super::super::models::error::Result;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub fn start() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode().expect("can run in raw mode");

    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    Ok(terminal)
}

pub fn end(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    terminal.clear()?;
    terminal.show_cursor()?;
    Ok(())
}
