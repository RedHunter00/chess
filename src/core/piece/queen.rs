use super::*;
use crate::board::Board;
use crate::core::direction::Direction;
use crate::core::mov::Move;

impl Piece {
    pub fn get_all_legal_moves_queen(&self, board: &Board) -> Vec<Move> {
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
                        let mov = Move::Normal {
                            from: start,
                            to: target,
                        };
                        moves.push(mov);
                    }
                    break;
                }

                let mov = Move::Normal {
                    from: start,
                    to: target,
                };
                moves.push(mov);

                i += 1;
            }
        }

        moves
    }
}
