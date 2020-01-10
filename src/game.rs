use crate::board::Board;
use crate::color::Color;

/// Contains all actions supported within the game
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Eq)]
pub enum Action {
    // MakeMove(BackgammonMove),
    MakeMove,
    // OfferDraw(Color),
    // AcceptDraw,
    // DeclareDraw,
    GiveUp(Color),
}

/// What was the result of this game?
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum GameResult {
    WhiteWins,
    WhiteGivesUp,
    BlackWins,
    BlackGivesUp,
}

/// Represent a token on the backgammon board
#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Debug, Hash)]
pub struct Game {
    board: Board,
}
