use crate::board::Board;

/// Contains all actions supported within the game
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Eq)]
pub enum Action {
    RollDie,
    RollDice,

    MoveToken,
    SetToken,
    BearOffToken,

    Wait,

    OfferRaise,
    AcceptRaise,

    GiveUp,
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
#[derive(Clone, Debug, Hash)]
pub struct Game {
    board: Board,
}
