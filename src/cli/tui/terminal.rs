use std::io::{self, stdout};
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::{
    ExecutableCommand,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{CompletedFrame, Frame, Terminal};

pub struct TerminalManager {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TerminalManager {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;

        Ok(TerminalManager { terminal })
    }

    pub fn draw<F>(&mut self, f: F) -> io::Result<CompletedFrame>
    where
        F: FnOnce(&mut Frame),
    {
        self.terminal.draw(f)
    }
}

impl Drop for TerminalManager {
    fn drop(&mut self) {
        // Cleanup is guaranteed to be called even if the program panics.
        let _ = disable_raw_mode();
        let _ = stdout().execute(LeaveAlternateScreen);
    }
}