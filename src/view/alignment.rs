use super::*;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Alignment {
    pub horizontal: HorizontalAlignment,
    pub vertical: VerticalAlignment,
}

impl std::fmt::Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Alignment::TOP_LEFT => write!(f, "Top Left"),
            Alignment::TOP_RIGHT => write!(f, "Top Right"),
            Alignment::TOP => write!(f, "Top"),
            Alignment::LEFT => write!(f, "Left"),
            Alignment::CENTER => write!(f, "Center"),
            Alignment::RIGHT => write!(f, "Right"),
            Alignment::BOTTOM_LEFT => write!(f, "Bottom Left"),
            Alignment::BOTTOM_RIGHT => write!(f, "Bottom Right"),
            Alignment::BOTTOM => write!(f, "Bottom"),
        }
    }
}

impl Alignment {
    pub const TOP_LEFT: Self = Self {
        horizontal: HorizontalAlignment::LEFT,
        vertical: VerticalAlignment::TOP,
    };

    pub const TOP: Self = Self {
        horizontal: HorizontalAlignment::CENTER,
        vertical: VerticalAlignment::TOP,
    };

    pub const TOP_RIGHT: Self = Self {
        horizontal: HorizontalAlignment::RIGHT,
        vertical: VerticalAlignment::TOP,
    };

    pub const LEFT: Self = Self {
        horizontal: HorizontalAlignment::LEFT,
        vertical: VerticalAlignment::CENTER,
    };

    pub const CENTER: Self = Self {
        horizontal: HorizontalAlignment::CENTER,
        vertical: VerticalAlignment::CENTER,
    };

    pub const RIGHT: Self = Self {
        horizontal: HorizontalAlignment::RIGHT,
        vertical: VerticalAlignment::CENTER,
    };

    pub const BOTTOM_LEFT: Self = Self {
        horizontal: HorizontalAlignment::LEFT,
        vertical: VerticalAlignment::BOTTOM,
    };

    pub const BOTTOM: Self = Self {
        horizontal: HorizontalAlignment::CENTER,
        vertical: VerticalAlignment::BOTTOM,
    };

    pub const BOTTOM_RIGHT: Self = Self {
        horizontal: HorizontalAlignment::RIGHT,
        vertical: VerticalAlignment::BOTTOM,
    };
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

impl VerticalAlignment {
    pub const CENTER: Self = Self::Center;
    pub const TOP: Self = Self::Top;
    pub const BOTTOM: Self = Self::Bottom;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

impl HorizontalAlignment {
    pub const CENTER: Self = Self::Center;
    pub const LEFT: Self = Self::Left;
    pub const RIGHT: Self = Self::Right;
}
