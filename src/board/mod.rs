/// Board module
/// Contains the code for the board struct
/// Contains most of the high level game logic accessible for the user 

use crate::pieces::bishop::Bishop;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::pawn::Pawn;
use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;
use crate::positions::mov::Move;
use crate::utilities::Color::Black;
use crate::utilities::Color::White;
use crate::{pieces::piece::Piece, positions::position::Position};
use std::collections::HashMap;

/// Board struct
/// Contains a hashmap of all the pieces on the board
/// Contains a string representation of the board in FEN notation
pub struct Board {
    pieces: HashMap<Position, Box<dyn Piece>>,
    fen: String,
}

impl Board {
    /// Creates a new empty board
    pub fn new() -> Board {
        Board {
            pieces: HashMap::new(),
            fen: String::new(),
        }
    }

    /// Creates a new board with all the pieces in their starting positions
    pub fn new_arranged() -> Board {
        let mut pieces = HashMap::new();
        for rank in 0..=7 {
            for file in 0..=7 {
                let position = Position::new(file, rank);
                let piece = match rank {
                    0 => match file {
                        0 | 7 => Some(Box::new(Rook::new(position, White)) as Box<dyn Piece>),
                        1 | 6 => Some(Box::new(Knight::new(position, White)) as Box<dyn Piece>),
                        2 | 5 => Some(Box::new(Bishop::new(position, White)) as Box<dyn Piece>),
                        3 => Some(Box::new(Queen::new(position, White)) as Box<dyn Piece>),
                        4 => Some(Box::new(King::new(position, White)) as Box<dyn Piece>),
                        _ => None,
                    },
                    1 => Some(Box::new(Pawn::new(position, White)) as Box<dyn Piece>),
                    6 => Some(Box::new(Pawn::new(position, Black)) as Box<dyn Piece>),
                    7 => match file {
                        0 | 7 => Some(Box::new(Rook::new(position, Black)) as Box<dyn Piece>),
                        1 | 6 => Some(Box::new(Knight::new(position, Black)) as Box<dyn Piece>),
                        2 | 5 => Some(Box::new(Bishop::new(position, Black)) as Box<dyn Piece>),
                        3 => Some(Box::new(Queen::new(position, Black)) as Box<dyn Piece>),
                        4 => Some(Box::new(King::new(position, Black)) as Box<dyn Piece>),
                        _ => None,
                    },
                    _ => None,
                };
                if let Some(piece) = piece {
                    pieces.insert(position, piece);
                }
            }
        }
        Board {
            pieces,
            fen: String::new(),
        }
    }

    /// Returns a reference to the piece at the given position
    pub fn get_piece(&self, position: Position) -> Option<&Box<dyn Piece>> {
        self.pieces.get(&position)
    }

    //^ Only for debugging purposes for now
    /// Adds a piece to the board
    pub fn add_piece(&mut self, position: Position, piece: Box<dyn Piece>) {
        self.pieces.insert(position, piece);
    }

    /// return the current FEN notation of the board
    pub fn get_fen(&self) -> String {
        self.fen.clone()
    }

    //^ Only for debugging purposes for now
    /// prints the position and piece at that position
    pub fn print_position(&self, position: Position) {
        if let Some(piece) = self.get_piece(position) {
            println!(
                "{} {} {:?}",
                position,
                piece.get_piece_type(),
                piece.get_color()
            );
        } else {
            println!("{} None", position);
        }
    }

    //^ Only for debugging purposes for now
    /// prints all the positions and pieces on the board
    pub fn print_all_positions(&self) {
        for (position, piece) in self.pieces.iter() {
            println!(
                "{} {} {} {:?}",
                position.get_x(),
                position.get_y(),
                piece.get_piece_type(),
                piece.get_color()
            );
        }
    }

    /// Makes a move on the board
    /// Checks if the move is legal
    /// If it is legal, it makes the move
    /// If it is not legal, it does nothing
    pub fn make_move(&mut self, mov: Move) {
        match mov {
            Move::Normal { from, to } => {
                let moves = self.get_piece(from).unwrap().get_all_legal_moves(self);

                for mv in moves.as_slice() {
                    match mv {
                        Move::Normal { from, to } => {
                            println!("Legal move from {} to {}", from, to);
                        }
                        _ => (),
                    }
                }

                if moves.contains(&mov) {
                    println!("Normal move from {} to {}", from, to);
                    self.pieces.remove(&to);
                    let mut piece = self.pieces.remove(&from).unwrap();
                    piece.set_position(to);
                    self.pieces.insert(to, piece);
                } else {
                    println!("Illegal move from {} to {}", from, to);
                }
            }
            _ => (),
        }
    }

    /// Returns a vector of all the legal moves on the board
    pub fn get_all_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for (_, piece) in self.pieces.iter() {
            moves.append(&mut piece.get_all_legal_moves(self));
        }
        moves
    }

    // TODO add an enum for the different types of pieces instead of a string
    // TODO call it after every move and at game start
    /// Updates the FEN notation of the board
    pub fn update_fen(&mut self) {
        let mut fen = String::new();
        for rank in 0..=7 {
            let mut empty = 0;
            for file in 0..=7 {
                let position = Position::new(file, rank);
                if let Some(piece) = self.get_piece(position) {
                    if empty > 0 {
                        fen.push_str(&empty.to_string());
                        empty = 0;
                    }
                    match piece.get_color() {
                        White => match piece.get_piece_type().as_str() {
                            "Pawn" => fen.push_str("P"),
                            "Rook" => fen.push_str("R"),
                            "Knight" => fen.push_str("N"),
                            "Bishop" => fen.push_str("B"),
                            "Queen" => fen.push_str("Q"),
                            "King" => fen.push_str("K"),
                            _ => (),
                        },
                        Black => match piece.get_piece_type().as_str() {
                            "Pawn" => fen.push_str("p"),
                            "Rook" => fen.push_str("r"),
                            "Knight" => fen.push_str("n"),
                            "Bishop" => fen.push_str("b"),
                            "Queen" => fen.push_str("q"),
                            "King" => fen.push_str("k"),
                            _ => (),
                        },
                    }
                } else {
                    empty += 1;
                }
            }
            if empty > 0 {
                fen.push_str(&empty.to_string());
            }
            fen.push_str("/");
        }
        self.fen = fen;
    }
}
