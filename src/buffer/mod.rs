pub mod cell;
pub mod color;
pub mod size;

pub use cell::*;
pub use color::*;
pub use size::*;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Buffer {
    pub size: Size,
    pub cells: Vec<Cell>,
}

impl Buffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            size: Size { width, height },
            cells: vec![Cell::default(); (height * width) as usize],
        }
    }

    pub fn get_mut(&mut self, x: u16, y: u16) -> &mut Cell {
        let index = self.point_to_index(x, y, self.size.width);
        &mut self.cells[index as usize]
    }

    pub fn set_char_at(
        &mut self,
        x: u16,
        y: u16,
        ch: char,
        fg: Color,
        bg: Option<Color>,
        modifier: Modifier,
    ) {
        let cell = self
            .get_mut(x, y)
            .set_symbol(ch.to_string().as_ref())
            .set_fg(fg)
            .set_modifier(modifier);

        bg.map(|bg| cell.set_bg(bg));
    }

    pub fn set_string_at(
        &mut self,
        x: u16,
        y: u16,
        max_width: u16,
        s: &str,
        fg: Color,
        bg: Option<Color>,
        modifier: Modifier,
    ) {
        let width = self.size.width;
        let mut index = self.point_to_index(x, y, width) as usize;
        let end_index = index + max_width as usize;

        for grapheme in s.graphemes(true) {
            if index >= end_index {
                break;
            }
            let cell = &mut self.cells[index];
            cell.set_symbol(grapheme).set_fg(fg).set_modifier(modifier);
            if let Some(bg_color) = bg {
                cell.set_bg(bg_color);
            }
            index += 1;
        }
    }

    /// Returns a string with escape sequences and ANSI color codes.
    pub fn as_str(&self) -> String {
        self.cells
            .chunks(self.size.width as usize)
            .map(|row| row.iter().map(|cell| cell.to_string()).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Returns a string with no escape sequences or ANSI color codes.
    pub fn as_plain_str(&self) -> String {
        self.cells
            .chunks(self.size.width as usize)
            .map(|row| {
                row.iter()
                    .map(|cell| cell.symbol.clone())
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = (u16, u16, &Cell)> {
        let width = self.size.width;
        let mut index = 0;
        self.cells.iter().map(move |cell| {
            let x = (index % width as usize) as u16;
            let y = (index / width as usize) as u16;
            index += 1;
            (x, y, cell)
        })
    }

    #[inline]
    fn index_to_point(&self, index: u16, width: u16) -> (u16, u16) {
        (index % width, index / width)
    }

    #[inline]
    fn point_to_index(&self, x: u16, y: u16, width: u16) -> u16 {
        y * width + x
    }

    pub fn clear(&mut self) {
        self.cells.iter_mut().for_each(|cell| cell.reset());
    }

    pub fn clear_line(&mut self, y: u16) {
        for x in 0..self.size.width {
            self.set_char_at(
                x,
                y,
                '¬',
                Color::Reset,
                Some(Color::Reset),
                Modifier::empty(),
            );
        }
    }

    pub fn diff(&self, previous: &Self) -> Vec<(u16, u16, &Cell)> {
        let mut updates = Vec::with_capacity(32);

        for (i, (current, previous)) in self.cells.iter().zip(&previous.cells).enumerate() {
            if current != previous {
                let (x, y) = self.index_to_point(i as u16, self.size.width);
                updates.push((x, y, current));
            }
        }

        updates
    }
}
