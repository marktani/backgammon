use crate::color::Color;

/// Represent a token on the backgammon board
#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Debug, Hash)]
pub struct Token {
    color: Color,
}
