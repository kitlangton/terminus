use self::buffer::{Buffer, Rect, Size};

use super::*;

#[derive(Clone, Debug)]
pub struct Border<V> {
    pub(crate) child: V,
    pub(crate) border_color: Color,
    pub(crate) border_style: BorderStyle,
    pub(crate) title: Option<String>,
}

impl<V> private::Sealed for Border<V> {}

impl<V> Border<V> {
    pub fn new(child: V, border_color: Color) -> Self {
        Self {
            child,
            border_color,
            border_style: BorderStyle::Single,
            title: None,
        }
    }

    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border_color = border_color;
        self
    }

    pub fn border_style(mut self, border_style: BorderStyle) -> Self {
        self.border_style = border_style;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    fn draw_corner(buffer: &mut Buffer, x: u16, y: u16, symbol: char, color: Color) {
        buffer
            .get_mut(x, y)
            .set_symbol(&symbol.to_string())
            .set_fg(color);
    }

    fn draw_borders(&self, buffer: &mut Buffer, rect: Rect) {
        let left = rect.left();
        let top = rect.top();
        let right = rect.right().saturating_sub(1);
        let bottom_y = rect.bottom().saturating_sub(1);

        let components = self.border_style.components();

        // Draw corners
        Self::draw_corner(buffer, left, top, components.top_left, self.border_color);
        Self::draw_corner(buffer, right, top, components.top_right, self.border_color);
        Self::draw_corner(
            buffer,
            left,
            bottom_y,
            components.bottom_left,
            self.border_color,
        );
        Self::draw_corner(
            buffer,
            right,
            bottom_y,
            components.bottom_right,
            self.border_color,
        );

        // Draw horizontal lines
        draw_horizontal_line(
            buffer,
            top,
            left + 1,
            right,
            components.horizontal,
            self.border_color,
        );
        draw_horizontal_line(
            buffer,
            bottom_y,
            left + 1,
            right,
            components.horizontal,
            self.border_color,
        );

        // Draw vertical lines
        draw_vertical_line(
            buffer,
            left,
            top + 1,
            bottom_y,
            components.vertical,
            self.border_color,
        );
        draw_vertical_line(
            buffer,
            right,
            top + 1,
            bottom_y,
            components.vertical,
            self.border_color,
        );

        if let Some(ref title) = self.title {
            title.render(
                &mut ViewId::empty(),
                Context {
                    rect: Rect {
                        point: Point {
                            x: left + 1,
                            y: top,
                        },
                        size: Size {
                            width: rect.size.width.saturating_sub(2),
                            height: 1,
                        },
                    },
                    fg: self.border_color,
                    modifier: Modifier::empty(),
                },
                &mut AppState::new(),
                buffer,
            );
        }
    }
}

#[derive(Debug, Clone)]
pub enum BorderStyle {
    Single,
    Double,
    Rounded,
}

impl BorderStyle {
    fn components(&self) -> &'static BorderComponents {
        match self {
            BorderStyle::Single => &SINGLE_BORDER_COMPONENTS,
            BorderStyle::Double => &DOUBLE_BORDER_COMPONENTS,
            BorderStyle::Rounded => &ROUNDED_BORDER_COMPONENTS,
        }
    }
}

struct BorderComponents {
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    horizontal: char,
    vertical: char,
}

const SINGLE_BORDER_COMPONENTS: BorderComponents = BorderComponents {
    top_left: '┌',
    top_right: '┐',
    bottom_left: '└',
    bottom_right: '┘',
    horizontal: '─',
    vertical: '│',
};

const DOUBLE_BORDER_COMPONENTS: BorderComponents = BorderComponents {
    top_left: '╔',
    top_right: '╗',
    bottom_left: '╚',
    bottom_right: '╝',
    horizontal: '═',
    vertical: '║',
};

const ROUNDED_BORDER_COMPONENTS: BorderComponents = BorderComponents {
    top_left: '╭',
    top_right: '╮',
    bottom_left: '╰',
    bottom_right: '╯',
    horizontal: '─',
    vertical: '│',
};

