use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

pub fn init() -> Terminal<CrosstermBackend<io::Stdout>> {
    enable_raw_mode();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen);

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).unwrap();
    terminal
}

pub fn cleanup(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) {
    disable_raw_mode();
    execute!(terminal.backend_mut(), LeaveAlternateScreen);
    terminal.show_cursor();
}
