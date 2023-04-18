/// This module contains the Knight struct and its implementation of the Piece trait.

use super::piece::Piece;
use crate::board::Board;
use crate::positions::mov::Move;
use crate::positions::position::{Direction, Position};
use crate::utilities::Color;

pub struct Knight {
    position: Position,
    color: Color,
    piece_type: String,
}

impl Knight {
    pub fn new(position: Position, color: Color) -> Knight {
        Knight {
            position,
            color,
            piece_type: String::from("Knight"),
        }
    }
}

impl Piece for Knight {
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

    fn get_all_legal_moves(&self, board: &Board) -> Vec<crate::positions::mov::Move> {
        let start = self.position;
        let mut moves = Vec::new();

        if let Some(aux) = start.increment(Direction::Right, 2) {
            if let Some(target) = aux.increment(Direction::Up, 1) {
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

        if let Some(aux) = start.increment(Direction::Right, 2) {
            if let Some(target) = aux.increment(Direction::Down, 1) {
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

        if let Some(aux) = start.increment(Direction::Left, 2) {
            if let Some(target) = aux.increment(Direction::Up, 1) {
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

        if let Some(aux) = start.increment(Direction::Left, 2) {
            if let Some(target) = aux.increment(Direction::Down, 1) {
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

        if let Some(aux) = start.increment(Direction::Up, 2) {
            if let Some(target) = aux.increment(Direction::Right, 1) {
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

        if let Some(aux) = start.increment(Direction::Up, 2) {
            if let Some(target) = aux.increment(Direction::Left, 1) {
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

        if let Some(aux) = start.increment(Direction::Down, 2) {
            if let Some(target) = aux.increment(Direction::Right, 1) {
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

        if let Some(aux) = start.increment(Direction::Down, 2) {
            if let Some(target) = aux.increment(Direction::Left, 1) {
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
