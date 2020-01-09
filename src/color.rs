use std::ops::Not;

/// Represent a color.
#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Color {
    White,
    Black,
}

/// How many colors are there?
pub const NUM_COLORS: usize = 2;

/// List all colors
pub const ALL_COLORS: [Color; NUM_COLORS] = [Color::White, Color::Black];

impl Not for Color {
    type Output = Color;

    /// Get the other color.
    #[inline]
    fn not(self) -> Color {
        if self == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }
}
