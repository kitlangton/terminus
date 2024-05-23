use self::view::View;
use compact_str::CompactString;

use super::*;

#[derive(Clone, Debug)]
pub struct Text {
    pub(crate) text: CompactString,
    pub(crate) width: u16,
}

impl private::Sealed for Text {}

impl View for Text {
    fn size(&self, proposed: Size) -> Size {
        Size {
            width: self.width.min(proposed.width),
            height: 1,
        }
    }

    fn render(&self, context: Context, buffer: &mut Buffer) {
        let rect = context.rect;
        buffer.set_string_at(
            rect.left(),
            rect.top(),
            self.width.min(rect.size.width),
            &self.text,
            context.fg,
            None,
            context.modifier,
        );
    }
}

impl private::Sealed for &str {}

impl View for &'static str {
    fn size(&self, proposed: Size) -> Size {
        size_for_text(self, proposed)
    }

    fn render(&self, context: Context, buffer: &mut Buffer) {
        render_text(self, context, buffer);
    }
}

impl private::Sealed for String {}

impl View for String {
    fn size(&self, proposed: Size) -> Size {
        size_for_text(self, proposed)
    }

    fn render(&self, context: Context, buffer: &mut Buffer) {
        render_text(self, context, buffer);
    }
}

#[inline]
fn size_for_text(text: &str, proposed: Size) -> Size {
    Size {
        width: (text.len() as u16).min(proposed.width),
        height: 1,
    }
}

#[inline]
fn render_text(text: &str, context: Context, buffer: &mut Buffer) {
    let rect = context.rect;
    let width = size_for_text(text, rect.size).width;
    buffer.set_string_at(
        rect.left(),
        rect.top(),
        width,
        &text,
        context.fg,
        None,
        context.modifier,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::*;
    use crate::view::Context;

    #[test]
    fn test_text_size() {
        let text = text("Hello");
        let expected_size = Size::new(5, 1);
        assert_eq!(text.size(Size::max()), expected_size);
    }

    #[test]
    fn test_text_render() {
        let text = text("Hello");
        let mut buffer = Buffer::new(10, 1);
        let rect = Rect::new(0, 0, 10, 1);
        let context = Context::new(rect);
        text.render(context, &mut buffer);

        let expected_output = "Hello     ";
        let result: String = buffer.as_str();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_text_render_truncated() {
        let text = text("Hello, World!");
        let size = text.size(Size::MAX);
        assert_eq!(size, Size::new(13, 1));

        let mut buffer = Buffer::new(5, 1);
        let rect = Rect::new(0, 0, 5, 1);
        let context = Context::new(rect);
        text.render(context, &mut buffer);

        let expected_output = "Hello";
        let result: String = buffer.as_str();
        assert_eq!(result, expected_output);
    }

    /// Test the size of this char: █
    /// Technically it's 3 chars wide, but it should be sized and rendered as 1 column wide.
    #[test]
    fn test_char_size() {
        let char = text("█");
        let size = char.size(Size::max());
        assert_eq!(size, Size::new(1, 1));
    }
}
