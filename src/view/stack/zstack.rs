use super::*;
#[derive(Clone, Debug)]
pub struct ZStack<VT> {
    pub(crate) children: VT,
    pub(crate) alignment: Alignment,
}

impl<VT> private::Sealed for ZStack<VT> {}

impl<VT: ViewTuple> ZStack<VT> {
    pub fn new(children: VT, alignment: Alignment) -> Self {
        Self {
            children,
            alignment,
        }
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

impl<VT: ViewTuple + 'static> View for ZStack<VT> {
    fn size(&self, proposed: Size) -> Size {
        let (width, height) =
            self.children
                .make_iterator()
                .fold((0, 0), |(max_width, max_height), child| {
                    let size = child.value.size(proposed);
                    (max_width.max(size.width), max_height.max(size.height))
                });
        Size {
            width: width.min(proposed.width),
            height: height.min(proposed.height),
        }
    }

    fn render(&self, id: &mut ViewId, context: Context, state: &mut AppState, buffer: &mut Buffer) {
        let mut max_width = 0;
        let mut max_height = 0;

        let sizes = self
            .children
            .make_iterator()
            .map(|child| {
                let size = child.value.size(context.rect.size);
                if size.width > max_width {
                    max_width = size.width;
                }
                if size.height > max_height {
                    max_height = size.height;
                }
                (child, size)
            })
            .collect::<Vec<_>>();

        for (child, size) in sizes {
            let alignment_offset = match self.alignment {
                Alignment::TOP_LEFT => (0, 0),
                Alignment::TOP => (max_width / 2 - size.width / 2, 0),
                Alignment::TOP_RIGHT => (max_width - size.width, 0),
                Alignment::LEFT => (0, max_height / 2 - size.height / 2),
                Alignment::CENTER => (
                    max_width / 2 - size.width / 2,
                    max_height / 2 - size.height / 2,
                ),
                Alignment::RIGHT => (max_width - size.width, max_height / 2 - size.height / 2),
                Alignment::BOTTOM_LEFT => (0, max_height - size.height),
                Alignment::BOTTOM => (max_width / 2 - size.width / 2, max_height - size.height),
                Alignment::BOTTOM_RIGHT => (max_width - size.width, max_height - size.height),
            };

            id.push(child.id);
            child.value.render(
                id,
                context
                    .clone()
                    .offset(alignment_offset.0, alignment_offset.1)
                    .with_size(size),
                state,
                buffer,
            );
            id.pop();
        }
    }
}
