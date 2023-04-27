use super::*;
use crate::board::Board;
use crate::core::castles::Castles;
use crate::core::color::Color;
use crate::core::direction::Direction;
use crate::core::mov::Move;

impl Piece {
    pub fn get_all_legal_moves_king(&self, board: &Board) -> Vec<Move> {
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

        match self.get_color() {
            Color::White => {
                if board.get_modifiers().can_white_castle_kingside {
                    if let None =
                        board.get_piece(self.get_position().increment(Direction::Right, 1).unwrap())
                    {
                        if let None = board
                            .get_piece(self.get_position().increment(Direction::Right, 2).unwrap())
                        {
                            let mov = Move::Castle {
                                color: Color::White,
                                castle_type: Castles::KingSide,
                            };
                            moves.push(mov);
                        }
                    };
                }
                if board.get_modifiers().can_white_castle_queenside {
                    if let None =
                        board.get_piece(self.get_position().increment(Direction::Left, 1).unwrap())
                    {
                        if let None = board
                            .get_piece(self.get_position().increment(Direction::Left, 2).unwrap())
                        {
                            if let None = board.get_piece(
                                self.get_position().increment(Direction::Left, 3).unwrap(),
                            ) {
                                let mov = Move::Castle {
                                    color: Color::White,
                                    castle_type: Castles::QueenSide,
                                };
                                moves.push(mov);
                            }
                        }
                    };
                }
            }
            Color::Black => {
                if board.get_modifiers().can_black_castle_kingside {
                    if let None =
                        board.get_piece(self.get_position().increment(Direction::Right, 1).unwrap())
                    {
                        if let None = board
                            .get_piece(self.get_position().increment(Direction::Right, 2).unwrap())
                        {
                            let mov = Move::Castle {
                                color: Color::Black,
                                castle_type: Castles::KingSide,
                            };
                            moves.push(mov);
                        }
                    };
                }
                if board.get_modifiers().can_black_castle_queenside {
                    if let None =
                        board.get_piece(self.get_position().increment(Direction::Left, 1).unwrap())
                    {
                        if let None = board
                            .get_piece(self.get_position().increment(Direction::Left, 2).unwrap())
                        {
                            if let None = board.get_piece(
                                self.get_position().increment(Direction::Left, 3).unwrap(),
                            ) {
                                let mov = Move::Castle {
                                    color: Color::Black,
                                    castle_type: Castles::QueenSide,
                                };
                                moves.push(mov);
                            }
                        }
                    };
                }
            }
        }

        moves
    }
}
