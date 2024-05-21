use self::buffer::{Buffer, Rect, Size};

use super::*;

#[derive(Clone, Debug)]
pub struct Border<V> {
    pub(crate) child: V,
    pub(crate) border_color: Color,
}

impl<V> private::Sealed for Border<V> {}

impl<V> Border<V> {
    pub fn new(child: V, border_color: Color) -> Self {
        Self { child, border_color }
    }

    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border_color = border_color;
        self
    }

    fn draw_corner(buffer: &mut Buffer, x: u16, y: u16, symbol: char, color: Color) {
        buffer.get_mut(x, y).set_symbol(&symbol.to_string()).set_fg(color);
    }

    fn draw_borders(&self, buffer: &mut Buffer, rect: Rect) {
        let left = rect.left();
        let top = rect.top();
        let right = rect.right() - 1;
        let bottom_y = rect.bottom() - 1;

        // Draw corners
        Self::draw_corner(buffer, left, top, TOP_LEFT, self.border_color);
        Self::draw_corner(buffer, right, top, TOP_RIGHT, self.border_color);
        Self::draw_corner(buffer, left, bottom_y, BOTTOM_LEFT, self.border_color);
        Self::draw_corner(buffer, right, bottom_y, BOTTOM_RIGHT, self.border_color);

        // Draw horizontal lines
        draw_horizontal_line(buffer, top, left + 1, right, HORIZONTAL, self.border_color);
        draw_horizontal_line(buffer, bottom_y, left + 1, right, HORIZONTAL, self.border_color);

        // Draw vertical lines
        draw_vertical_line(buffer, left, top + 1, bottom_y, VERTICAL, self.border_color);
        draw_vertical_line(buffer, right, top + 1, bottom_y, VERTICAL, self.border_color);
    }
}

const VERTICAL: char = '│';
const HORIZONTAL: char = '─';
const TOP_LEFT: char = '┌';
const TOP_RIGHT: char = '┐';
const BOTTOM_LEFT: char = '└';
const BOTTOM_RIGHT: char = '┘';

fn draw_horizontal_line(buffer: &mut Buffer, y: u16, start_x: u16, end_x: u16, char: char, color: Color) {
    for x in start_x..end_x {
        buffer.set_char_at(x, y, char, color, Color::Reset, Modifier::empty());
    }
}

fn draw_vertical_line(buffer: &mut Buffer, x: u16, start_y: u16, end_y: u16, char: char, color: Color) {
    for y in start_y..end_y {
        buffer.set_char_at(x, y, char, color, Color::Reset, Modifier::empty());
    }
}

impl<V: View> View for Border<V> {
    fn size(&self, proposed: Size) -> Size {
        let inset = proposed.inset_by(2, 2, 1, 1);
        let child_size = self.child.size(inset);
        child_size.outset_by(2, 2, 1, 1).min(proposed)
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let size = self.size(context.rect.size);
        let border_rect = Rect {
            point: context.rect.point,
            size,
        };

        let inner_rect = border_rect.inset_by(2, 2, 1, 1);

        // Render the child view within the inner rectangle
        self.child.render(RenderContext::new(inner_rect), buffer);

        self.draw_borders(buffer, border_rect);
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
        let expected_size = Size { width: 13, height: 3 };
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
        assert_eq!(outer_view.size(Size::max()), Size { width: 14, height: 5 });
        assert_rendered_view(outer_view, expected_output, 14, 5);
    }
}
