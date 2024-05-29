use super::*;
#[derive(Clone, Debug)]
pub struct HStack<VT> {
    pub(crate) children: VT,
    pub(crate) spacing: u16,
    pub(crate) alignment: VerticalAlignment,
}

impl<VT> private::Sealed for HStack<VT> {}

impl<VT: ViewTuple> HStack<VT> {
    pub fn new(children: VT, spacing: u16, alignment: VerticalAlignment) -> Self {
        Self {
            children,
            spacing,
            alignment,
        }
    }

    pub fn alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    fn layout(&self, proposed: Size) -> (Vec<Size>, u16, u16) {
        let mut sizes = vec![Size::zero(); self.children.length()];

        let mut views_with_flex = self
            .children
            .make_iterator()
            .enumerate()
            .map(|(index, child)| {
                let lower = child.value.size(Size::new(0, proposed.height)).width;
                let upper = child.value.size(Size::new(u16::MAX, proposed.height)).width;
                (index, child, upper - lower)
            })
            .collect::<Vec<_>>();
        views_with_flex.sort_by(|a, b| a.2.cmp(&b.2));

        let total = self.children.length();
        let mut remaining_width = proposed.width;
        let mut total_width = 0;
        let mut max_height = 0;

        views_with_flex
            .iter_mut()
            .enumerate()
            .for_each(|(i, (render_index, child, _))| {
                let width = remaining_width / (total - i) as u16;
                let child_size = child.value.size(Size::new(width, proposed.height));
                remaining_width = remaining_width
                    .saturating_sub(child_size.width)
                    .saturating_sub(self.spacing);
                sizes[*render_index] = child_size;
                total_width += child_size.width + self.spacing;
                if child_size.height > max_height {
                    max_height = child_size.height;
                }
            });

        total_width = total_width.saturating_sub(self.spacing); // Remove the last added spacing

        (sizes, total_width, max_height)
    }
}

impl<VT: ViewTuple + 'static> View for HStack<VT> {
    fn size(&self, proposed: Size) -> Size {
        let (_, max_width, max_height) = self.layout(proposed);
        Size {
            width: max_width.min(proposed.width),
            height: max_height.min(proposed.height),
        }
    }

    fn render(&self, id: &mut ViewId, context: Context, state: &mut AppState, buffer: &mut Buffer) {
        let rect = context.rect;
        let (sizes, _, max_height) = self.layout(rect.size);

        let mut offset_x: u16 = 0;
        self.children
            .make_iterator()
            .zip(sizes)
            .for_each(|(child, size)| {
                let offset_y = match self.alignment {
                    VerticalAlignment::Top => 0,
                    VerticalAlignment::Center => (max_height.saturating_sub(size.height)) / 2,
                    VerticalAlignment::Bottom => max_height.saturating_sub(size.height),
                };
                id.push(child.id);
                child.value.render(
                    id,
                    context.clone().offset(offset_x, offset_y).with_size(size),
                    state,
                    buffer,
                );
                id.pop();
                offset_x += size.width + self.spacing;
            });
    }
}

#[cfg(test)]
mod tests {
    use view::tests::assert_rendered_view;

    use super::*;

    #[test]
    fn test_horizontal_stack_sizing() {
        let stack = hstack((vstack(("hello", "world")), "1234567", "cool"));

        let size = stack.size(Size::MAX);

        assert_eq!(size.width, 18); // Sum of widths + spacing
        assert_eq!(size.height, 2); // The maximum height of the children
    }

    #[test]
    fn test_horizontal_stack_rendering() {
        let stack = hstack((vstack(("hello", "world")), "1234567", "cool"));
        let expected_output = vec![
            "hello 1234567 cool", //
            "world             ", //
        ];
        assert_rendered_view(stack, expected_output, 18, 2);
    }

    #[test]
    fn test_border_view_in_hstack_center_vertically() {
        let view = hstack((
            text("LEFT"), //
            text("CENTER").center().border(),
            text("RIGHT"),
        ));

        let expected_output = vec![
            "LEFT ┌────────────┐ RIGHT", //
            "     │            │      ", //
            "     │            │      ", //
            "     │   CENTER   │      ", //
            "     │            │      ", //
            "     │            │      ", //
            "     └────────────┘      ", //
        ];
        assert_rendered_view(view, expected_output, 25, 7);
    }

    #[test]
    fn test_bottom_alignment_in_hstack() {
        let view = hstack((
            text("LEFT"), //
            text("CENTER").center().border(),
            text("RIGHT"),
        ))
        .alignment(VerticalAlignment::Bottom);

        let expected_output = vec![
            "     ┌────────────┐      ", //
            "     │            │      ", //
            "     │            │      ", //
            "     │   CENTER   │      ", //
            "     │            │      ", //
            "     │            │      ", //
            "LEFT └────────────┘ RIGHT", //
        ];
        assert_rendered_view(view, expected_output, 25, 7);
    }

    #[test]
    fn test_horizontal_alignment_in_hstack() {
        let stack = hstack((
            text("A"),                                 //
            vstack((text("A"), text("A"), text("A"))), //
            vstack((
                text("A"),
                text("A"),
                text("A"),
                text("A"),
                text("A"),
                text("A"),
                text("A"),
                text("A"),
                text("A"),
                text("A"),
                text("A"),
            )), //
            vstack((text("A"), text("A"), text("A"))), //
            text("A"),
        ))
        .alignment(VerticalAlignment::Center);

        let expected_output = vec![
            "    A    ",
            "    A    ",
            "    A    ",
            "    A    ",
            "  A A A  ",
            "A A A A A",
            "  A A A  ",
            "    A    ",
            "    A    ",
            "    A    ",
            "    A    ",
        ];
        assert_rendered_view(stack, expected_output, 9, 11);
    }
}
