use self::view::View;

use super::*;

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
    F: Fn(Size) -> V,
{
    fn size(&self, proposed: Size) -> Size {
        let f = &self.view;
        f(proposed).size(proposed)
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let f = &self.view;
        let view = f(context.rect.size);
        let size = view.size(context.rect.size);
        view.render(context.with_size(size), buffer);
    }
}

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

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let size = self.view.size(context.rect.size);
        self.background.render(context.with_size(size), buffer);
        self.view.render(context, buffer);
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

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let point = context.rect.point;
        let size = context.rect.size;
        for y in point.y..point.y + size.height {
            for x in point.x..point.x + size.width {
                buffer.set_char_at(x, y, ' ', Color::Reset, Some(self.color), Modifier::empty());
            }
        }
    }
}

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

    fn layout(&self, proposed: Size) -> Vec<Size> {
        let mut sizes = vec![Size::zero(); self.children.length()];

        let mut views_with_flex = self
            .children
            .make_iterator()
            .enumerate()
            .map(|(index, child)| {
                let lower = child.size(Size::new(0, proposed.height)).width;
                let upper = child.size(Size::new(u16::MAX, proposed.height)).width;
                (index, child, upper - lower)
            })
            .collect::<Vec<_>>();
        views_with_flex.sort_by(|a, b| a.2.cmp(&b.2));

        let total = self.children.length();
        let mut remaining_width = proposed.width;

        views_with_flex
            .iter()
            .enumerate()
            .for_each(|(i, (render_index, child, _))| {
                let width = remaining_width / (total - i) as u16;
                let child_size = child.size(Size::new(width, proposed.height));
                remaining_width = remaining_width
                    .saturating_sub(child_size.width)
                    .saturating_sub(self.spacing);
                sizes[*render_index] = child_size;
            });
        sizes
    }
}

impl<VT: ViewTuple> View for HStack<VT> {
    fn size(&self, proposed: Size) -> Size {
        let sizes = self.layout(proposed);
        let width: u16 =
            sizes.iter().map(|s| s.width).sum::<u16>() + (self.spacing * sizes.len().saturating_sub(1) as u16);
        let height: u16 = sizes.iter().map(|s| s.height).max().unwrap_or(0);
        Size {
            width: width.min(proposed.width),
            height: height.min(proposed.height),
        }
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let rect = context.rect;
        let sizes = self.layout(rect.size);
        let max_height: u16 = sizes.iter().map(|s| s.height).max().unwrap_or(0);

        let mut offset_x: u16 = 0;
        self.children.make_iterator().zip(sizes).for_each(|(child, size)| {
            let offset_y = match self.alignment {
                VerticalAlignment::Top => 0,
                VerticalAlignment::Center => (max_height.saturating_sub(size.height)) / 2,
                VerticalAlignment::Bottom => max_height.saturating_sub(size.height),
            };
            child.render(context.offset(offset_x, offset_y).with_size(size), buffer);
            offset_x += size.width + self.spacing;
        });
    }
}

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

    fn layout(&self, proposed: Size) -> Vec<Size> {
        let total = self.children.length();
        let mut sizes = vec![Size::zero(); total]; // Initialize with default Size values

        let mut views_with_flex = self
            .children
            .make_iterator()
            .enumerate()
            .map(|(index, child)| {
                let lower = child.size(Size::new(proposed.width, 0)).height;
                let upper = child.size(Size::new(proposed.width, u16::MAX)).height;
                (index, child, upper - lower)
            })
            .collect::<Vec<_>>();
        views_with_flex.sort_by(|a, b| a.2.cmp(&b.2));

        let mut remaining_height = proposed.height;

        views_with_flex
            .iter()
            .enumerate()
            .for_each(|(i, (render_index, child, _))| {
                let height = remaining_height / (total - i) as u16;
                let child_size = child.size(Size::new(proposed.width, height));
                remaining_height = remaining_height
                    .saturating_sub(child_size.height)
                    .saturating_sub(self.spacing);
                sizes[*render_index] = child_size;
            });
        sizes
    }
}

impl<VT: ViewTuple> View for VStack<VT> {
    fn size(&self, proposed: Size) -> Size {
        let sizes = self.layout(proposed);
        let width: u16 = sizes.iter().map(|s| s.width).max().unwrap_or(0);
        let height: u16 =
            sizes.iter().map(|s| s.height).sum::<u16>() + (self.spacing * sizes.len().saturating_sub(1) as u16);
        Size {
            width: width.min(proposed.width),
            height: height.min(proposed.height),
        }
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let rect = context.rect;
        let sizes = self.layout(rect.size);
        let max_width: u16 = sizes.iter().map(|s| s.width).max().unwrap_or(0);

        let mut offset_y: u16 = 0;
        self.children.make_iterator().zip(sizes).for_each(|(child, size)| {
            let offset_x = match self.alignment {
                HorizontalAlignment::Left => 0,
                HorizontalAlignment::Center => (max_width.saturating_sub(size.width)) / 2,
                HorizontalAlignment::Right => max_width.saturating_sub(size.width),
            };
            child.render(context.offset(offset_x, offset_y).with_size(size), buffer);
            offset_y += size.height + self.spacing;
        });
    }
}

#[derive(Clone, Debug)]
pub struct ZStack<VT> {
    pub(crate) children: VT,
    pub(crate) alignment: Alignment,
}

impl<VT> private::Sealed for ZStack<VT> {}

impl<VT: ViewTuple> ZStack<VT> {
    pub fn new(children: VT, alignment: Alignment) -> Self {
        Self { children, alignment }
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

impl<VT: ViewTuple> View for ZStack<VT> {
    fn size(&self, proposed: Size) -> Size {
        let mut width = 0;
        let mut height = 0;
        self.children.make_iterator().for_each(|child| {
            let size = child.size(proposed);
            width = width.max(size.width);
            height = height.max(size.height);
        });
        Size {
            width: width.min(proposed.width),
            height: height.min(proposed.height),
        }
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let sizes = self
            .children
            .make_iterator()
            .map(|child| (child, child.size(context.rect.size)))
            .collect::<Vec<_>>();

        let (max_width, max_height) = sizes
            .iter()
            .fold((0, 0), |(mw, mh), s| (mw.max(s.1.width), mh.max(s.1.height)));

        for (child, size) in sizes {
            let alignment_offset = match self.alignment {
                Alignment::TOP_LEFT => (0, 0),
                Alignment::TOP => (max_width / 2 - size.width / 2, 0),
                Alignment::TOP_RIGHT => (max_width - size.width, 0),
                Alignment::LEFT => (0, max_height / 2 - size.height / 2),
                Alignment::CENTER => (max_width / 2 - size.width / 2, max_height / 2 - size.height / 2),
                Alignment::RIGHT => (max_width - size.width, max_height / 2 - size.height / 2),
                Alignment::BOTTOM_LEFT => (0, max_height - size.height),
                Alignment::BOTTOM => (max_width / 2 - size.width / 2, max_height - size.height),
                Alignment::BOTTOM_RIGHT => (max_width - size.width, max_height - size.height),
            };

            child.render(context.offset(alignment_offset.0, alignment_offset.1).clone(), buffer);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::assert_rendered_view;

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
            text("Centered Text").frame(None, None, Some(u16::MAX), Some(u16::MAX), Alignment::BOTTOM_LEFT),
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
        let view = vstack((text("TOP"), text("CENTER").center().border(), text("BOTTOM")));

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
