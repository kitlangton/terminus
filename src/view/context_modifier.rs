use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextModifier<V> {
    pub(crate) child: V,
    pub(crate) fg: Option<Color>,
    pub(crate) modifier: Option<Modifier>,
}

impl<V> private::Sealed for ContextModifier<V> {}

impl<V: View> ContextModifier<V> {
    pub(crate) fn modifier(child: V, modifier: Modifier) -> Self {
        Self {
            child,
            fg: None,
            modifier: Some(modifier),
        }
    }

    pub(crate) fn modifier_when(child: V, condition: bool, modifier: Modifier) -> Self {
        Self {
            child,
            fg: None,
            modifier: if condition { Some(modifier) } else { None },
        }
    }
}

impl<V: View> View for ContextModifier<V> {
    fn size(&self, proposed: Size) -> Size {
        self.child.size(proposed)
    }

    fn render(&self, id: &mut ViewId, context: Context, state: &mut AppState, buffer: &mut Buffer) {
        let context = context.with_fg(self.fg).with_modifier(self.modifier);
        self.child.render(id, context, state, buffer);
    }
}
