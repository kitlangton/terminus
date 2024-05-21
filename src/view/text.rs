use self::view::View;

use super::*;

#[derive(Clone, Debug)]
pub struct Text {
    pub(crate) text: String,
}

impl View for Text {
    fn size(&self, proposed: Size) -> Size {
        Size {
            width: (self.text.len() as u16).min(proposed.width),
            height: 1,
        }
    }

    fn render(&self, context: RenderContext, buffer: &mut Buffer) {
        let rect = context.rect;
        let mut x = rect.left();
        let y = rect.top();
        let max_x = rect.right();
        for c in self.text.chars() {
            if x >= max_x {
                break;
            }
            buffer.set_char_at(x, y, c, context.fg, context.bg, context.modifier);
            x += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::{Buffer, Point, Rect, Size};
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
