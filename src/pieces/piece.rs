/// This file contains the Piece trait which is implemented by all pieces

use crate::board::Board;
use crate::positions::mov::Move;
use crate::positions::position::Position;
use crate::utilities::Color;

/// A core trait that defines the basic functionality of a chess piece.
/// 
/// # Examples
/// see any of the other pieces for examples
/// this trait is implemented on all pieces
pub trait Piece {

    /// Returns the color of the piece
    fn get_color(&self) -> Color;

    // TODO: Remove and replace with enum PieceType
    /// Returns the type of the piece as a string
    /// Same for all pieces
    fn get_piece_type(&self) -> String;

    /// Returns the position of the piece
    /// Same for all pieces
    fn get_position(&self) -> Position;

    /// Sets the position of the piece
    /// Same for all pieces
    fn set_position(&mut self, position: Position);

    /// Returns a vector of all the legal moves for the piece
    /// Each piece has a different implementation of this function
    fn get_all_legal_moves(&self, board: &Board) -> Vec<Move>;
}
