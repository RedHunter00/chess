/// This module contains the King struct and its implementation of the Piece trait.

use super::piece::Piece;
use crate::board::Board;
use crate::positions::mov::Move;
use crate::positions::position::{Direction, Position};
use crate::utilities::Color;

pub struct King {
    position: Position,
    color: Color,
    piece_type: String,
}

impl King {
    pub fn new(position: Position, color: Color) -> King {
        King {
            position,
            color,
            piece_type: String::from("King"),
        }
    }
}

impl Piece for King {
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_piece_type(&self) -> String {
        self.piece_type.clone()
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn get_all_legal_moves(&self, board: &Board) -> Vec<Move> {
        let start = self.position;
        let directions = vec![
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
        ];
        let mut moves = Vec::new();

        for direction in directions {
            if let Some(target) = start.increment(direction, 1) {
                if let Some(piece) = board.get_piece(target) {
                    if piece.get_color() != self.get_color() {
                        let mov = Move::Normal {
                            from: start,
                            to: target,
                        };
                        moves.push(mov);
                    }
                } else {
                    let mov = Move::Normal {
                        from: start,
                        to: target,
                    };
                    moves.push(mov);
                }
            }
        }

        moves
    }
}
