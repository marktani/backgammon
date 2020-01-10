# backgammon-rust

A CLI backgammon game written in Rust.

## Commands

* new: start a new game
* move x, x between (1, 24): move token on point x
* status: print status

## Ideas

### Play vs. computer

* Employ different strategies:
  * "hit"
  * "build defense"
  * "avoid danger"
  * "rush"
  * "move random"
* These could be shifted over time, for example:
  * "Rush in the early game, defend in the mid game, move random in the late game"
* Time periods need to be loosely defined, some ideas:
  * "early game - no direct contact yet"
  * "mid game - "
  * pip count or difference -> advantage: more closely to the up-and-down nature of the game

### Highlight possible moves

* For games against the computer, we could highlight the possible moves to help the player.
