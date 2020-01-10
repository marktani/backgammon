use rand::{thread_rng, Rng};
use std::cmp::{max, min};

pub struct Roll(pub u8, pub u8);

impl Roll {
    /// Generates a randomized roll.
    ///
    /// ```
    /// use backgammon::{Roll};
    ///
    /// let roll = Roll::roll();
    /// assert!((roll.0 < 7) & (roll.0 >= 1) & (roll.1 < 7) & (roll.1 >= 1));
    /// ```
    pub fn roll() -> Roll {
        Roll(Roll::roll_die(), Roll::roll_die())
    }

    /// Generates a randomized roll that is no double.
    ///
    /// ```
    /// use backgammon::{Roll};
    ///
    /// let roll = Roll::opening_roll();
    /// assert_ne!(roll.0, roll.1);
    /// ```
    pub fn opening_roll() -> Roll {
        let mut roll = Roll::roll();

        while roll.is_double() {
            roll = Roll::roll();
        }
        roll
    }

    /// Whether a roll is a double.
    ///
    /// ```
    /// use backgammon::{Roll};
    ///
    /// let roll = Roll(1, 1);
    /// let roll2 = Roll(1, 2);
    /// assert!(roll.is_double());
    /// assert!(!roll2.is_double());
    /// ```
    pub fn is_double(&self) -> bool {
        self.0 == self.1
    }

    /// Returns the higher die. Returns either die if it is a double roll.
    ///
    /// ```
    /// use backgammon::{Roll};
    ///
    /// let roll = Roll(1, 2);
    /// let double = Roll(2, 2);
    /// let roll2 = Roll(3, 2);
    ///
    /// assert_eq!(roll.get_higher(), 2);
    /// assert_eq!(double.get_higher(), 2);
    /// assert_eq!(roll2.get_higher(), 3);
    /// ```
    pub fn get_higher(&self) -> u8 {
        max(self.0, self.1)
    }

    /// Returns the lower die. Returns either die if it is a double roll.
    ///
    /// ```
    /// use backgammon::{Roll};
    ///
    /// let roll = Roll(1, 2);
    /// let double = Roll(2, 2);
    /// let roll2 = Roll(3, 2);
    ///
    /// assert_eq!(roll.get_lower(), 1);
    /// assert_eq!(double.get_lower(), 2);
    /// assert_eq!(roll2.get_lower(), 2);
    /// ```
    pub fn get_lower(&self) -> u8 {
        min(self.0, self.1)
    }

    fn roll_die() -> u8 {
        let mut rng = thread_rng();
        rng.gen_range(1, 7)
    }
}
