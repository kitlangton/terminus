use compact_str::CompactString;

use crate::Color;

use super::Modifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub(crate) symbol: CompactString,
    pub(crate) fg: Color,
    pub(crate) bg: Color,
    pub(crate) modifier: Modifier,
}

impl Cell {
    pub fn set_symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = symbol.into();
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

    pub fn set_modifier(&mut self, modifier: Modifier) -> &mut Self {
        self.modifier = modifier;
        self
    }

    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.into(),
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::empty(),
        }
    }

    pub(crate) fn reset(&mut self) {
        self.symbol = CompactString::new_inline(" ");
        self.fg = Color::Reset;
        self.bg = Color::Reset;
        self.modifier = Modifier::empty();
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            symbol: CompactString::new_inline(" "),
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::empty(),
        }
    }
}
