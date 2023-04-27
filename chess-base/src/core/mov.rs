use super::castles::Castles;
use super::color::Color;
use super::pieces::Pieces;
/// This module contains the Move struct and its related enums.
use super::position::Position;

/// Represents a move on the board
/// It has 3 variants:
/// Normal: a normal move
/// Promotion: a move that promotes a pawn
/// Castle: a move that involves castleing a king
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Move {
    Normal {
        from: Position,
        to: Position,
    },
    Promotion {
        from: Position,
        to: Position,
        promotion: Pieces,
    },
    Castle {
        color: Color,
        castle_type: Castles,
    },
}
