/// This module contains the Move struct and its related enums.
use super::position::Position;
use crate::utilities::Color;

// ! Not actually used yet, but will be used to represent a promotion
// ! Also should be used by every piece insted of a string to represent it's type
// ! Should be moved out of this file
#[derive(Clone, Copy,PartialEq)]
pub enum PieceTypes {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CastleTypes {
    KingSide,
    QueenSide,
}

/// Represents a move on the board
/// It has 3 variants:
/// Normal: a normal move
/// Promotion: a move that promotes a pawn
/// Castle: a move that involves castleing a king
#[derive(PartialEq)]
pub enum Move {
    Normal {
        from: Position,
        to: Position,
    },
    Promotion {
        from: Position,
        to: Position,
        promotion: PieceTypes,
    },
    Castle {
        color: Color,
        castle_type: CastleTypes,
    },
}

impl Iterator for PieceTypes {
    type Item = PieceTypes;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PieceTypes::Pawn => Some(PieceTypes::Knight),
            PieceTypes::Knight => Some(PieceTypes::Bishop),
            PieceTypes::Bishop => Some(PieceTypes::Rook),
            PieceTypes::Rook => Some(PieceTypes::Queen),
            PieceTypes::Queen => Some(PieceTypes::King),
            PieceTypes::King => None,
        }
    }
}