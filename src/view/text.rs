use self::view::View;

use super::*;

#[derive(Clone, Debug)]
pub struct Text {
    pub(crate) text: String,
}

impl View for Text {
    fn size(&self, proposed: Size) -> Size {
        Size {
            width: (self.text.len() as u16).min(proposed.width),
            height: 1,
        }
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let rect = context.rect;
        let mut x = rect.left();
        let y = rect.top();
        let max_x = rect.right();
        for c in self.text.chars() {
            if x >= max_x {
                break;
            }
            buffer.set_char_at(x, y, c, context.fg, context.bg, context.modifier);
            x += 1;
        }
    }
}
