/// This module contains the Pawn struct and its implementation of the Piece trait.

use super::piece::Piece;
use crate::board::Board;
use crate::positions::mov::Move;
use crate::positions::position::{Direction, Position};
use crate::utilities::Color;

pub struct Pawn {
    position: Position,
    color: Color,
    piece_type: String,
}

impl Pawn {
    pub fn new(position: Position, color: Color) -> Pawn {
        Pawn {
            position,
            color,
            piece_type: String::from("Pawn"),
        }
    }
}

impl Piece for Pawn {
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
        let mut moves = Vec::new();

        let vertical;
        let diag1;
        let diag2;
        let y;

        if self.get_color() == Color::White {
            vertical = Direction::Down;
            diag1 = Direction::DownRight;
            diag2 = Direction::DownLeft;
            y = 1;
        } else {
            vertical = Direction::Up;
            diag1 = Direction::UpRight;
            diag2 = Direction::UpLeft;
            y = 6;
        }

        if let Some(target) = start.increment(diag1, 1) {
            if let Some(piece) = board.get_piece(target) {
                if piece.get_color() != self.get_color() {
                    let mov = Move::Normal {
                        from: start,
                        to: target,
                    };
                    moves.push(mov);
                }
            }
        }

        if let Some(target) = start.increment(diag2, 1) {
            if let Some(piece) = board.get_piece(target) {
                if piece.get_color() != self.get_color() {
                    let mov = Move::Normal {
                        from: start,
                        to: target,
                    };
                    moves.push(mov);
                }
            }
        }

        println!("{}", start.increment(vertical, 1).unwrap().to_string());
        if let Some(target) = start.increment(vertical, 1) {
            if let None = board.get_piece(target) {
                let mov = Move::Normal {
                    from: start,
                    to: target,
                };
                moves.push(mov);
            } else {
                return moves;
            }
        }

        println!("{} {}", y, self.position.get_y());
        println!("{:?}", start.increment(vertical, 3));
        if y == self.position.get_y() {
            if let Some(target) = start.increment(vertical, 2) {
                println!("fffff");
                if let None = board.get_piece(target) {
                    let mov = Move::Normal {
                        from: start,
                        to: target,
                    };
                    moves.push(mov);
                } else {
                    println!("{:?}", target);
                }
            }
        }

        moves
    }
}
