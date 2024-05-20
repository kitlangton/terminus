use self::buffer::{Buffer, Rect, Size};

use super::*;

#[derive(Clone, Debug)]
pub struct Padding<V: View> {
    pub(crate) child: V,
    pub(crate) padding_top: u16,
    pub(crate) padding_bottom: u16,
    pub(crate) padding_left: u16,
    pub(crate) padding_right: u16,
}

impl<V: View> View for Padding<V> {
    fn size(&self, proposed: Size) -> Size {
        let inset = proposed.inset_by(
            self.padding_left,
            self.padding_right,
            self.padding_top,
            self.padding_bottom,
        );
        let child_size = self.child.size(inset);
        Size {
            width: child_size.width + self.padding_left + self.padding_right,
            height: child_size.height + self.padding_top + self.padding_bottom,
        }
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let inner_rect = context.rect.inset_by(
            self.padding_left,
            self.padding_right,
            self.padding_top,
            self.padding_bottom,
        );
        self.child.render(RenderContext::new(inner_rect), buffer);
    }
}
