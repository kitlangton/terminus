use super::*;
#[derive(Clone, Debug)]
pub struct VStack<VT> {
    pub(crate) children: VT,
    pub(crate) spacing: u16,
    pub(crate) alignment: HorizontalAlignment,
}

impl<VT> private::Sealed for VStack<VT> {}

impl<VT: ViewTuple> VStack<VT> {
    pub fn new(children: VT, spacing: u16, alignment: HorizontalAlignment) -> Self {
        Self {
            children,
            spacing,
            alignment,
        }
    }

    pub fn alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    fn layout(&self, proposed: Size) -> (Vec<Size>, u16, u16) {
        let total = self.children.length();
        let mut sizes = vec![Size::zero(); total]; // Initialize with default Size values

        let mut views_with_flex = self
            .children
            .make_iterator()
            .enumerate()
            .map(|(index, child)| {
                let lower = child.value.size(Size::new(proposed.width, 0)).height;
                let upper = child.value.size(Size::new(proposed.width, u16::MAX)).height;
                (index, child, upper - lower)
            })
            .collect::<Vec<_>>();
        views_with_flex.sort_by(|a, b| a.2.cmp(&b.2));

        let mut remaining_height = proposed.height;
        let mut max_width = 0;
        let mut total_height = 0;
        views_with_flex
            .iter()
            .enumerate()
            .for_each(|(i, (render_index, child, _))| {
                let height = remaining_height / (total - i) as u16;
                let child_size = child.value.size(Size::new(proposed.width, height));
                remaining_height = remaining_height
                    .saturating_sub(child_size.height)
                    .saturating_sub(self.spacing);

                total_height += child_size.height + self.spacing;
                if child_size.width > max_width {
                    max_width = child_size.width;
                }

                sizes[*render_index] = child_size;
            });

        total_height = total_height.saturating_sub(self.spacing);

        (sizes, max_width, total_height)
    }
}

impl<VT: ViewTuple + 'static> View for VStack<VT> {
    fn size(&self, proposed: Size) -> Size {
        let (_, max_width, max_height) = self.layout(proposed);
        Size {
            width: max_width.min(proposed.width),
            height: max_height.min(proposed.height),
        }
    }

    fn render(&self, id: &mut ViewId, context: Context, state: &mut AppState, buffer: &mut Buffer) {
        let rect = context.rect;
        let (sizes, max_width, _) = self.layout(rect.size);

        let mut offset_y: u16 = 0;
        self.children
            .make_iterator()
            .zip(sizes)
            .for_each(|(child, size)| {
                let offset_x = match self.alignment {
                    HorizontalAlignment::Left => 0,
                    HorizontalAlignment::Center => (max_width.saturating_sub(size.width)) / 2,
                    HorizontalAlignment::Right => max_width.saturating_sub(size.width),
                };
                id.push(child.id);
                child.value.render(
                    id,
                    context.clone().offset(offset_x, offset_y).with_size(size),
                    state,
                    buffer,
                );
                id.pop();
                offset_y += size.height + self.spacing;
            });
    }
}

#[cfg(test)]
mod tests {
    use view::tests::assert_rendered_view;

    use super::*;

    #[test]
    fn test_vertical_stack_sizing() {
        let stack = vstack(("hello", "1234567", "cool"));

        let size = stack.size(Size::MAX);

        assert_eq!(size.width, 7); // The maximum width of the children
        assert_eq!(size.height, 3); // Sum of heights + spacing
    }

    #[test]
    fn test_vertical_stack_rendering() {
        let stack = vstack(("hello", "1234567", "cool"));
        let expected_output = vec![
            "hello  ", //
            "1234567", //
            "cool   ", //
        ];
        assert_rendered_view(stack, expected_output, 7, 3);
    }

    #[test]
    fn test_bottom_left_alignment_in_vstack() {
        let stack = vstack((
            text("Centered Text").frame(
                None,
                None,
                Some(u16::MAX),
                Some(u16::MAX),
                Alignment::BOTTOM_LEFT,
            ),
            vstack((text("Bottom Text"), text("Bottom Text"))),
        ));

        let expected_output = vec![
            "             ", // Empty space for vertical centering
            "             ", // Empty space for vertical centering
            "             ", // Empty space for vertical centering
            "             ", // Empty space for vertical centering
            "Centered Text",
            "Bottom Text  ",
            "Bottom Text  ",
        ];
        assert_rendered_view(stack, expected_output, 13, 7);
    }

    #[test]
    fn test_border_view_in_vstack_center_vertically() {
        let view = vstack((
            text("TOP"), //
            text("CENTER").center().border(),
            text("BOTTOM"),
        ));

        let expected_output = vec![
            "TOP          ", // Top text
            "┌───────────┐", // Border top
            "│           │", // Centered text inside border
            "│           │", // Centered text inside border
            "│  CENTER   │", // Centered text inside border
            "│           │", // Centered text inside border
            "│           │", // Centered text inside border
            "└───────────┘", // Border bottom
            "BOTTOM       ", // Bottom text
        ];
        assert_rendered_view(view, expected_output, 13, 9);
    }

    #[test]
    fn test_vertical_alignment_in_vstack() {
        let stack = vstack((
            text("A"),           //
            text("AAA"),         //
            text("AAAAAAAAAAA"), //
            text("AAA"),         //
            text("A"),           //
        ))
        .alignment(HorizontalAlignment::Center);

        let expected_output = vec![
            "     A     ",
            "    AAA    ",
            "AAAAAAAAAAA",
            "    AAA    ",
            "     A     ",
        ];
        assert_rendered_view(stack, expected_output, 11, 5);
    }

    #[test]
    fn test_vertical_alignment_in_vstack_and_frame() {
        let stack = vstack((
            text("A"),     //
            text("AAA"),   //
            text("AAAAA"), //
            text("AAA"),   //
            text("A"),     //
        ))
        .alignment(HorizontalAlignment::Center)
        .center_horizontally();

        let expected_output = vec![
            "     A     ",
            "    AAA    ",
            "   AAAAA   ",
            "    AAA    ",
            "     A     ",
        ];
        assert_rendered_view(stack, expected_output, 11, 5);
    }
}
