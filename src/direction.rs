#[derive(Clone, Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    pub fn next(&self) -> Direction {
        match self {
            Direction::Horizontal => Direction::Vertical,
            Direction::Vertical => Direction::Horizontal,
        }
    }
}
