use crate::{private, AppState, Size, View, ViewId};

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

pub struct Frame<V> {
    pub(crate) child: V,
    pub(crate) min_width: Option<u16>,
    pub(crate) min_height: Option<u16>,
    pub(crate) max_width: Option<u16>,
    pub(crate) max_height: Option<u16>,
    pub(crate) alignment: Alignment,
}

impl<V: View> private::Sealed for Frame<V> {}

impl<V: View> View for Frame<V> {
    fn size(&self, proposed: crate::Size) -> crate::Size {
        /// if max == u16::max, it should use the proposed size.
        /// otherwise, it should be the child size bounded by the min and max
        /// max is not optional.
        fn calculate_dimension(
            proposed: u16,
            child: u16,
            min: Option<u16>,
            max: Option<u16>,
        ) -> u16 {
            let max = max.unwrap_or(proposed);
            if max == u16::MAX {
                proposed
            } else {
                child.clamp(min.unwrap_or(0), max)
            }
        }

        let child_size = self.child.size(proposed);
        let width = calculate_dimension(
            proposed.width,
            child_size.width,
            self.min_width,
            self.max_width,
        );
        let height = calculate_dimension(
            proposed.height,
            child_size.height,
            self.min_height,
            self.max_height,
        );

        Size::new(width, height)
    }

    fn render(
        &self,
        id: &mut ViewId,
        context: crate::Context,
        state: &mut AppState,
        buffer: &mut crate::Buffer,
    ) {
        let child_size = self.child.size(context.rect.size);
        let size = self.size(context.rect.size);

        let offset_x = if self.max_width == Some(u16::MAX) {
            match self.alignment.horizontal {
                HorizontalAlignment::Left => 0,
                HorizontalAlignment::Center => {
                    (size.width / 2).saturating_sub(child_size.width / 2)
                }
                HorizontalAlignment::Right => size.width.saturating_sub(child_size.width),
            }
        } else {
            0
        };

        let offset_y = if self.max_height == Some(u16::MAX) {
            match self.alignment.vertical {
                VerticalAlignment::Top => 0,
                VerticalAlignment::Center => {
                    (size.height / 2).saturating_sub(child_size.height / 2)
                }
                VerticalAlignment::Bottom => size.height.saturating_sub(child_size.height),
            }
        } else {
            0
        };

        self.child.render(
            id,
            context.with_size(size).offset(offset_x, offset_y),
            state,
            buffer,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tests::assert_rendered_view, *};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_frame_size_with_min_constraints() {
        let frame = text("Hello").frame(Some(10), Some(5), None, None, Alignment::TOP_LEFT);
        let proposed_size = Size::new(100, 100);
        let size = frame.size(proposed_size);
        assert_eq!(size, Size::new(10, 5));
    }

    #[test]
    fn test_frame_size_with_max_constraints() {
        let frame = text("Hello, world!").frame(None, None, Some(5), Some(1), Alignment::TOP_LEFT);
        let proposed_size = Size::new(100, 100);
        let size = frame.size(proposed_size);

        assert_eq!(size.width, 5);
        assert_eq!(size.height, 1);
    }

    #[test]
    fn test_frame_size_within_constraints() {
        let frame = text("Hello").frame(Some(3), Some(1), Some(10), Some(2), Alignment::TOP_LEFT);
        let proposed_size = Size::max();
        let size = frame.size(proposed_size);
        assert_eq!(size, Size::new(5, 1));
    }

    #[test]
    fn test_frame_size_with_no_constraints() {
        let frame = text("Hello").frame(None, None, None, None, Alignment::TOP_LEFT);
        let proposed_size = Size::new(20, 10);
        let size = frame.size(proposed_size);
        assert_eq!(size, Size::new(5, 1)); // Assuming "Hello" takes 5x1 space
    }

    #[test]
    fn test_frame_max_width_takes_up_entire_proposed_width() {
        let frame = text("Hello").frame(None, None, Some(u16::MAX), None, Alignment::TOP_LEFT);
        let proposed_size = Size::new(20, 2);
        let size = frame.size(proposed_size);
        assert_eq!(size, Size::new(20, 1));
    }

    #[test]
    fn test_frame_max_height_takes_up_entire_proposed_height() {
        let frame = text("Hello").frame(None, None, None, Some(u16::MAX), Alignment::TOP_LEFT);
        let proposed_size = Size::new(20, 8);
        let size = frame.size(proposed_size);
        assert_eq!(size, Size::new(5, 8));
    }

    #[test]
    fn test_render_frame_with_text() {
        let frame = text("HELLO WORLD").frame(None, None, Some(5), None, Alignment::TOP_LEFT);
        let expected_output = vec!["HELLO      "];
        assert_rendered_view(frame, expected_output, 11, 1);
    }

    #[test]
    fn test_frame_max_width_with_alignment() {
        let frame = text("HELLO!").frame(None, None, Some(u16::MAX), None, Alignment::CENTER);
        let proposed_size = Size::new(20, 1);
        let size = frame.size(proposed_size);
        assert_eq!(size, Size::new(20, 1));

        let expected_output = vec!["       HELLO!       "];
        assert_rendered_view(frame, expected_output, 20, 1);
    }

    #[test]
    fn test_frame_max_width_with_right_alignment() {
        let frame = text("WOW")
            .frame(None, None, Some(u16::MAX), None, Alignment::RIGHT)
            .border();
        let proposed_size = Size::new(10, 3);
        let size = frame.size(proposed_size);
        assert_eq!(size, Size::new(10, 3));

        let expected_output = vec![
            "┌────────┐", // 1
            "│    WOW │", // 2
            "└────────┘", // 3
        ];
        assert_rendered_view(frame, expected_output, 10, 3);
    }

    #[test]
    fn test_frame_vertical_and_horizontal_alignment() {
        // Test for vertical and horizontal center alignment
        let frame = text("WOW")
            .frame(
                None,
                None,
                Some(u16::MAX),
                Some(u16::MAX),
                Alignment::CENTER,
            )
            .border();
        let proposed_size = Size::new(9, 5);
        let size = frame.size(proposed_size);
        assert_eq!(size, Size::new(9, 5));

        let expected_output = vec![
            "┌───────┐", // 1
            "│       │", // 2
            "│  WOW  │", // 3
            "│       │", // 4
            "└───────┘", // 5
        ];
        assert_rendered_view(frame, expected_output, 9, 5);
    }

    #[test]
    fn test_frame_vertical_and_horizontal_alignment_bottom_right() {
        // Test for vertical and horizontal center alignment
        let view = text("WOW")
            .frame(
                None,
                None,
                Some(u16::MAX),
                Some(u16::MAX),
                Alignment::BOTTOM_RIGHT,
            )
            .border();
        let proposed_size = Size::new(9, 5);
        let size = view.size(proposed_size);
        assert_eq!(size, Size::new(9, 5));

        let expected_output = vec![
            "┌───────┐", // 1
            "│       │", // 2
            "│       │", // 3
            "│   WOW │", // 4
            "└───────┘", // 5
        ];
        assert_rendered_view(view, expected_output, 9, 5);
    }
}
