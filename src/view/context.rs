use super::*;

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) rect: Rect,
    pub(crate) fg: Color,
    pub(crate) modifier: Modifier,
}

impl Context {
    pub(crate) fn new(rect: Rect) -> Self {
        Self {
            rect,
            fg: Color::Reset,
            modifier: Modifier::empty(),
        }
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.rect.size = size;
        self
    }

    pub fn inset_by(mut self, left: u16, right: u16, top: u16, bottom: u16) -> Self {
        self.rect = self.rect.inset_by(left, right, top, bottom);
        self
    }

    pub fn with_fg(mut self, fg: Option<Color>) -> Self {
        self.fg = fg.unwrap_or(self.fg);
        self
    }

    pub fn with_modifier(mut self, modifier: Option<Modifier>) -> Self {
        self.modifier = self.modifier | modifier.unwrap_or_else(Modifier::empty);
        self
    }

    pub fn offset(mut self, offset_x: u16, offset_y: u16) -> Self {
        self.rect = self.rect.offset(offset_x, offset_y);
        self
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            rect: Rect::new(0, 0, 0, 0),
            fg: Color::Reset,
            modifier: Modifier::empty(),
        }
    }
}
