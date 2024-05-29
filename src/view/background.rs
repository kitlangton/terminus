use crate::*;

#[derive(Clone, Debug)]
pub struct Background<V, BG> {
    pub(crate) background: BG,
    pub(crate) view: V,
}

impl<V, BG> private::Sealed for Background<V, BG> {}

impl<V: View, BG: View> View for Background<V, BG> {
    fn size(&self, proposed: Size) -> Size {
        self.view.size(proposed)
    }

    fn render(&self, id: &mut ViewId, context: Context, state: &mut AppState, buffer: &mut Buffer) {
        let size = self.view.size(context.rect.size);

        id.push(0);
        self.background
            .render(id, context.clone().with_size(size), state, buffer);
        id.pop();

        id.push(1);
        self.view.render(id, context, state, buffer);
        id.pop();
    }
}

pub struct FillColor {
    pub(crate) color: Color,
}

impl private::Sealed for FillColor {}

impl View for FillColor {
    fn size(&self, proposed: Size) -> Size {
        proposed
    }

    fn render(
        &self,
        _id: &mut ViewId,
        context: Context,
        _state: &mut AppState,
        buffer: &mut Buffer,
    ) {
        let point = context.rect.point;
        let size = context.rect.size;
        for y in point.y..point.y + size.height {
            for x in point.x..point.x + size.width {
                buffer.set_char_at(x, y, ' ', Color::Reset, Some(self.color), Modifier::empty());
            }
        }
    }
}
