pub mod buffer;
pub mod direction;
pub mod terminal_app;
pub mod view;

pub use buffer::Color;
pub use terminal_app::*;
pub use view::*;

use direction::*;
use std::sync::Arc;

#[cfg(test)]
mod tests;

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
pub fn text(text: &str) -> Text {
    Text { text: text.to_string() }
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
pub fn vstack<VT: ViewTuple>(children: VT) -> Stack<VT> {
    Stack {
        children,
        direction: Direction::Vertical,
        spacing: 0,
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
pub fn hstack<VT: ViewTuple>(children: VT) -> Stack<VT> {
    Stack {
        children,
        direction: Direction::Horizontal,
        spacing: 1,
    }
}

#[derive(Clone)]
pub struct AnyView {
    view: Arc<dyn View>,
}

impl AnyView {
    pub fn new(view: impl View + 'static) -> Self {
        AnyView { view: Arc::new(view) }
    }
}

impl View for AnyView {
    fn render(&self, context: RenderContext, buffer: &mut buffer::Buffer) {
        self.view.render(context, buffer)
    }

    fn size(&self, proposed: buffer::Size) -> buffer::Size {
        self.view.size(proposed)
    }
}
