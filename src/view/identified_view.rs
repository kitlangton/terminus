use super::*;
#[derive(Clone, Debug)]
pub struct IdentifiedView<V> {
    pub id: u64,
    pub value: V,
}

impl<V> IdentifiedView<V> {
    pub fn new<ID: Hash>(id: ID, value: V) -> Self {
        Self {
            id: do_hash(&id),
            value,
        }
    }
}

impl<V: View> private::Sealed for IdentifiedView<V> {}

impl<V: View> View for IdentifiedView<V> {
    fn size(&self, proposed: Size) -> Size {
        self.value.size(proposed)
    }

    fn render(&self, id: &mut ViewId, context: Context, state: &mut AppState, buffer: &mut Buffer) {
        self.value.render(id, context, state, buffer)
    }
}
