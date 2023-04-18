/// This module contains the Move struct and its related enums.

use super::position::Position;

// ! Not actually used yet, but will be used to represent a promotion
// ! Also should be used by every piece insted of a string to represent it's type
// ! Should be moved out of this file
#[derive(PartialEq)]
pub enum PieceTypes {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// Represents a move on the board
/// It has 3 variants:
/// Normal: a normal move
/// Promotion: a move that promotes a pawn
/// Castle: a move that involves castleing a king
#[derive(PartialEq)]
pub enum Move {
    Normal { from: Position, to: Position },
    Promotion { from: Position, to: Position, promotion: PieceTypes },
    Castle { king_from: Position, king_to: Position, rook_from: Position, rook_to: Position },
}