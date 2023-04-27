mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

use super::color::Color;
use super::mov::Move;
use super::pieces::Pieces;
use super::position::Position;
use crate::board::Board;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    position: Position,
    color: Color,
    piece_type: Pieces,
    value: u8,
}

impl Piece {
    pub fn new(position: Position, color: Color, piece_type: Pieces) -> Piece {
        let value = match piece_type {
            Pieces::King => 0,
            Pieces::Queen => 9,
            Pieces::Rook => 5,
            Pieces::Bishop => 3,
            Pieces::Knight => 3,
            Pieces::Pawn => 1,
        };

        Piece {
            position,
            color,
            piece_type,
            value,
        }
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_piece_type(&self) -> Pieces {
        self.piece_type
    }

    pub fn get_all_legal_moves(&self, board: &Board) -> Vec<Move> {
        match self.piece_type {
            Pieces::King => self.get_all_legal_moves_king(board),
            Pieces::Queen => self.get_all_legal_moves_queen(board),
            Pieces::Rook => self.get_all_legal_moves_rook(board),
            Pieces::Bishop => self.get_all_legal_moves_bishop(board),
            Pieces::Knight => self.get_all_legal_moves_knight(board),
            Pieces::Pawn => self.get_all_legal_moves_pawn(board),
        }
    }
}
