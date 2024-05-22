pub mod border;
pub mod frame;
pub mod padding;
pub mod stack;
pub mod text;
pub mod view_tuple;
use crate::*;

pub use border::{Border, BorderStyle};
pub use buffer::*;
pub use context_modifier::*;
pub use frame::*;
pub use padding::*;
pub use stack::*;
pub use text::*;
pub use view_tuple::*;

/// Syntax Examples
/// ---------------
/// Example of a VStack with nested HStacks
/// ```
/// use terminus::*;
/// let view = vstack((
///     hstack((text("1."), text("Eggs"))),
///     hstack((text("2."), text("Powders"))),
///     hstack((text("3."), text("Milk"))),
/// )); // .border()
/// ```
/// Rendered Output:
/// +----------------+
/// | 1. Eggs        |
/// | 2. Powders     |
/// | 3. Milk        |
/// +----------------+
pub trait View: private::Sealed {
    fn size(&self, proposed: Size) -> Size;

    fn render(&self, context: RenderContext, buffer: &mut Buffer);
}

pub(crate) mod private {
    pub trait Sealed {}
}

pub trait ViewExtensions: View + Sized {
    fn frame(
        self,
        min_width: Option<u16>,
        min_height: Option<u16>,
        max_width: Option<u16>,
        max_height: Option<u16>,
        alignment: Alignment,
    ) -> Frame<Self> {
        Frame {
            child: self,
            min_width,
            min_height,
            max_width,
            max_height,
            alignment,
        }
    }

    fn border(self) -> Border<Self> {
        Border {
            child: self,
            border_color: Color::Reset,
            border_style: BorderStyle::Single,
            title: None,
        }
    }

    fn padding(self, padding: u16) -> Padding<Self> {
        Padding {
            child: self,
            padding_top: padding,
            padding_bottom: padding,
            padding_left: padding,
            padding_right: padding,
        }
    }

    fn padding_h(self, padding: u16) -> Padding<Self> {
        Padding {
            child: self,
            padding_left: padding,
            padding_right: padding,
            padding_top: 0,
            padding_bottom: 0,
        }
    }

    fn padding_v(self, padding: u16) -> Padding<Self> {
        Padding {
            child: self,
            padding_top: padding,
            padding_bottom: padding,
            padding_left: 0,
            padding_right: 0,
        }
    }

    fn color(self, color: Color) -> ContextModifier<Self> {
        ContextModifier {
            child: self,
            fg: Some(color),
            bg: None,
            modifier: None,
        }
    }

    fn background(self, color: Color) -> ContextModifier<Self> {
        ContextModifier {
            child: self,
            fg: None,
            bg: Some(color),
            modifier: None,
        }
    }

    fn bold(self) -> ContextModifier<Self> {
        ContextModifier::modifier(self, Modifier::BOLD)
    }

    fn bold_when(self, condition: bool) -> ContextModifier<Self> {
        ContextModifier::modifier(self, if condition { Modifier::BOLD } else { Modifier::empty() })
    }

    fn underline(self) -> ContextModifier<Self> {
        ContextModifier::modifier(self, Modifier::UNDERLINE)
    }

    fn underline_when(self, condition: bool) -> ContextModifier<Self> {
        ContextModifier::modifier(
            self,
            if condition {
                Modifier::UNDERLINE
            } else {
                Modifier::empty()
            },
        )
    }

    fn dim(self) -> ContextModifier<Self> {
        ContextModifier::modifier(self, Modifier::DIM)
    }

    fn as_any(self) -> AnyView
    where
        Self: 'static,
    {
        AnyView::new(self)
    }

    fn as_str(self) -> String {
        let size = self.size(Size::max());
        let mut buffer = Buffer::new(size.width, size.height);
        self.render(
            RenderContext::new(Rect::new(0, 0, size.width, size.height)),
            &mut buffer,
        );
        buffer.as_str()
    }
}

impl<T: View> ViewExtensions for T {}

impl<V> private::Sealed for Option<V> {}

impl<V: View> View for Option<V> {
    fn size(&self, proposed: Size) -> Size {
        match self {
            Some(view) => view.size(proposed),
            None => Size::zero(),
        }
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        if let Some(view) = self {
            view.render(context, buffer);
        }
    }
}

mod context_modifier;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderContext {
    pub(crate) rect: Rect,
    pub(crate) fg: Color,
    pub(crate) bg: Color,
    pub(crate) modifier: Modifier,
}

impl RenderContext {
    pub(crate) fn new(rect: Rect) -> Self {
        Self {
            rect,
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::empty(),
        }
    }

    fn with_rect(&self, rect: Rect) -> Self {
        Self { rect, ..self.clone() }
    }

    fn with_size(&self, size: Size) -> Self {
        Self {
            rect: Rect {
                point: self.rect.point,
                size,
            },
            ..self.clone()
        }
    }

    pub fn offset(&self, offset_x: u16, offset_y: u16) -> RenderContext {
        Self {
            rect: self.rect.offset(offset_x, offset_y),
            ..self.clone()
        }
    }
}

impl Default for RenderContext {
    fn default() -> Self {
        Self {
            rect: Rect::new(0, 0, 0, 0),
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::empty(),
        }
    }
}

pub struct IfThenElse<T: View, F: View> {
    pub condition: bool,
    pub trueView: T,
    pub falseView: F,
}

impl<T: View, F: View> private::Sealed for IfThenElse<T, F> {}

impl<T: View, F: View> View for IfThenElse<T, F> {
    fn size(&self, proposed: Size) -> Size {
        if self.condition {
            self.trueView.size(proposed)
        } else {
            self.falseView.size(proposed)
        }
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        if self.condition {
            self.trueView.render(context, buffer);
        } else {
            self.falseView.render(context, buffer);
        }
    }
}

pub fn if_then_else_view<T: View, F: View>(condition: bool, trueView: T, falseView: F) -> IfThenElse<T, F> {
    IfThenElse {
        condition,
        trueView,
        falseView,
    }
}

pub struct EmptyView;

impl private::Sealed for EmptyView {}

impl View for EmptyView {
    fn size(&self, proposed: Size) -> Size {
        Size::zero()
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        // Do nothing
    }
}

pub fn empty() -> EmptyView {
    EmptyView {}
}
