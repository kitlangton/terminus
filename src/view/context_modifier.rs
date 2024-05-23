use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextModifier<V> {
    pub(crate) child: V,
    pub(crate) fg: Option<Color>,
    pub(crate) bg: Option<Color>,
    pub(crate) modifier: Option<Modifier>,
}

impl<V> private::Sealed for ContextModifier<V> {}

impl<V: View> ContextModifier<V> {
    pub(crate) fn modifier(child: V, modifier: Modifier) -> Self {
        Self {
            child,
            fg: None,
            bg: None,
            modifier: Some(modifier),
        }
    }
}

impl<V: View> View for ContextModifier<V> {
    fn size(&self, proposed: Size) -> Size {
        self.child.size(proposed)
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let context = RenderContext {
            rect: context.rect,
            fg: self.fg.unwrap_or(context.fg),
            bg: self.bg.unwrap_or(context.bg),
            modifier: context.modifier | self.modifier.unwrap_or(Modifier::empty()),
        };
        self.child.render(context, buffer);
    }
}
