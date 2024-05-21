use crate::{
    buffer::{Buffer, Modifier, Rect, Size},
    Color, RenderContext, View,
};
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

use super::Renderer;

pub(crate) struct FullScreenRenderer<W: Write> {
    writer: W,
    current_buffer: Buffer,
    prev_buffer: Buffer,
    terminal_size: Size,
}

impl<W: Write> Renderer for FullScreenRenderer<W> {
    fn render(&mut self, view: &impl View) {
        self.render(view)
    }

    fn resize(&mut self, terminal_width: u16, terminal_height: u16) {
        self.resize(terminal_width, terminal_height);
    }

    fn move_cursor_to_bottom_of_current_view(&mut self) {}
}

impl<W: Write> FullScreenRenderer<W> {
    /// Creates a new `FullScreenRenderer` with the given writer.
    pub(crate) fn new(writer: W) -> Self {
        let (terminal_width, terminal_height) = crossterm::terminal::size().unwrap();
        let terminal_size = Size::new(terminal_width, terminal_height);
        Self {
            writer,
            current_buffer: Buffer::new(terminal_width, terminal_height),
            prev_buffer: Buffer::new(terminal_width, terminal_height),
            terminal_size,
        }
    }

    /// Renders the given view to the terminal.
    pub(crate) fn render(&mut self, view: &impl View) {
        self.swap_buffers();
        let Size {
            width: view_width,
            height: view_height,
        } = view.size(self.terminal_size);

        assert!(
            view_height <= self.terminal_size.height || view_width <= self.terminal_size.width,
            "View height or width is greater than terminal height or width"
        );

        let rect = Rect::new(0, 0, view_width, view_height);
        view.render(RenderContext::new(rect), &mut self.current_buffer);
        self.print_buffer().unwrap();
    }

    fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.current_buffer, &mut self.prev_buffer);
    }

    /// Prints the buffer to the terminal.
    fn print_buffer(&mut self) -> io::Result<()> {
        // Diff the current buffer with the previous buffer
        // Returns the x, y, and cell for each diff
        let diff = self.current_buffer.diff(&self.prev_buffer);

        let mut last_fg = Color::Reset;
        let mut last_bg = Color::Reset;
        let mut last_modifier = Modifier::empty();
        let (mut last_x, mut last_y) = (u16::MAX - 1, u16::MAX - 1);

        for (x, y, cell) in diff {
            if cell.fg != last_fg {
                queue!(self.writer, SetForegroundColor(cell.fg.into()))?;
                last_fg = cell.fg;
            }

            if cell.bg != last_bg {
                queue!(self.writer, SetBackgroundColor(cell.bg.into()))?;
                last_bg = cell.bg;
            }

            if cell.modifier != last_modifier {
                Modifier::write_diff(cell.modifier, last_modifier, &mut self.writer);
                last_modifier = cell.modifier;
            }

            if last_x + 1 != x || last_y != y {
                queue!(self.writer, MoveTo(x, y))?;
            }
            queue!(self.writer, Print(&cell.symbol))?;

            (last_x, last_y) = (x, y);
        }

        // Reset colors and attributes at the end
        queue!(
            self.writer,
            SetForegroundColor(crossterm::style::Color::Reset),
            SetBackgroundColor(crossterm::style::Color::Reset),
            SetAttribute(crossterm::style::Attribute::Reset),
        )?;

        self.writer.flush()?;
        self.prev_buffer.clear();
        Ok(())
    }

    /// Resizes the terminal buffers to the specified width and height.
    ///
    /// This function clears the current view and initializes new buffers
    /// with the given dimensions.
    pub(crate) fn resize(&mut self, terminal_width: u16, terminal_height: u16) {
        queue!(self.writer, Clear(ClearType::All)).unwrap();

        self.current_buffer = Buffer::new(terminal_width, terminal_height);
        self.prev_buffer = Buffer::new(terminal_width, terminal_height);
        self.terminal_size = Size::new(terminal_width, terminal_height);
    }
}
