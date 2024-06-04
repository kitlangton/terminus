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

        if removed.contains(Modifier::BOLD) || removed.contains(Modifier::DIM) {
            queue!(writer, SetAttribute(Attribute::NormalIntensity)).unwrap();
            if next.contains(Modifier::DIM) {
                queue!(writer, SetAttribute(Attribute::Dim)).unwrap();
            } else if next.contains(Modifier::BOLD) {
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

pub use crossterm::style::Color;
