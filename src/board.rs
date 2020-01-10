use crate::color::Color;
use crate::token::Token;

/// Represent a point on the backgammon board
#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Debug, Hash)]
pub struct Board {
    tokens: [Token; 30],
    side_to_move: Color,
}

impl Board {
    /// Who's turn is it?
    ///
    #[inline]
    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }
}
