use std::io::{stdout, Write};

use crossterm::{
    queue,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct RawModeGuard {
    use_full_screen: bool,
}

impl RawModeGuard {
    pub fn new(use_full_screen: bool) -> Self {
        crossterm::terminal::enable_raw_mode().expect("Failed to enter raw mode");
        if use_full_screen {
            queue!(stdout(), crossterm::cursor::Hide, EnterAlternateScreen).unwrap();
        } else {
            queue!(stdout(), crossterm::cursor::Hide).unwrap();
        }
        stdout().flush().unwrap();
        Self { use_full_screen }
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().expect("Failed to exit raw mode");
        if self.use_full_screen {
            queue!(stdout(), crossterm::cursor::Show, LeaveAlternateScreen).unwrap();
        } else {
            queue!(stdout(), crossterm::cursor::Show, crossterm::cursor::MoveToColumn(0)).unwrap();
        }
        stdout().flush().unwrap();
    }
}
