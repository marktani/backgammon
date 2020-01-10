use crate::color::Color;
// use crate::token::Token;
use crate::point::{Point, ALL_POINTS};

/// Represent a backgammon board
#[derive(Clone, Debug, Hash)]
pub struct Board {
    points: Vec<(Point, u8, Option<Color>)>,
    side_to_move: Option<Color>,
}

impl Default for Board {
    fn default() -> Board {
        let points = ALL_POINTS;
        let mut np = Vec::new();
        for p in &points {
            let q = p.convert_by_color(Color::White);
            let input: (u8, Option<Color>) = match q.point {
                24 => {
                    (2, Some(Color::Black))
                },
                19 => {
                    (5, Some(Color::White))
                },
                17 => {
                    (3, Some(Color::White))
                },
                13 => {
                    (5, Some(Color::Black))
                },
                12 => {
                    (5, Some(Color::White))
                },
                8 => {
                    (3, Some(Color::Black))
                },
                6 => {
                    (5, Some(Color::Black))
                },
                1 => {
                    (2, Some(Color::White))
                },
                _ => {
                    (0, None)
                }
            };
            np.push((q, input.0, input.1));
        }

        Board {
            points: np,
            side_to_move: None,
        }
    }
}

impl Board {
    /// Whose turn is it?
    ///
    /// If it returns `None`, both colors first need to roll a die to decide who starts.
    #[inline]
    pub fn side_to_move(&self) -> Option<Color> {
        self.side_to_move
    }

}
