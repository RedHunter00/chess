use super::*;
use crate::board::Board;
use crate::core::direction::Direction;
use crate::core::mov::Move;

impl Piece {
    pub fn get_all_legal_moves_knight(&self, board: &Board) -> Vec<crate::core::mov::Move> {
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
