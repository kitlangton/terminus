use crate::*;

#[derive(Clone, Debug)]
pub struct GeometryReader<F> {
    pub(crate) view: F,
}

impl<F> GeometryReader<F> {
    pub fn new(view: F) -> Self {
        Self { view }
    }
}

impl<F> private::Sealed for GeometryReader<F> {}

// f is a function from Size -> impl View
impl<F, V: View> View for GeometryReader<F>
where
    F: Fn(Size) -> V + 'static,
{
    fn size(&self, proposed: Size) -> Size {
        let f = &self.view;
        f(proposed).size(proposed)
    }

    fn render(&self, id: &mut ViewId, context: Context, state: &mut AppState, buffer: &mut Buffer) {
        let f = &self.view;
        let view = f(context.rect.size);
        view.render(id, context, state, buffer);
    }
}
