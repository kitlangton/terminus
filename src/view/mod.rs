pub mod border;
pub mod frame;
pub mod padding;
pub mod stack;
pub mod text;
pub mod view_tuple;

use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};

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
/// )).border();
///
/// let expected = vec![
///     "┌────────────┐",
///     "│ 1. Eggs    │",
///     "│ 2. Powders │",
///     "│ 3. Milk    │",
///     "└────────────┘",
/// ].join("\n");
///
/// assert_eq!(expected, view.as_str());
/// ```
pub trait View: private::Sealed + 'static {
    fn size(&self, proposed: Size) -> Size;

    fn render(&self, id: &mut ViewId, context: Context, state: &mut AppState, buffer: &mut Buffer);
}
// HELLO! Just re-connecting my mic :D
// Meanwhile. I'll demo the current state below...

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

    fn center_horizontally(self) -> Frame<Self> {
        self.frame(None, None, Some(u16::MAX), None, Alignment::CENTER)
    }

    fn fill_horizontally(self) -> Frame<Self> {
        self.frame(None, None, Some(u16::MAX), None, Alignment::LEFT)
    }

    fn fill_vertically(self) -> Frame<Self> {
        self.frame(None, None, None, Some(u16::MAX), Alignment::TOP)
    }

    fn fill(self) -> Frame<Self> {
        self.frame(
            None,
            None,
            Some(u16::MAX),
            Some(u16::MAX),
            Alignment::TOP_LEFT,
        )
    }

    fn center_vertically(self) -> Frame<Self> {
        self.frame(None, None, None, Some(u16::MAX), Alignment::CENTER)
    }

    fn min_height(self, min_height: u16) -> Frame<Self> {
        self.frame(None, Some(min_height), None, None, Alignment::TOP)
    }

    fn min_width(self, min_width: u16) -> Frame<Self> {
        self.frame(Some(min_width), None, None, None, Alignment::LEFT)
    }

    fn center(self) -> Frame<Self> {
        self.frame(
            None,
            None,
            Some(u16::MAX),
            Some(u16::MAX),
            Alignment::CENTER,
        )
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
            modifier: None,
        }
    }

    fn green(self) -> ContextModifier<Self> {
        self.color(Color::Green)
    }

    fn red(self) -> ContextModifier<Self> {
        self.color(Color::Red)
    }

    fn blue(self) -> ContextModifier<Self> {
        self.color(Color::Blue)
    }

    fn yellow(self) -> ContextModifier<Self> {
        self.color(Color::Yellow)
    }

    fn white(self) -> ContextModifier<Self> {
        self.color(Color::Green)
    }

    fn black(self) -> ContextModifier<Self> {
        self.color(Color::Black)
    }

    fn cyan(self) -> ContextModifier<Self> {
        self.color(Color::Cyan)
    }

    fn magenta(self) -> ContextModifier<Self> {
        self.color(Color::Magenta)
    }

    fn background(self, color: Color) -> Background<Self, FillColor> {
        Background {
            view: self,
            background: FillColor { color },
        }
    }

    fn bold(self) -> ContextModifier<Self> {
        ContextModifier::modifier(self, Modifier::BOLD)
    }

    fn bold_when(self, condition: bool) -> ContextModifier<Self> {
        ContextModifier::modifier(
            self,
            if condition {
                Modifier::BOLD
            } else {
                Modifier::empty()
            },
        )
    }

    fn italic(self) -> ContextModifier<Self> {
        ContextModifier::modifier(self, Modifier::ITALIC)
    }

    fn italic_when(self, condition: bool) -> ContextModifier<Self> {
        ContextModifier::modifier_when(self, condition, Modifier::ITALIC)
    }

    fn underline(self) -> ContextModifier<Self> {
        ContextModifier::modifier(self, Modifier::UNDERLINE)
    }

    fn underline_when(self, condition: bool) -> ContextModifier<Self> {
        ContextModifier::modifier_when(self, condition, Modifier::UNDERLINE)
    }

    fn dim(self) -> ContextModifier<Self> {
        ContextModifier::modifier(self, Modifier::DIM)
    }

    fn dim_when(self, condition: bool) -> ContextModifier<Self> {
        ContextModifier::modifier_when(self, condition, Modifier::DIM)
    }

    fn id<ID: Hash>(self, id: ID) -> IdentifiedView<Self> {
        IdentifiedView::new(id, self)
    }

    fn strikethrough(self) -> ContextModifier<Self> {
        ContextModifier::modifier(self, Modifier::STRIKETHROUGH)
    }

    fn strikethrough_when(self, condition: bool) -> ContextModifier<Self> {
        ContextModifier::modifier_when(self, condition, Modifier::STRIKETHROUGH)
    }

    fn visible(self, condition: bool) -> IfThenView<Self, EmptyView> {
        IfThenView {
            condition,
            true_view: self,
            false_view: empty(),
        }
    }

    fn as_any(self) -> AnyView
    where
        Self: 'static,
    {
        AnyView::new(self)
    }

    /// Returns the type ID of the underlying view.
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn as_str(self) -> String {
        let size = self.size(Size::max());
        let mut buffer = Buffer::new(size.width, size.height);
        self.render(
            &mut ViewId::empty(),
            Context::new(Rect::new(0, 0, size.width, size.height)),
            &mut AppState::new(),
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

    fn render(&self, id: &mut ViewId, context: Context, state: &mut AppState, buffer: &mut Buffer) {
        if let Some(view) = self {
            view.render(id, context, state, buffer);
        }
    }
}

mod context_modifier;

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmptyView;

impl private::Sealed for EmptyView {}

impl View for EmptyView {
    fn size(&self, _proposed: Size) -> Size {
        Size::zero()
    }

    fn render(
        &self,
        _id: &mut ViewId,
        _context: Context,
        _state: &mut AppState,
        _buffer: &mut Buffer,
    ) {
        // Do nothing
    }
}

pub fn empty() -> EmptyView {
    EmptyView {}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IfThenView<T, F> {
    condition: bool,
    true_view: T,
    false_view: F,
}

pub fn if_then_view<T: View, F: View>(
    condition: bool,
    true_view: T,
    false_view: F,
) -> IfThenView<T, F> {
    IfThenView {
        condition,
        true_view,
        false_view,
    }
}

impl<T: View, F: View> private::Sealed for IfThenView<T, F> {}

impl<T: View, F: View> View for IfThenView<T, F> {
    fn size(&self, proposed: Size) -> Size {
        if self.condition {
            self.true_view.size(proposed)
        } else {
            self.false_view.size(proposed)
        }
    }

    fn render(&self, id: &mut ViewId, context: Context, state: &mut AppState, buffer: &mut Buffer) {
        if self.condition {
            id.push(1);
            self.true_view.render(id, context, state, buffer);
        } else {
            id.push(0);
            self.false_view.render(id, context, state, buffer);
        }
        id.pop();
    }
}

/// Pilfered, with love, from rui [[https://github.com/audulus/rui]]
///
/// let mut stateMap: Arc<RwLock<HashMap<ViewId, Box<dyn Any>>>>
///
/// vstack(
///     hstack(view1, view2),
///     view3,
/// )
///
/// vstack( []
///     hstack( [0]
///       view1,  [0,0]
///       view2,  [0,1]
///      ),
///     view3, [1]
/// )
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ViewId {
    path: Vec<u64>,
}

fn do_hash<H: Hash>(id: H) -> u64 {
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    hasher.finish()
}

impl ViewId {
    pub(crate) fn empty() -> Self {
        Self { path: vec![] }
    }

    pub(crate) fn push_hashable<H: Hash>(&mut self, id: H) {
        self.path.push(do_hash(id));
    }

    pub(crate) fn push(&mut self, id: u64) {
        self.path.push(id);
    }

    pub(crate) fn pop(&mut self) {
        self.path.pop().unwrap();
    }
}

use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Debug)]
pub struct AppState {
    pub view_map: HashMap<ViewId, Box<dyn Any + Send>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            view_map: HashMap::new(),
        }
    }

    pub fn get_mut<T: Any + 'static + Send>(
        &mut self,
        view_id: &ViewId,
        default: impl FnOnce() -> T,
    ) -> &mut T {
        let entry = self.view_map.entry(view_id.clone());
        let value = entry.or_insert_with(|| Box::new(default()));
        value.downcast_mut::<T>().unwrap()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderCounter {}

impl private::Sealed for RenderCounter {}

impl View for RenderCounter {
    fn size(&self, proposed: Size) -> Size {
        Size::new(80, 1).min(proposed)
    }

    fn render(&self, id: &mut ViewId, context: Context, state: &mut AppState, buffer: &mut Buffer) {
        let count = state.get_mut(id, || 0);
        *count += 1;

        let rect = context.rect;
        buffer.set_string_at(
            rect.point.x,
            rect.point.y,
            rect.size.width,
            &format!("Render {:?}: {}", id.path, count),
            context.fg,
            None,
            context.modifier,
        );
    }
}
