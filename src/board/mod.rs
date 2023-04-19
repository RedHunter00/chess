/// Board module
/// Contains the code for the board struct
/// Contains most of the high level game logic accessible for the user
mod modifiers;
use crate::pieces::bishop::Bishop;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::pawn::Pawn;
use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;
use crate::positions::mov::CastleTypes::{KingSide, QueenSide};
use crate::positions::mov::Move;
use crate::positions::mov::PieceTypes;
use crate::positions::position::Direction;
use crate::positions::position::Direction::{Down, Up};
use crate::utilities::Color;
use crate::utilities::Color::Black;
use crate::utilities::Color::White;
use crate::{pieces::piece::Piece, positions::position::Position};
use modifiers::Modifiers;
use std::collections::HashMap;

/// Board struct
/// Contains a hashmap of all the pieces on the board
/// Contains a string representation of the board in FEN notation
pub struct Board {
    pieces: HashMap<Position, Box<dyn Piece>>,
    modifiers: Modifiers,
    turn: Color,
    fen: String,
}

impl Board {
    /// Creates a new empty board
    pub fn new() -> Board {
        Board {
            pieces: HashMap::new(),
            modifiers: Modifiers::new(),
            turn: White,
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
            modifiers: Modifiers::new(),
            turn: White,
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

    pub fn get_turn(&self) -> Color {
        self.turn
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

    pub fn get_modifiers(&self) -> &Modifiers {
        &self.modifiers
    }

    /// Makes a move on the board
    /// Checks if the move is legal
    /// If it is legal, it makes the move
    pub fn make_move(&mut self, mov: Move) {
        // match self.get_modifiers().en_passant {
        //     Some(_) => {
        //         self.modifiers.en_passant = None;
        //     }
        //     None => (),
        // }

        match mov {
            Move::Normal { from, to } => {
                if from.get_x() > 7 || from.get_y() > 7 || to.get_x() > 7 || to.get_y() > 7 {
                    println!("Invalid location at from {} and/or to {}", from, to);
                    return;
                }

                if let None = self.get_piece(from) {
                    println!("No piece at {}", from);
                    return;
                }

                if let Some(piece) = self.get_piece(from) {
                    if piece.get_color() != self.turn {
                        println!("Wrong color piece at {}", from);
                        return;
                    }
                }

                let moves = self.get_piece(from).unwrap().get_all_legal_moves(self);

                // & Only for debugging purposes
                // for mv in moves.as_slice() {
                //     match mv {
                //         Move::Normal { from, to } => {
                //             println!("Legal move from {} to {}", from, to);
                //         }
                //         _ => (),
                //     }
                // }

                if moves.contains(&mov) {
                    println!("Normal move from {} to {}", from, to);

                    self.pieces.remove(&to);
                    let mut piece = self.pieces.remove(&from).unwrap();
                    piece.set_position(to);

                    if piece.get_piece_type() == "King" {
                        match self.turn {
                            White => {
                                self.modifiers.can_white_castle_kingside = false;
                                self.modifiers.can_white_castle_queenside = false;
                            }
                            Black => {
                                self.modifiers.can_black_castle_kingside = false;
                                self.modifiers.can_black_castle_queenside = false;
                            }
                        }
                    } else if piece.get_piece_type() == "Rook" {
                        match self.turn {
                            White => {
                                if from == Position::new(0, 0) {
                                    self.modifiers.can_white_castle_queenside = false;
                                } else if from == Position::new(7, 0) {
                                    self.modifiers.can_white_castle_kingside = false;
                                }
                            }
                            Black => {
                                if from == Position::new(0, 7) {
                                    self.modifiers.can_black_castle_queenside = false;
                                } else if from == Position::new(7, 7) {
                                    self.modifiers.can_black_castle_kingside = false;
                                }
                            }
                        }
                    } else if piece.get_piece_type() == "Pawn" {
                        match piece.get_color() {
                            White => {
                                if let Some(ep) = self.modifiers.en_passant {
                                    if ep == to {
                                        self.pieces.remove(
                                            &piece.get_position().increment(Up, 1).unwrap(),
                                        );
                                    }
                                }
                            }
                            Black => {
                                if let Some(ep) = self.modifiers.en_passant {
                                    if ep == to {
                                        self.pieces.remove(
                                            &piece.get_position().increment(Down, 1).unwrap(),
                                        );
                                    }
                                }
                            }
                        }

                        match self.turn {
                            White => {
                                if from.get_y() == 1 && to.get_y() == 3 {
                                    self.modifiers.en_passant =
                                        Some(to.increment(Direction::Up, 1).unwrap());
                                }
                            }
                            Black => {
                                if from.get_y() == 6 && to.get_y() == 4 {
                                    self.modifiers.en_passant =
                                        Some(to.increment(Direction::Down, 1).unwrap());
                                }
                            }
                        }
                    }

                    self.pieces.insert(to, piece);
                } else {
                    println!("Illegal move from {} to {}", from, to);
                    return;
                }
            }
            Move::Castle { color, castle_type } => {
                let mut moves = vec![];
                for piece in self.pieces.values() {
                    if piece.get_color() == color && piece.get_piece_type() == "King" {
                        moves = piece.get_all_legal_moves(self);
                    }
                }

                if moves.contains(&mov) {
                    match castle_type {
                        KingSide => {
                            let king_from = match color {
                                White => Position::new(4, 0),
                                Black => Position::new(4, 7),
                            };
                            let king_to = match color {
                                White => Position::new(6, 0),
                                Black => Position::new(6, 7),
                            };
                            let rook_from = match color {
                                White => Position::new(7, 0),
                                Black => Position::new(7, 7),
                            };
                            let rook_to = match color {
                                White => Position::new(5, 0),
                                Black => Position::new(5, 7),
                            };

                            let mut king = self.pieces.remove(&king_from).unwrap();
                            king.set_position(king_to);
                            self.pieces.insert(king_to, king);

                            let mut rook = self.pieces.remove(&rook_from).unwrap();
                            rook.set_position(rook_to);
                            self.pieces.insert(rook_to, rook);
                        }
                        QueenSide => {
                            let king_from = match color {
                                White => Position::new(4, 0),
                                Black => Position::new(4, 7),
                            };
                            let king_to = match color {
                                White => Position::new(2, 0),
                                Black => Position::new(2, 7),
                            };
                            let rook_from = match color {
                                White => Position::new(0, 0),
                                Black => Position::new(0, 7),
                            };
                            let rook_to = match color {
                                White => Position::new(3, 0),
                                Black => Position::new(3, 7),
                            };

                            let mut king = self.pieces.remove(&king_from).unwrap();
                            king.set_position(king_to);
                            self.pieces.insert(king_to, king);

                            let mut rook = self.pieces.remove(&rook_from).unwrap();
                            rook.set_position(rook_to);
                            self.pieces.insert(rook_to, rook);
                        }
                    }
                } else {
                    println!("Illegal castleing move");
                    return;
                }

                match self.turn {
                    White => {
                        self.modifiers.can_white_castle_kingside = false;
                        self.modifiers.can_white_castle_queenside = false;
                    }
                    Black => {
                        self.modifiers.can_black_castle_kingside = false;
                        self.modifiers.can_black_castle_queenside = false;
                    }
                }
            }
            Move::Promotion {
                from,
                to,
                promotion,
            } => {
                let mut moves = vec![];
                for piece in self.pieces.values() {
                    if piece.get_position() == from {
                        moves = piece.get_all_legal_moves(self);
                    }
                }

                if moves.contains(&mov) {
                    match promotion {
                        PieceTypes::Queen => {
                            self.pieces.insert(to, Box::new(Queen::new(to, White)))
                        }
                        PieceTypes::Rook => self.pieces.insert(to, Box::new(Rook::new(to, White))),
                        PieceTypes::Bishop => {
                            self.pieces.insert(to, Box::new(Bishop::new(to, White)))
                        }
                        PieceTypes::Knight => {
                            self.pieces.insert(to, Box::new(Knight::new(to, White)))
                        }
                        _ => self.pieces.insert(to, Box::new(Queen::new(to, White))),
                    };

                    self.pieces.remove(&from);
                } else {
                    println!("Illegal promotion move from {} to {}", from, to);
                    return;
                }
            }
        }

        self.turn = match self.turn {
            White => Black,
            Black => White,
        };

        if let Some(pos) = self.get_modifiers().en_passant {
            println!("En passant at {}", pos);
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
