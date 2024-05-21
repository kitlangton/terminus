use crate::buffer::*;

use self::view::View;

use super::*;
use pretty_assertions::assert_eq;

pub(crate) fn assert_rendered_view(view: impl View, expected: Vec<&str>, buffer_width: u16, buffer_height: u16) {
    let mut buffer = Buffer::new(buffer_width, buffer_height);
    let render_context = RenderContext::new(Rect {
        point: Point::zero(),
        size: buffer.size.clone(),
    });
    view.render(render_context, &mut buffer);
    let result: String = buffer.as_str();
    assert_eq!(result, expected.join("\n"));
}

#[test]
fn test_rendered_padded_text() {
    let padded_text_view = text("RUST").padding(1);
    let expected = vec![
        "      ", //
        " RUST ", //
        "      ",
    ];
    assert_rendered_view(padded_text_view, expected, 6, 3);
}

#[test]
fn test_vstack_of_texts() {
    let vstack = vstack((text("Hello"), text("World"), text("!!!!!")));
    let expected = vec![
        "Hello", //
        "World", //
        "!!!!!", //
    ];
    let size = vstack.size(Size::max());
    assert_rendered_view(vstack, expected, size.width, size.height);
}

#[test]
fn test_hstack_with_spacing() {
    let hstack_view = hstack((text("A"), text("B"), text("C"))).spacing(2);
    let expected = vec!["A  B  C"];
    let size = hstack_view.size(Size::max());
    assert_rendered_view(hstack_view, expected, size.width, size.height);
}

#[test]
fn test_vstack_with_padded_text() {
    let vstack = vstack((text("A"), text("B").padding(1), text("C")));
    let expected = vec![
        "A  ", //
        "   ", //
        " B ", //
        "   ", //
        "C  ",
    ];
    let size = vstack.size(Size::max());
    assert_rendered_view(vstack, expected, size.width, size.height);
}

#[test]
fn test_alternating_hstack_vstack() {
    let alternating_stack = vstack((text("B"), hstack((text("C"), text("D"), text("E"))), text("F")));
    let expected = vec![
        "B    ", //
        "C D E", //
        "F    ", //
    ];
    let size = alternating_stack.size(Size::max());
    assert_rendered_view(alternating_stack, expected, size.width, size.height);
}
