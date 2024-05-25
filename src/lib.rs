pub mod buffer;
pub mod direction;
pub mod terminal_app;
pub mod view;

pub use buffer::Color;
pub use terminal_app::*;
use unicode_width::UnicodeWidthStr;
pub use view::*;

use std::{any::Any, sync::Arc};

#[cfg(test)]
mod tests;

pub use crossterm::event::{KeyCode, KeyEvent};

/// Creates a text view
///
/// # Examples
/// ```
/// use terminus::*;
/// let view = text("Hello There");
/// ```
///
/// Rendered Output
/// ---------------
/// Hello There
pub fn text<S: AsRef<str>>(text: S) -> Text {
    let ref_text = text.as_ref();
    Text {
        text: ref_text.into(),
        width: ref_text.width() as u16,
    }
}

/// Creates a vertical stack view
///
/// # Examples
/// ```
/// use terminus::*;
/// let view = vstack((
///     text("Hello"),
///     text("There"),
/// ));
/// ```
///
/// Rendered Output
/// ---------------
/// Hello
/// There
pub fn vstack<VT: ViewTuple>(children: VT) -> VStack<VT> {
    VStack {
        children,
        spacing: 0,
        alignment: HorizontalAlignment::Left,
    }
}

/// Creates a horizontal stack view
///
/// # Examples
/// ```
/// use terminus::*;
/// let view = hstack((
///     text("Tick"),
///     text("Tock"),
/// ));
/// ```
///
/// Rendered Output
/// ---------------
/// Tick Tock
pub fn hstack<VT: ViewTuple>(children: VT) -> HStack<VT> {
    HStack {
        children,
        spacing: 1,
        alignment: VerticalAlignment::Top,
    }
}

/// Creates overlapping views
///
/// # Examples
/// ```
/// use terminus::*;
/// let view = zstack((
///     text("Layer 1"),
///     text("Layer 2"),
/// ));
/// ```
///
/// Rendered Output
/// ---------------
/// Layer 1
/// Layer 2
pub fn zstack<VT: ViewTuple>(children: VT) -> ZStack<VT> {
    ZStack {
        children,
        alignment: Alignment::TOP_LEFT,
    }
}

/// Creates a view given a function that's passed the current size of the view
pub fn with_size<F, V>(f: F) -> GeometryReader<F>
where
    F: Fn(buffer::Size) -> V,
    V: View,
{
    GeometryReader::new(f)
}

#[derive(Clone)]
pub struct AnyView {
    view: Arc<dyn View>,
}

impl private::Sealed for AnyView {}

impl AnyView {
    pub fn new(view: impl View + 'static) -> Self {
        AnyView {
            view: Arc::new(view),
        }
    }
}

impl View for AnyView {
    fn render(
        &self,
        id: &mut ViewId,
        context: Context,
        state: &mut AppState,
        buffer: &mut buffer::Buffer,
    ) {
        id.push_hashable(self.view.type_id());
        self.view.render(id, context, state, buffer);
        id.pop();
    }

    fn size(&self, proposed: buffer::Size) -> buffer::Size {
        self.view.size(proposed)
    }
}