fn draw_horizontal_line(
    buffer: &mut Buffer,
    y: u16,
    start_x: u16,
    end_x: u16,
    char: char,
    color: Color,
) {
    for x in start_x..end_x {
        buffer.set_char_at(x, y, char, color, None, Modifier::empty());
    }
}

fn draw_vertical_line(
    buffer: &mut Buffer,
    x: u16,
    start_y: u16,
    end_y: u16,
    char: char,
    color: Color,
) {
    for y in start_y..end_y {
        buffer.set_char_at(x, y, char, color, None, Modifier::empty());
    }
}

impl<V: View> View for Border<V> {
    fn size(&self, proposed: Size) -> Size {
        let inset = proposed.inset_by(2, 2, 1, 1);
        let child_size = self.child.size(inset);
        child_size.outset_by(2, 2, 1, 1).min(proposed)
    }

    fn render(&self, id: &mut ViewId, context: Context, state: &mut AppState, buffer: &mut Buffer) {
        // Calculate the size of the border view based on the proposed size from the context
        let size = self.size(context.rect.size);

        // Define the rectangle for the border using the calculated size and the starting point from the context
        let border_context = context.with_size(size);

        // Calculate the inner rectangle by insetting the border rectangle
        // The insets are 2 units from the left and right, and 1 unit from the top and bottom
        let inner_context = border_context.clone().inset_by(2, 2, 1, 1);

        // Render the child view within the inner rectangle
        // This ensures the child view is drawn inside the borders
        self.child.render(id, inner_context, state, buffer);

        // Draw the borders around the border rectangle
        self.draw_borders(buffer, border_context.rect);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        tests::assert_rendered_view,
        view::{hstack, text, Size},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_border_size() {
        let view = hstack((text("Test"), text("View")));
        let border = view.border();
        let expected_size = Size {
            width: 13,
            height: 3,
        };
        assert_eq!(border.size(Size::max()), expected_size);
    }

    #[test]
    fn test_border_rendering() {
        let view = hstack((text("Test"), text("View"))).border();
        let expected_output = vec![
            "┌───────────┐", //
            "│ Test View │", //
            "└───────────┘", //
        ];
        assert_rendered_view(view, expected_output, 13, 3);
    }

    #[test]
    fn test_nested_bordered_view() {
        let inner_view = text("Nested").border();
        let outer_view = inner_view.border();
        let expected_output = vec![
            "┌────────────┐", //
            "│ ┌────────┐ │", //
            "│ │ Nested │ │", //
            "│ └────────┘ │", //
            "└────────────┘", //
        ];
        assert_eq!(
            outer_view.size(Size::max()),
            Size {
                width: 14,
                height: 5
            }
        );
        assert_rendered_view(outer_view, expected_output, 14, 5);
    }

    #[test]
    fn test_nested_bordered_view_double() {
        let inner_view = text("Nested").border().border_style(BorderStyle::Double);
        let outer_view = inner_view.border().border_style(BorderStyle::Double);
        let expected_output = vec![
            "╔════════════╗", //
            "║ ╔════════╗ ║", //
            "║ ║ Nested ║ ║", //
            "║ ╚════════╝ ║", //
            "╚════════════╝", //
        ];
        assert_eq!(
            outer_view.size(Size::max()),
            Size {
                width: 14,
                height: 5
            }
        );
        assert_rendered_view(outer_view, expected_output, 14, 5);
    }

    #[test]
    fn test_title() {
        let view = text("Titled View").border().title("Title");
        let expected_output = vec![
            "┌Title────────┐", //
            "│ Titled View │", //
            "└─────────────┘", //
        ];
        assert_eq!(
            view.size(Size::max()),
            Size {
                width: 15,
                height: 3
            }
        );
        assert_rendered_view(view, expected_output, 15, 3);
    }
}
