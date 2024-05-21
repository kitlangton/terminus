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

impl<VT: ViewTuple> View for Stack<VT> {
    fn size(&self, proposed: Size) -> Size {
        let (mut width_acc, mut height_acc, mut count): (u16, u16, u16) = (0, 0, 0);
        match self.direction {
            Direction::Horizontal => {
                let mut remaining_width = proposed.width;
                self.children.for_each(|child| {
                    let proposed = Size {
                        width: remaining_width,
                        height: proposed.height,
                    };
                    let size = child.size(proposed);
                    remaining_width = remaining_width.saturating_sub(size.width);
                    width_acc += size.width;
                    height_acc = height_acc.max(size.height);
                    count += 1;
                });

                width_acc += self.spacing * count.saturating_sub(1);

                Size {
                    height: height_acc.min(proposed.height),
                    width: width_acc.min(proposed.width),
                }
            }
            Direction::Vertical => {
                let mut remaining_height = proposed.height;
                self.children.for_each(|child| {
                    let proposed = Size {
                        width: proposed.width,
                        height: remaining_height,
                    };
                    let size = child.size(proposed);
                    width_acc = width_acc.max(size.width);
                    height_acc += size.height;
                    remaining_height = remaining_height.saturating_sub(size.height);
                    count += 1;
                });

                height_acc += self.spacing * count.saturating_sub(1);

                Size {
                    width: width_acc.min(proposed.width),
                    height: height_acc.min(proposed.height),
                }
            }
        }
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let rect = context.rect;
        match self.direction {
            Direction::Horizontal => {
                let mut remaining_width = rect.size.width;
                let mut offset = rect.left();
                let y = rect.top();
                self.children.for_each(|child| {
                    let size = child.size(rect.size);
                    let rect = Rect {
                        point: Point { x: offset, y },
                        size: Size {
                            width: remaining_width,
                            height: rect.size.height,
                        },
                    };
                    child.render(context.with_rect(rect), buffer);
                    offset += size.width + self.spacing;
                    remaining_width = remaining_width.saturating_sub(size.width + self.spacing);
                });
            }
            Direction::Vertical => {
                let mut remaining_height = rect.size.height;
                let mut offset = rect.top();
                let x = rect.left();
                self.children.for_each(|child| {
                    let size = child.size(rect.size);
                    let rect = Rect {
                        point: Point { x, y: offset },
                        size: Size {
                            width: rect.size.width,
                            height: remaining_height,
                        },
                    };
                    child.render(context.with_rect(rect), buffer);
                    offset += size.height + self.spacing;
                    remaining_height = remaining_height.saturating_sub(size.height + self.spacing);
                });
            }
        }
    }
}
