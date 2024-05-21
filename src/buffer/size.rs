#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn zero() -> Point {
        Point { x: 0, y: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub point: Point,
    pub size: Size,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            point: Point { x, y },
            size: Size { width, height },
        }
    }

    pub fn left(&self) -> u16 {
        self.point.x
    }

    pub fn top(&self) -> u16 {
        self.point.y
    }

    pub fn right(&self) -> u16 {
        self.left() + self.size.width
    }

    pub fn bottom(&self) -> u16 {
        self.top() + self.size.height
    }

    pub fn inset_by(self, inset_left: u16, inset_right: u16, inset_top: u16, inset_bottom: u16) -> Rect {
        Rect {
            point: Point {
                x: self.point.x + inset_left,
                y: self.point.y + inset_top,
            },
            size: self.size.inset_by(inset_left, inset_right, inset_top, inset_bottom),
        }
    }

    pub fn outset_by(self, outset_left: u16, outset_right: u16, outset_top: u16, outset_bottom: u16) -> Rect {
        Rect {
            point: Point {
                x: self.point.x - outset_left,
                y: self.point.y - outset_top,
            },
            size: self
                .size
                .outset_by(outset_left, outset_right, outset_top, outset_bottom),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    /// Creates a new size with the given width and height.
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    /// Creates a new size with the width and height set to 0.
    pub fn zero() -> Size {
        Size { width: 0, height: 0 }
    }

    /// Creates a new size with the width and height set to the maximum value of `u16`.
    pub fn max() -> Size {
        Size {
            width: u16::MAX,
            height: u16::MAX,
        }
    }

    /// Reduces the size by the given inset on all sides.
    ///
    /// # Example
    /// ```
    /// use terminus::buffer::Size;
    /// let size = Size { width: 10, height: 8 };
    /// let inset_size = size.inset_by(1, 1, 1, 1);
    /// assert_eq!(inset_size, Size { width: 8, height: 6 });
    /// ```
    pub fn inset_by(self, left: u16, right: u16, top: u16, bottom: u16) -> Size {
        Size {
            width: self.width - left - right,
            height: self.height - top - bottom,
        }
    }

    /// Increases the size by the given outset on all sides.
    ///
    /// # Example
    /// ```
    /// use terminus::buffer::Size;
    /// let size = Size { width: 10, height: 8 };
    /// let outset_size = size.outset_by(1, 1, 1, 1);
    /// assert_eq!(outset_size, Size { width: 12, height: 10 });
    /// ```
    pub fn outset_by(self, left: u16, right: u16, top: u16, bottom: u16) -> Size {
        Size {
            width: self.width + left + right,
            height: self.height + top + bottom,
        }
    }

    /// Returns a new size with the smaller dimensions of the two sizes.
    ///
    /// # Example
    /// ```
    /// use terminus::buffer::Size;
    /// let size = Size { width: 10, height: 8 };
    /// let proposed_size = Size { width: 12, height: 10 };
    /// let min_size = size.min(proposed_size);
    /// assert_eq!(min_size, Size { width: 10, height: 8 });
    /// ```
    pub fn min(&self, proposed: Size) -> Size {
        // TODO: Is this name weird?
        Size {
            width: self.width.min(proposed.width),
            height: self.height.min(proposed.height),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_inset_by() {
        let rect = Rect::new(10, 10, 30, 20);
        let inset_rect = rect.inset_by(5, 5, 5, 5);
        assert_eq!(inset_rect.point.x, 15);
        assert_eq!(inset_rect.point.y, 15);
        assert_eq!(inset_rect.size.width, 20);
        assert_eq!(inset_rect.size.height, 10);
    }

    #[test]
    fn test_rect_outset_by() {
        let rect = Rect::new(10, 10, 30, 20);
        let outset_rect = rect.outset_by(5, 5, 5, 5);
        assert_eq!(outset_rect.point.x, 5);
        assert_eq!(outset_rect.point.y, 5);
        assert_eq!(outset_rect.size.width, 40);
        assert_eq!(outset_rect.size.height, 30);
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_inset_outset_identity(width in 50..100u16, height in 50..100u16, left in 0..25u16, right in 0..25u16, top in 0..25u16, bottom in 0..25u16) {
            let size = Size { width, height };
            let inset_size = size.inset_by(left, right, top, bottom);
            let outset_size = inset_size.outset_by(left, right, top, bottom);
            assert_eq!(size, outset_size);
        }

        #[test]
        fn test_outset_inset_identity(width in 50..100u16, height in 50..100u16, left in 0..25u16, right in 0..25u16, top in 0..25u16, bottom in 0..25u16) {
            let size = Size { width, height };
            let outset_size = size.outset_by(left, right, top, bottom);
            let inset_size = outset_size.inset_by(left, right, top, bottom);
            assert_eq!(size, inset_size);
        }
    }
}
