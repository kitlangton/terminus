use crate::Color;

use super::Modifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub(crate) symbol: String,
    pub(crate) fg: Color,
    pub(crate) bg: Color,
    pub(crate) modifier: Modifier,
}

impl Cell {
    pub fn set_symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = symbol.to_string();
        self
    }

    pub fn set_fg(&mut self, fg: Color) -> &mut Self {
        self.fg = fg;
        self
    }

    pub fn set_bg(&mut self, bg: Color) -> &mut Self {
        self.bg = bg;
        self
    }

    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::empty(),
        }
    }

    pub fn to_ansi_code(&self) -> String {
        format!(
            "{}{}{}",
            self.fg.to_foreground_code(),
            self.bg.to_background_code(),
            self.symbol,
        )
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            symbol: " ".to_string(),
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::empty(),
        }
    }
}
