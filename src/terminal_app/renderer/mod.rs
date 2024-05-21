use std::io::Write;

use crate::View;

use self::{fullscreen_renderer::FullScreenRenderer, inline_renderer::InlineRenderer};

pub mod fullscreen_renderer;
pub mod inline_renderer;

pub trait Renderer {
    fn render(&mut self, view: &impl View);
    fn resize(&mut self, terminal_width: u16, terminal_height: u16);
    fn move_cursor_to_bottom_of_current_view(&mut self);
}

pub(crate) enum SomeRenderer<W: Write> {
    FullScreen(FullScreenRenderer<W>),
    Inline(InlineRenderer<W>),
}

impl<W: Write> Renderer for SomeRenderer<W> {
    fn render(&mut self, view: &impl View) {
        match self {
            SomeRenderer::FullScreen(ref mut renderer) => renderer.render(view),
            SomeRenderer::Inline(ref mut renderer) => renderer.render(view),
        }
    }

    fn resize(&mut self, terminal_width: u16, terminal_height: u16) {
        match self {
            SomeRenderer::FullScreen(ref mut renderer) => renderer.resize(terminal_width, terminal_height),
            SomeRenderer::Inline(ref mut renderer) => renderer.resize(terminal_width, terminal_height),
        }
    }

    fn move_cursor_to_bottom_of_current_view(&mut self) {
        match self {
            SomeRenderer::FullScreen(ref mut renderer) => renderer.move_cursor_to_bottom_of_current_view(),
            SomeRenderer::Inline(ref mut renderer) => renderer.move_cursor_to_bottom_of_current_view(),
        }
    }
}
