use crate::{private, AppState, Buffer, Context, Size, View, ViewId};

#[derive(Clone, Debug)]
pub struct Padding<V: View> {
    pub(crate) child: V,
    pub(crate) padding_top: u16,
    pub(crate) padding_bottom: u16,
    pub(crate) padding_left: u16,
    pub(crate) padding_right: u16,
}

impl<V: View> private::Sealed for Padding<V> {}

impl<V: View> View for Padding<V> {
    fn size(&self, proposed: Size) -> Size {
        let inset = proposed.inset_by(
            self.padding_left,
            self.padding_right,
            self.padding_top,
            self.padding_bottom,
        );
        let child_size = self.child.size(inset);
        child_size.outset_by(
            self.padding_left,
            self.padding_right,
            self.padding_top,
            self.padding_bottom,
        )
    }

    fn render(&self, id: &mut ViewId, context: Context, state: &mut AppState, buffer: &mut Buffer) {
        let inner_rect = context.rect.inset_by(
            self.padding_left,
            self.padding_right,
            self.padding_top,
            self.padding_bottom,
        );
        self.child
            .render(id, Context::new(inner_rect), state, buffer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_rendered_view;
    use crate::*;

    #[test]
    fn test_padding() {
        let view = text("Padded View").padding(2);
        let expected_output = vec![
            "               ", //
            "               ", //
            "  Padded View  ", //
            "               ", //
            "               ", //
        ];
        assert_eq!(
            view.size(Size::max()),
            Size {
                width: 15,
                height: 5
            }
        );
        assert_rendered_view(view, expected_output, 15, 5);
    }
}
