use crate::color::Color;

/// Represent a point on the backgammon board
#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Debug, Hash)]
pub struct Point {
    point: u8,
    color: Color,
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
    pub fn new(point: u8, color: Color) -> Point {
        Point {
            point: point,
            color: color,
        }
    }

    pub fn convert_by_color(&self, color: Color) -> Point {
        if color == !self.color {
            Point {
                point: 25 - self.point,
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
}
