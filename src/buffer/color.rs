use std::io::Write;

use bitflags::bitflags;
use crossterm::queue;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Modifier: u16 {
        const RESET = 0b0000_0001;
        const BOLD = 0b0000_0010;
        const DIM = 0b0000_0100;
        const ITALIC = 0b0000_1000;
        const UNDERLINE = 0b0001_0000;
        const BLINK = 0b0010_0000;
        const INVERSE = 0b0100_0000;
        const HIDDEN = 0b1000_0000;
        const STRIKETHROUGH = 0b0001_0000_0000;
    }
}

impl Modifier {
    pub fn write_diff<W: Write>(next: Modifier, previous: Modifier, writer: &mut W) {
        use crossterm::style::*;

        let removed = previous.difference(next);
        let added = next.difference(previous);

        if removed.contains(Modifier::BOLD) {
            queue!(writer, SetAttribute(Attribute::NormalIntensity)).unwrap();

            if next.contains(Modifier::DIM) {
                queue!(writer, SetAttribute(Attribute::Dim)).unwrap();
            }
        }

        if removed.contains(Modifier::DIM) {
            queue!(writer, SetAttribute(Attribute::NormalIntensity)).unwrap();
            if next.contains(Modifier::BOLD) {
                queue!(writer, SetAttribute(Attribute::Bold)).unwrap();
            }
        }

        if removed.contains(Modifier::ITALIC) {
            queue!(writer, SetAttribute(Attribute::NoItalic)).unwrap();
        }

        if removed.contains(Modifier::UNDERLINE) {
            queue!(writer, SetAttribute(Attribute::NoUnderline)).unwrap();
        }

        if removed.contains(Modifier::BLINK) {
            queue!(writer, SetAttribute(Attribute::NoBlink)).unwrap();
        }

        if removed.contains(Modifier::INVERSE) {
            queue!(writer, SetAttribute(Attribute::NoReverse)).unwrap();
        }

        if removed.contains(Modifier::HIDDEN) {
            queue!(writer, SetAttribute(Attribute::NoHidden)).unwrap();
        }

        if removed.contains(Modifier::STRIKETHROUGH) {
            queue!(writer, SetAttribute(Attribute::NotCrossedOut)).unwrap();
        }

        if added.contains(Modifier::BOLD) {
            queue!(writer, SetAttribute(Attribute::Bold)).unwrap();
        }

        if added.contains(Modifier::DIM) {
            queue!(writer, SetAttribute(Attribute::Dim)).unwrap();
        }

        if added.contains(Modifier::ITALIC) {
            queue!(writer, SetAttribute(Attribute::Italic)).unwrap();
        }

        if added.contains(Modifier::UNDERLINE) {
            queue!(writer, SetAttribute(Attribute::Underlined)).unwrap();
        }

        if added.contains(Modifier::BLINK) {
            queue!(writer, SetAttribute(Attribute::SlowBlink)).unwrap();
        }

        if added.contains(Modifier::INVERSE) {
            queue!(writer, SetAttribute(Attribute::Reverse)).unwrap();
        }

        if added.contains(Modifier::HIDDEN) {
            queue!(writer, SetAttribute(Attribute::Hidden)).unwrap();
        }

        if added.contains(Modifier::STRIKETHROUGH) {
            queue!(writer, SetAttribute(Attribute::CrossedOut)).unwrap();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Black => write!(f, "Black"),
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Yellow => write!(f, "Yellow"),
            Color::Blue => write!(f, "Blue"),
            Color::Magenta => write!(f, "Magenta"),
            Color::Cyan => write!(f, "Cyan"),
            Color::White => write!(f, "White"),
            Color::Reset => write!(f, "Reset"),
        }
    }
}

impl Color {
    pub fn to_ansi_code(&self) -> u8 {
        match *self {
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
            Color::Reset => 0,
        }
    }

    pub fn to_foreground_code(&self) -> String {
        if self == &Color::Reset {
            "".to_string()
        } else {
            format!("\x1b[38;5;{}m", self.to_ansi_code())
        }
    }

    pub fn to_background_code(&self) -> String {
        if self == &Color::Reset {
            "".to_string()
        } else {
            format!("\x1b[48;5;{}m", self.to_ansi_code())
        }
    }
}

impl From<Color> for crossterm::style::Color {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => crossterm::style::Color::Black,
            Color::Red => crossterm::style::Color::Red,
            Color::Green => crossterm::style::Color::Green,
            Color::Yellow => crossterm::style::Color::Yellow,
            Color::Blue => crossterm::style::Color::Blue,
            Color::Magenta => crossterm::style::Color::Magenta,
            Color::Cyan => crossterm::style::Color::Cyan,
            Color::White => crossterm::style::Color::White,
            Color::Reset => crossterm::style::Color::Reset,
        }
    }
}
