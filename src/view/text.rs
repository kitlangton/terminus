use self::view::View;

use super::*;

#[derive(Clone, Debug)]
pub struct Text {
    pub(crate) text: String,
}

impl private::Sealed for Text {}

impl View for Text {
    fn size(&self, proposed: Size) -> Size {
        Size {
            width: (self.text.len() as u16).min(proposed.width),
            height: 1,
        }
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let rect = context.rect;
        let text = self.text.chars().take(rect.size.width as usize).collect::<String>();
        buffer.set_string_at(rect.left(), rect.top(), &text, context.fg, None, context.modifier);
    }
}

impl private::Sealed for &str {}

impl View for &str {
    fn size(&self, proposed: Size) -> Size {
        let text = Text { text: self.to_string() };
        text.size(proposed)
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let text = Text { text: self.to_string() };
        text.render(context, buffer);
    }
}

impl private::Sealed for String {}

impl View for String {
    fn size(&self, proposed: Size) -> Size {
        let text = Text { text: self.to_string() };
        text.size(proposed)
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let text = Text { text: self.to_string() };
        text.render(context, buffer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::*;
    use crate::view::RenderContext;

    #[test]
    fn test_text_size() {
        let text = Text {
            text: "Hello".to_string(),
        };
        let expected_size = Size::new(5, 1);
        assert_eq!(text.size(Size::max()), expected_size);
    }

    #[test]
    fn test_text_render() {
        let text = Text {
            text: "Hello".to_string(),
        };
        let mut buffer = Buffer::new(10, 1);
        let rect = Rect::new(0, 0, 10, 1);
        let context = RenderContext::new(rect);
        text.render(context, &mut buffer);

        let expected_output = "Hello     ";
        let result: String = buffer.as_str();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_text_render_truncated() {
        let text = Text {
            text: "Hello, World!".to_string(),
        };
        let mut buffer = Buffer::new(5, 1);
        let rect = Rect::new(0, 0, 5, 1);
        let context = RenderContext::new(rect);
        text.render(context, &mut buffer);

        let expected_output = "Hello";
        let result: String = buffer.as_str();
        assert_eq!(result, expected_output);
    }
}
