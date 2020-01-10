use crate::color::Color;

/// Represent a point on the backgammon board
#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Debug, Hash)]
pub struct Point {
    pub point: u8,
    pub color: Color,
}

/// How many points are there?
pub const NUM_POINTS: usize = 24;

// impl Default for Point {
//     /// Create a point on white 1.
//     ///
//     /// ```
//     /// use backgammon::{Point, Color};
//     ///
//     /// let explicit_sq = Point::make_point(1, Color::White);
//     /// let implicit_sq = Point::default();
//     ///
//     /// assert_eq!(explicit_sq, implicit_sq);
//     /// ```
//     fn default() -> Point {
//         unsafe { Point::new{point: 1, color: Color::White} }
//     }
// }

impl Point {
    /// Creates a new point with a specified color.
    ///
    /// ```
    /// use backgammon::{Point, Color};
    ///
    /// let p = Point::new(1, Color::White);
    /// let q = Point::new(13, Color::Black);
    ///
    /// assert_eq!(p.point, 1);
    /// assert_eq!(q.point, 13);
    /// ```
    pub fn new(point: u8, color: Color) -> Point {
        Point {
            point: point,
            color: color,
        }
    }

    /// Transforms this point to a new point with the specified color.
    ///
    /// ```
    /// use backgammon::{Point, Color};
    ///
    /// let p = Point::new(1, Color::White).convert_by_color(Color::Black);
    /// let q = Point::new(13, Color::Black).convert_by_color(Color::White);
    ///
    /// assert_eq!(p.point, 24);
    /// assert_eq!(q.point, 12);
    /// ```
    pub fn convert_by_color(&self, color: Color) -> Point {
        if color == !self.color {
            Point {
                point: NUM_POINTS as u8 + 1 - self.point,
                color: color,
            }
        } else {
            *self
        }
    }

    pub fn flip_color(&mut self) -> () {
        let point = self.convert_by_color(!self.color);
        self.color = point.color;
        self.point = point.point;
    }

    /// Generates an index based on this point.
    /// The first point (index 0) is 1 for white and 24 for black.
    ///
    /// ```
    /// use backgammon::{Point, Color};
    ///
    /// let p = Point::new(1, Color::White);
    /// let q = Point::new(24, Color::Black);
    /// let r = Point::new(12, Color::White);
    /// let s = Point::new(13, Color::Black);
    ///
    /// assert_eq!(p.to_index(), 0);
    /// assert_eq!(q.to_index(), 0);
    /// assert_eq!(r.to_index(), 11);
    /// assert_eq!(s.to_index(), 11);
    /// ```
    pub fn to_index(&self) -> usize {
        if self.color == Color::White {
            (self.point - 1) as usize
        } else {
            NUM_POINTS - (self.point as usize)
        }
    }
}
