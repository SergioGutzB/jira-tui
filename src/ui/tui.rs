use crate::domain::errors::Result;
use crossterm::{execute, terminal::*};
use ratatui::prelude::*;
use std::io::{self, Stdout};

/// A type alias for the terminal backend used in this application.
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initializes the terminal interface.
///
/// This enables raw mode, switches to the alternate screen, and sets up the panic hook
/// to restore the terminal if the application crashes.
pub fn init() -> Result<Tui> {
    execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode().map_err(|e| crate::domain::errors::AppError::Unknown(e.to_string()))?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)
        .map_err(|e| crate::domain::errors::AppError::Unknown(e.to_string()))?;
    terminal
        .clear()
        .map_err(|e| crate::domain::errors::AppError::Unknown(e.to_string()))?;

    Ok(terminal)
}

/// Restores the terminal to its original state.
///
/// This is critical to run before exiting, otherwise the user's terminal will remain
/// in a broken state (no cursor, raw input).
pub fn restore() -> Result<()> {
    execute!(io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode().map_err(|e| crate::domain::errors::AppError::Unknown(e.to_string()))?;
    Ok(())
}
