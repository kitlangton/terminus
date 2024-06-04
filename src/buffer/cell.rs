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

    pub(crate) fn to_string(&self) -> String {
        use crossterm::style::{
            Attribute, Color as CrosstermColor, Print, SetAttribute, SetBackgroundColor,
            SetForegroundColor,
        };
        use std::fmt::Write;

        let mut result = String::new();

        write!(result, "{}", SetForegroundColor(self.fg)).unwrap();
        write!(result, "{}", SetBackgroundColor(self.bg)).unwrap();

        // Set modifiers
        if self.modifier.contains(Modifier::BOLD) {
            write!(result, "{}", SetAttribute(Attribute::Bold)).unwrap();
        }
        if self.modifier.contains(Modifier::ITALIC) {
            write!(result, "{}", SetAttribute(Attribute::Italic)).unwrap();
        }
        if self.modifier.contains(Modifier::UNDERLINE) {
            write!(result, "{}", SetAttribute(Attribute::Underlined)).unwrap();
        }
        if self.modifier.contains(Modifier::DIM) {
            write!(result, "{}", SetAttribute(Attribute::Dim)).unwrap();
        }
        if self.modifier.contains(Modifier::INVERSE) {
            write!(result, "{}", SetAttribute(Attribute::Reverse)).unwrap();
        }
        if self.modifier.contains(Modifier::STRIKETHROUGH) {
            write!(result, "{}", SetAttribute(Attribute::CrossedOut)).unwrap();
        }

        // Print the symbol
        write!(result, "{}", Print(&self.symbol)).unwrap();

        // Reset all attributes
        write!(result, "{}", SetForegroundColor(CrosstermColor::Reset)).unwrap();
        write!(result, "{}", SetBackgroundColor(CrosstermColor::Reset)).unwrap();
        write!(result, "{}", SetAttribute(Attribute::Reset)).unwrap();

        result
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
