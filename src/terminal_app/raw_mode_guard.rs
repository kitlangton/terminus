use std::io::{stdout, Write};

use crossterm::queue;

pub struct RawModeGuard;

impl RawModeGuard {
    pub fn new() -> Self {
        crossterm::terminal::enable_raw_mode().expect("Failed to enter raw mode");
        queue!(stdout(), crossterm::cursor::Hide).unwrap();
        stdout().flush().unwrap();
        Self
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().expect("Failed to exit raw mode");
        queue!(
            stdout(),
            crossterm::cursor::Show,
            crossterm::cursor::MoveToColumn(0)
        )
        .unwrap();
        stdout().flush().unwrap();
    }
}
