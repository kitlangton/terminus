use std::io::{self, Write};

use crate::{
    buffer::{Buffer, Modifier, Rect, Size},
    Color,
};
use crossterm::{
    cursor::{MoveTo, MoveUp},
    style::{Print, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ScrollUp},
    *,
};

use crate::{RenderContext, View};

pub(crate) struct Renderer<W: Write> {
    writer: W,
    current_buffer: Buffer,
    prev_buffer: Buffer,
    view_height: u16,
    claimed_height: u16,
    terminal_size: Size,
}

impl<W: Write> Renderer<W> {
    pub(crate) fn new(writer: W) -> Self {
        let (terminal_width, terminal_height) = crossterm::terminal::size().unwrap();
        let terminal_size = Size::new(terminal_width, terminal_height);
        let (_, cursor_y) = crossterm::cursor::position().unwrap();
        let claimed_height = terminal_height.saturating_sub(cursor_y);
        Self {
            writer,
            current_buffer: Buffer::new(terminal_width, terminal_height),
            prev_buffer: Buffer::new(terminal_width, terminal_height),
            view_height: 0,
            claimed_height,
            terminal_size,
        }
    }

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

        self.claim_space(view_height.max(self.claimed_height), self.terminal_size.height);

        let start_y = self.terminal_size.height.saturating_sub(self.claimed_height).max(0);
        let rect = Rect::new(0, start_y, view_width, view_height);
        view.render(RenderContext::new(rect), &mut self.current_buffer);
        self.view_height = view_height;
        self.print_buffer().unwrap();
    }

    pub(crate) fn move_cursor_to_bottom_of_current_view(&mut self) {
        let target = self.terminal_size.height.saturating_sub(self.claimed_height) + self.view_height;
        queue!(self.writer, MoveTo(0, target), Print("\n")).unwrap();
    }

    /// Swaps the current buffer with the previous buffer.
    fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.current_buffer, &mut self.prev_buffer);
    }

    /// Ensures that the terminal has enough space to render the view by adding new lines if necessary.
    fn claim_space(&mut self, view_height: u16, terminal_height: u16) {
        let diff = view_height.saturating_sub(self.claimed_height);
        if diff > 0 {
            queue!(self.writer, ScrollUp(diff)).unwrap();
            self.claimed_height += diff;

            // clear each line
            for y in terminal_height.saturating_sub(view_height)..terminal_height {
                self.prev_buffer.clear_line(y);
            }
        }
    }

    fn print_buffer(&mut self) -> io::Result<()> {
        let diff = self.current_buffer.diff(&self.prev_buffer);
        let mut last_fg = Color::Reset;
        let mut last_bg = Color::Reset;
        let mut last_modifier = Modifier::empty();
        let mut last_x = u16::MAX;
        let mut last_y = u16::MAX;

        for (x, y, cell) in diff {
            let modifier = cell.modifier;

            if cell.fg != last_fg {
                queue!(self.writer, SetForegroundColor(cell.fg.into()))?;
                last_fg = cell.fg;
            }

            if cell.bg != last_bg {
                queue!(self.writer, SetBackgroundColor(cell.bg.into()))?;
                last_bg = cell.bg;
            }

            if modifier != last_modifier {
                Modifier::write_diff(modifier, last_modifier, &mut self.writer);
                last_modifier = modifier;
            }

            if last_x + 1 != x || last_y != y {
                queue!(self.writer, MoveTo(x, y))?;
            }
            queue!(self.writer, Print(&cell.symbol))?;

            last_x = x;
            last_y = y;
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
        queue!(
            self.writer,
            MoveUp(self.claimed_height),
            Clear(terminal::ClearType::FromCursorDown)
        )
        .unwrap();

        self.current_buffer = Buffer::new(terminal_width, terminal_height);
        self.prev_buffer = Buffer::new(terminal_width, terminal_height);
        self.terminal_size = Size::new(terminal_width, terminal_height);
    }
}
