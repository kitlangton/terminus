use self::view::View;

use super::*;

#[derive(Clone, Debug)]
pub struct Stack<VT> {
    pub(crate) children: VT,
    pub(crate) direction: Direction,
    pub(crate) spacing: u16,
}

impl<VT> private::Sealed for Stack<VT> {}

impl<VT: ViewTuple> Stack<VT> {
    pub fn new(children: VT, direction: Direction, spacing: u16) -> Self {
        Self {
            children,
            direction,
            spacing,
        }
    }

    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }
}

impl<VT: ViewTuple> Stack<VT> {
    fn layout_horizontal(&self, proposed: Size) -> Vec<Size> {
        let mut sizes = vec![Size::zero(); self.children.length()];

        let mut views_with_flex = self
            .children
            .make_iterator()
            .enumerate()
            .map(|(index, child)| {
                let lower = child.size(Size::new(0, proposed.height)).width;
                let upper = child.size(Size::new(proposed.width, 0)).width;
                (index, child, upper - lower)
            })
            .collect::<Vec<_>>();
        views_with_flex.sort_by(|a, b| a.2.cmp(&b.2));

        let total = self.children.length();
        let mut remaining_width = proposed.width;

        views_with_flex
            .iter()
            .enumerate()
            .for_each(|(i, (render_index, child, flex))| {
                let width = remaining_width / (total - i) as u16;
                let size = child.size(Size::new(width, proposed.height));
                remaining_width = remaining_width.saturating_sub(size.width).saturating_sub(self.spacing);
                sizes[*render_index] = size;
            });
        sizes
    }

    fn layout_vertical(&self, proposed: Size) -> Vec<Size> {
        let total = self.children.length();
        let mut sizes = vec![Size::zero(); total]; // Initialize with default Size values

        let mut views_with_flex = self
            .children
            .make_iterator()
            .enumerate()
            .map(|(index, child)| {
                let lower = child.size(Size::new(proposed.width, 0)).height;
                let upper = child.size(Size::new(0, proposed.height)).height;
                (index, child, upper - lower)
            })
            .collect::<Vec<_>>();
        views_with_flex.sort_by(|a, b| a.2.cmp(&b.2));

        let mut remaining_height = proposed.height;

        views_with_flex
            .iter()
            .enumerate()
            .for_each(|(i, (render_index, child, flex))| {
                let height = remaining_height / (total - i) as u16;
                let size = child.size(Size::new(proposed.width, height));
                remaining_height = remaining_height
                    .saturating_sub(size.height)
                    .saturating_sub(self.spacing);
                sizes[*render_index] = size;
            });
        sizes
    }
}

impl<VT: ViewTuple> View for Stack<VT> {
    fn size(&self, proposed: Size) -> Size {
        match self.direction {
            Direction::Horizontal => {
                let sizes = self.layout_horizontal(proposed);
                let width: u16 =
                    sizes.iter().map(|s| s.width).sum::<u16>() + (self.spacing * sizes.len().saturating_sub(1) as u16);
                let height: u16 = sizes.iter().map(|s| s.height).max().unwrap_or(0);
                Size {
                    width: width.min(proposed.width),
                    height: height.min(proposed.height),
                }
            }
            Direction::Vertical => {
                let sizes = self.layout_vertical(proposed);
                let width: u16 = sizes.iter().map(|s| s.width).max().unwrap_or(0);
                let height: u16 =
                    sizes.iter().map(|s| s.height).sum::<u16>() + (self.spacing * sizes.len().saturating_sub(1) as u16);
                Size {
                    width: width.min(proposed.width),
                    height: height.min(proposed.height),
                }
            }
        }
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let rect = context.rect;
        match self.direction {
            Direction::Horizontal => {
                let sizes = self.layout_horizontal(rect.size);

                let mut x: u16 = 0;
                self.children.make_iterator().zip(sizes).for_each(|(child, size)| {
                    child.render(context.offset(x, 0), buffer);
                    x += size.width + self.spacing;
                });
            }
            Direction::Vertical => {
                let sizes = self.layout_vertical(rect.size);

                let mut y: u16 = 0;
                self.children.make_iterator().zip(sizes).for_each(|(child, size)| {
                    child.render(context.offset(0, y), buffer);
                    y += size.height + self.spacing;
                });
            }
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
            "hello   ", //
            "1234567 ", //
            "cool    ", //
        ];
        assert_rendered_view(stack, expected_output, 7, 3);
    }
}
