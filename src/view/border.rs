use self::buffer::{Buffer, Rect, Size};

use super::*;

#[derive(Clone, Debug)]
pub struct Border<V: View> {
    pub(crate) child: V,
    pub(crate) border_color: Color,
}

impl<V: View> Border<V> {
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

    fn draw_borders(&self, buffer: &mut Buffer, size: Size, left: u16, top: u16, bottom_y: u16) {
        // Draw corners
        Self::draw_corner(buffer, left, top, TOP_LEFT, self.border_color);
        Self::draw_corner(buffer, size.width - 1, top, TOP_RIGHT, self.border_color);
        Self::draw_corner(buffer, left, bottom_y, BOTTOM_LEFT, self.border_color);
        Self::draw_corner(buffer, size.width - 1, bottom_y, BOTTOM_RIGHT, self.border_color);

        // Draw horizontal lines
        draw_horizontal_line(buffer, top, 1, size.width - 1, HORIZONTAL, self.border_color);
        draw_horizontal_line(buffer, bottom_y, 1, size.width - 1, HORIZONTAL, self.border_color);

        // Draw vertical lines
        draw_vertical_line(buffer, left, top + 1, bottom_y, VERTICAL, self.border_color);
        draw_vertical_line(buffer, size.width - 1, top + 1, bottom_y, VERTICAL, self.border_color);
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
        child_size.outset_by(2, 2, 1, 1)
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let inner_rect = context.rect.inset_by(2, 2, 1, 1);
        self.child.render(RenderContext::new(inner_rect), buffer);
        let size = self.size(context.rect.size);

        let top = context.rect.top();
        let left = context.rect.left();
        let bottom_y = top + size.height - 1;

        self.draw_borders(buffer, size, left, top, bottom_y);
    }
}
