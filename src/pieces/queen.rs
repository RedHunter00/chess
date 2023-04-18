/// This module contains the Queen struct and its implementation of the Piece trait.

use super::piece::Piece;
use crate::board::Board;
use crate::positions::mov::Move;
use crate::positions::position::{Direction, Position};
use crate::utilities::Color;

pub struct Queen {
    position: Position,
    color: Color,
    piece_type: String,
}

impl Queen {
    pub fn new(position: Position, color: Color) -> Queen {
        Queen { position, color, piece_type: String::from("Queen") }
    }
}

impl Piece for Queen {
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
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownRight,
            Direction::DownLeft,
        ];
        let mut moves = Vec::new();

        for direction in directions {
            let mut i = 1;
            while let Some(target) = start.increment(direction, i) {
                if let Some(piece) = board.get_piece(target) {
                    if piece.get_color() != self.get_color() {
                        let mov = Move::Normal { from: start, to: target };
                        moves.push(mov);
                    }
                    break;
                }

                let mov = Move::Normal { from: start, to: target };
                moves.push(mov);

                i += 1;
            }
        }

        moves
    }
}
