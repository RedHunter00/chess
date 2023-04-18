// This module contains the Position struct

use std::fmt;

//? adding UpUpLeft, UpUpRight, DownDownLeft, DownDownRight, LeftLeftUp, LeftLeftDown, RightRightUp, RightRightDown
//? for the knight might be a good idea
/// Core enum that represents the direction of a move
/// Used in the Position struct to get a relative position from a given position
/// in a specific direction
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

/// A core struct that represents a position on the board.
/// The position is represented by a tuple of two u8s.
/// The first u8 represents the x coordinate, the second u8 represents the y coordinate.
/// The x coordinate is represented by the file, the y coordinate is represented by the rank.
/// 
/// # Examples
/// a1 is represented by (0, 0)
/// e2 is represented by (4, 1)
/// h8 is represented by (7, 7)
/// 
/// For ease of use the Position struct has a method that allows you to create a Position struct from a file and rank.
/// The the x,y position should only be used by the board and pieces internally
/// # Examples
/// 
/// ```
/// use chess::positions::position::Position;
/// 
/// let position = Position::from_an('a', 1);
/// assert_eq!(position, Position::new(0, 0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: u8,
    y: u8,
}

impl Position {

    /// Creates a new Position struct from a x and y coordinate
    /// Use of this method is discouraged, use Position::from_an instead
    /// for ease of use
    pub fn new(x: u8, y: u8) -> Position {
        Position { x, y }
    }

    /// Creates a new Position struct from a file and rank
    /// Use this method for ease of use
    pub fn from_an(file: char, rank: u8) -> Position{
        Position::new((file as u8) - 97, rank - 1, )
    }

    /// Returns the x coordinate of the position
    pub fn get_x(&self) -> u8 {
        self.x
    }

    /// Returns the y coordinate of the position
    pub fn get_y(&self) -> u8 {
        self.y
    }

    /// Return an Option of a new Position struct that is n squares away in the given direction
    /// If the new position is off the board, None is returned
    pub fn increment(&self, direction: Direction, n: u8) -> Option<Position> {
        match direction {
            Direction::Right => {
                if self.x + n > 7 {
                    None
                } else {
                    Some(Position::new(self.x + n, self.y))
                }
            }
            Direction::Left => {
                if self.x < n {
                    None
                } else {
                    Some(Position::new(self.x - n, self.y))
                }
            }
            Direction::Up => {
                if self.y < n {
                    None
                } else {
                    Some(Position::new(self.x, self.y - n))
                }
            }
            Direction::Down => {
                if self.y + n > 7 {
                    None
                } else {
                    Some(Position::new(self.x, self.y + n))
                }
            }
            Direction::UpRight => {
                if self.x + n > 7 || self.y < n {
                    None
                } else {
                    Some(Position::new(self.x + n, self.y - n))
                }
            }
            Direction::UpLeft => {
                if self.x < n || self.y < n {
                    None
                } else {
                    Some(Position::new(self.x - n, self.y - n))
                }
            }
            Direction::DownRight => {
                if self.x + n > 7 || self.y + n > 7 {
                    None
                } else {
                    Some(Position::new(self.x + n, self.y + n))
                }
            }
            Direction::DownLeft => {
                if self.x < n || self.y + n > 7 {
                    None
                } else {
                    Some(Position::new(self.x - n, self.y + n))
                }
            }
        }
    }

    //? might be unnecessary as increment works better
    /// Returns a vector of all positions in the given direction
    pub fn get_all_rel_pos(&self, direction: Direction) -> Vec<Position> {
        let mut moves = Vec::new();
        let mut pos = self.increment(direction, 1);
        while let Some(p) = pos {
            moves.push(p);
            pos = p.increment(direction, 1);
        }
        moves
    }
}

/// Allows for the Position struct to be displayed in two different ways
/// In the normal mode, the position is displayed as an algebraic notation
/// In the debug mode, the position is displayed as a tuple of the x and y coordinates
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Debug mode, display as tuple
            write!(f, "x:{}, y:{}", self.x, self.y)
        } else {
            // Normal mode, display as algebraic notation
            let file = (self.x + 97) as char;
            let rank = self.y + 1;
            write!(f, "{}{}", file, rank)
        }
    }
}