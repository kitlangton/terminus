pub mod cell;
pub mod color;
pub mod size;

pub use cell::*;
pub use color::*;
pub use size::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Buffer {
    pub size: Size,
    pub cells: Vec<Cell>,
}

impl Buffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            size: Size { width, height },
            cells: vec![Cell::new(" "); (height * width) as usize],
        }
    }

    pub fn get_mut(&mut self, x: u16, y: u16) -> &mut Cell {
        let index = self.point_to_index(x, y, self.size.width);
        &mut self.cells[index as usize]
    }

    pub fn set_char_at(&mut self, x: u16, y: u16, ch: char, fg: Color, bg: Color, modifier: Modifier) {
        let index = self.point_to_index(x, y, self.size.width);
        if index < self.cells.len() as u16 {
            self.cells[index as usize] = Cell {
                symbol: ch.to_string().into(),
                fg,
                bg,
                modifier,
            };
        }
    }

    pub fn as_str(&self) -> String {
        self.cells
            .chunks(self.size.width as usize)
            .map(|row| {
                row.iter()
                    .map(|cell| cell.to_ansi_code())
                    .collect::<Vec<String>>()
                    .join("")
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

    fn index_to_point(&self, index: u16, width: u16) -> Point {
        Point {
            x: index % width,
            y: index / width,
        }
    }

    fn point_to_index(&self, x: u16, y: u16, width: u16) -> u16 {
        y * width + x
    }

    pub fn clear(&mut self) {
        self.cells = vec![Cell::new(" "); (self.size.height * self.size.width) as usize];
    }

    pub fn clear_line(&mut self, y: u16) {
        for x in 0..self.size.width {
            self.set_char_at(x, y, '¬', Color::Reset, Color::Reset, Modifier::empty());
        }
    }

    pub fn diff(&self, previous: &Self) -> Vec<(u16, u16, &Cell)> {
        let mut updates = Vec::with_capacity(32);

        for (i, (current, previous)) in self.cells.iter().zip(&previous.cells).enumerate() {
            if current != previous {
                let Point { x, y } = self.index_to_point(i as u16, self.size.width);
                updates.push((x, y, current));
            }
        }

        updates
    }
}
