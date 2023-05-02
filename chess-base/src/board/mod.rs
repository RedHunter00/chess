/// Board module
/// Contains the code for the board struct
/// Contains most of the high level game logic accessible for the user
mod modifiers;
use crate::core::castles::Castles::{self, KingSide, QueenSide};
use crate::core::color::Color;
use crate::core::color::Color::Black;
use crate::core::color::Color::White;
use crate::core::direction::Direction;
use crate::core::direction::Direction::{Down, Up};
use crate::core::mov::Move;
use crate::core::piece::Piece;
use crate::core::pieces::Pieces;
use crate::core::position::Position;
use modifiers::Modifiers;
use std::collections::HashMap;

/// Board struct
/// Contains a hashmap of all the pieces on the board
/// Contains a string representation of the board in FEN notation
#[derive(Debug, Clone)]
pub struct Board {
    pieces: HashMap<Position, Piece>,
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
                        0 | 7 => Some(Piece::new(position, White, Pieces::Rook)),
                        1 | 6 => Some(Piece::new(position, White, Pieces::Knight)),
                        2 | 5 => Some(Piece::new(position, White, Pieces::Bishop)),
                        3 => Some(Piece::new(position, White, Pieces::Queen)),
                        4 => Some(Piece::new(position, White, Pieces::King)),
                        _ => None,
                    },
                    1 => Some(Piece::new(position, White, Pieces::Pawn)),
                    6 => Some(Piece::new(position, Black, Pieces::Pawn)),
                    7 => match file {
                        0 | 7 => Some(Piece::new(position, Black, Pieces::Rook)),
                        1 | 6 => Some(Piece::new(position, Black, Pieces::Knight)),
                        2 | 5 => Some(Piece::new(position, Black, Pieces::Bishop)),
                        3 => Some(Piece::new(position, Black, Pieces::Queen)),
                        4 => Some(Piece::new(position, Black, Pieces::King)),
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

    pub fn from_fen(fen: &str) -> Board {
        let mut pieces = HashMap::new();
        let modifiers = Modifiers::new();
        let turn = Color::White;
        let mut rank = 7;
        let mut file = 0;

        for c in fen.chars() {
            match c {
                ' ' => break,
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                'K' => {
                    pieces.insert(
                        Position::new(file, rank),
                        Piece::new(Position::new(file, rank), Color::White, Pieces::King),
                    );
                    file += 1;
                }
                'Q' => {
                    pieces.insert(
                        Position::new(file, rank),
                        Piece::new(Position::new(file, rank), Color::White, Pieces::Queen),
                    );
                    file += 1;
                }
                'R' => {
                    pieces.insert(
                        Position::new(file, rank),
                        Piece::new(Position::new(file, rank), Color::White, Pieces::Rook),
                    );
                    file += 1;
                }
                'B' => {
                    pieces.insert(
                        Position::new(file, rank),
                        Piece::new(Position::new(file, rank), Color::White, Pieces::Bishop),
                    );
                    file += 1;
                }
                'N' => {
                    pieces.insert(
                        Position::new(file, rank),
                        Piece::new(Position::new(file, rank), Color::White, Pieces::Knight),
                    );
                    file += 1;
                }
                'P' => {
                    pieces.insert(
                        Position::new(file, rank),
                        Piece::new(Position::new(file, rank), Color::White, Pieces::Pawn),
                    );
                    file += 1;
                }
                'k' => {
                    pieces.insert(
                        Position::new(file, rank),
                        Piece::new(Position::new(file, rank), Color::Black, Pieces::King),
                    );
                    file += 1;
                }
                'q' => {
                    pieces.insert(
                        Position::new(file, rank),
                        Piece::new(Position::new(file, rank), Color::Black, Pieces::Queen),
                    );
                    file += 1;
                }
                'r' => {
                    pieces.insert(
                        Position::new(file, rank),
                        Piece::new(Position::new(file, rank), Color::Black, Pieces::Rook),
                    );
                    file += 1;
                }
                'b' => {
                    pieces.insert(
                        Position::new(file, rank),
                        Piece::new(Position::new(file, rank), Color::Black, Pieces::Bishop),
                    );
                    file += 1;
                }
                'n' => {
                    pieces.insert(
                        Position::new(file, rank),
                        Piece::new(Position::new(file, rank), Color::Black, Pieces::Knight),
                    );
                    file += 1;
                }
                'p' => {
                    pieces.insert(
                        Position::new(file, rank),
                        Piece::new(Position::new(file, rank), Color::Black, Pieces::Pawn),
                    );
                    file += 1;
                }
                '1'..='8' => {
                    let count = c.to_digit(10).unwrap() as usize;
                    for _ in 0..count {
                        pieces.remove(&Position::new(file, rank));
                        file += 1;
                    }
                }
                _ => (),
            }
        }

        Board {
            pieces,
            modifiers,
            turn,
            fen: fen.to_string(),
        }
    }

    /// Returns a reference to the piece at the given position
    pub fn get_piece(&self, position: Position) -> Option<&Piece> {
        self.pieces.get(&position)
    }

    pub fn get_all_pieces(&self) -> &HashMap<Position, Piece> {
        &self.pieces
    }

    pub fn get_turn(&self) -> Color {
        self.turn
    }

    pub fn get_fen(&self) -> String {
        self.fen.clone()
    }

    pub fn get_modifiers(&self) -> &Modifiers {
        &self.modifiers
    }

    pub fn remove_piece(&mut self, position: Position) -> Option<Piece> {
        self.pieces.remove(&position)
    }

    pub fn add_piece(&mut self, position: Position, piece: Piece) {
        self.pieces.insert(position, piece);
    }

    pub fn generate_fen(&mut self) -> String {
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
                        White => match piece.get_piece_type() {
                            Pieces::Pawn => fen.push_str("P"),
                            Pieces::Rook => fen.push_str("R"),
                            Pieces::Knight => fen.push_str("N"),
                            Pieces::Bishop => fen.push_str("B"),
                            Pieces::Queen => fen.push_str("Q"),
                            Pieces::King => fen.push_str("K"),
                        },
                        Black => match piece.get_piece_type() {
                            Pieces::Pawn => fen.push_str("p"),
                            Pieces::Rook => fen.push_str("r"),
                            Pieces::Knight => fen.push_str("n"),
                            Pieces::Bishop => fen.push_str("b"),
                            Pieces::Queen => fen.push_str("q"),
                            Pieces::King => fen.push_str("k"),
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
        fen
    }

    pub fn get_all_legal_moves_for_black(&self) -> Vec<Move> {
        let pseudo_moves = self.get_all_pseudo_legal_moves_for_black();
        self.verify_checks(pseudo_moves)
    }

    pub fn get_all_legal_moves_for_white(&self) -> Vec<Move> {
        let pseudo_moves = self.get_all_pseudo_legal_moves_for_white();
        self.verify_checks(pseudo_moves)
    }

    pub fn get_all_legal_moves(&self) -> Vec<Move> {
        let pseudo_moves = self.get_all_pseudo_legal_moves();
        self.verify_checks(pseudo_moves)
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        let mut king_pos = None;
        for piece in self.get_all_pieces().values() {
            if piece.get_piece_type() == Pieces::King && piece.get_color() == color {
                king_pos = Some(piece.get_position());
                break;
            }
        }

        if let None = king_pos {
            return false;
        }

        let king_pos = king_pos.unwrap();

        let mut in_check = false;
        match color {
            White => {
                for mov in self.get_all_pseudo_legal_moves_for_black() {
                    match mov {
                        Move::Normal { from: _, to } => {
                            if to == king_pos {
                                in_check = true;
                            }
                        }
                        _ => (),
                    }
                }
            }
            Black => {
                for mov in self.get_all_pseudo_legal_moves_for_white() {
                    match mov {
                        Move::Normal { from: _, to } => {
                            if to == king_pos {
                                in_check = true;
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        in_check
    }

    pub fn calculate_nr_of_moves_with_depth(&self, depth: u32) -> u32 {
        let mut nr_of_moves = 0;
        if depth == 0 {
            return 1;
        }

        match self.get_turn() {
            White => {
                for mov in self.get_all_legal_moves_for_white() {
                    let mut board = self.clone();
                    board.make_move(mov);
                    nr_of_moves += board.calculate_nr_of_moves_with_depth(depth - 1);
                }
            }
            Black => {
                for mov in self.get_all_legal_moves_for_black() {
                    let mut board = self.clone();
                    board.make_move(mov);
                    nr_of_moves += board.calculate_nr_of_moves_with_depth(depth - 1);
                }
            }
        }

        nr_of_moves
    }

    pub fn make_move_no_checks(&mut self, mov: Move) {
        match mov {
            Move::Normal { from, to } => {
                let piece = self.remove_piece(from).unwrap();
                self.remove_piece(to);
                self.add_piece(to, piece);
            }
            Move::Promotion {
                from,
                to,
                promotion,
            } => {
                let piece = self.remove_piece(from).unwrap();
                self.remove_piece(to);
                self.add_piece(to, Piece::new(to, piece.get_color(), promotion));
            }
            Move::Castle { color, castle_type } => match color {
                White => match castle_type {
                    Castles::KingSide => {
                        let king = self.remove_piece(Position::new(4, 0)).unwrap();
                        let rook = self.remove_piece(Position::new(7, 0)).unwrap();
                        self.add_piece(Position::new(6, 0), king);
                        self.add_piece(Position::new(5, 0), rook);
                    }
                    Castles::QueenSide => {
                        let king = self.remove_piece(Position::new(4, 0)).unwrap();
                        let rook = self.remove_piece(Position::new(0, 0)).unwrap();
                        self.add_piece(Position::new(2, 0), king);
                        self.add_piece(Position::new(3, 0), rook);
                    }
                },
                Black => match castle_type {
                    Castles::KingSide => {
                        let king = self.remove_piece(Position::new(4, 7)).unwrap();
                        let rook = self.remove_piece(Position::new(7, 7)).unwrap();
                        self.add_piece(Position::new(6, 7), king);
                        self.add_piece(Position::new(5, 7), rook);
                    }
                    Castles::QueenSide => {
                        let king = self.remove_piece(Position::new(4, 7)).unwrap();
                        let rook = self.remove_piece(Position::new(0, 7)).unwrap();
                        self.add_piece(Position::new(2, 7), king);
                        self.add_piece(Position::new(3, 7), rook);
                    }
                },
            },
        }
    }

    pub fn in_checkmate(&self, color: Color) -> bool {
        match color {
            White => {
                let pseudo_moves = self.get_all_legal_moves_for_white();
                let moves = self.verify_checks(pseudo_moves);
                if moves.len() == 0 && self.is_in_check(color) {
                    return true;
                }
            }
            Black => {
                let pseudo_moves = self.get_all_legal_moves_for_black();
                let moves = self.verify_checks(pseudo_moves);
                if moves.len() == 0 && self.is_in_check(color) {
                    return true;
                }
            }
        }

        false
    }

    pub fn in_stalemate(&self, color: Color) -> bool {
        match color {
            White => {
                let pseudo_moves = self.get_all_legal_moves_for_white();
                let moves = self.verify_checks(pseudo_moves);
                if moves.len() == 0 {
                    return true;
                }
            }
            Black => {
                let pseudo_moves = self.get_all_legal_moves_for_black();
                let moves = self.verify_checks(pseudo_moves);
                if moves.len() == 0 {
                    return true;
                }
            }
        }

        false
    }

    pub fn make_move(&mut self, mov: Move) -> bool {
        match mov {
            Move::Normal { from, to } => {
                if from.get_x() > 7 || from.get_y() > 7 || to.get_x() > 7 || to.get_y() > 7 {
                    //warn!("Invalid location at from {} and/or to {}", from, to);
                    return false;
                }

                if let None = self.get_piece(from) {
                    //warn!("No piece at {}", from);
                    return false;
                }

                if let Some(piece) = self.get_piece(from) {
                    if piece.get_color() != self.turn {
                        //warn!("Wrong color piece at {}", from);
                        return false;
                    }
                }

                let moves = self.get_piece(from).unwrap().get_all_legal_moves(self);

                // DEBUG
                // debug!("!!PSEUDO LEGAL MOVES:");
                // for mv in moves.as_slice() {
                //     match mv {
                //         Move::Normal { from, to } => {
                //             debug!("Legal move from {} to {}", from, to);
                //         }
                //         _ => (),
                //     }
                // }
                // END DEBUG

                let moves = self.verify_checks(moves);

                if moves.contains(&mov) {
                    //info!("Making move from {} to {}", from, to);

                    self.pieces.remove(&to);
                    let mut piece = self.pieces.remove(&from).unwrap();
                    piece.set_position(to);

                    let to_x = to.get_x();
                    let to_y = to.get_y();

                    match to_x {
                        0 => {
                            if to_y == 0 {
                                self.modifiers.can_white_castle_queenside = false;
                            } else if to_y == 7 {
                                self.modifiers.can_black_castle_queenside = false;
                            }
                        }
                        7 => {
                            if to_y == 0 {
                                self.modifiers.can_white_castle_kingside = false;
                            } else if to_y == 7 {
                                self.modifiers.can_black_castle_kingside = false;
                            }
                        }
                        _ => (),
                    }

                    if piece.get_piece_type() == Pieces::King {
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
                    } else if piece.get_piece_type() == Pieces::Rook {
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
                    } else if piece.get_piece_type() == Pieces::Pawn {
                        match piece.get_color() {
                            White => {
                                if let Some(ep) = self.modifiers.en_passant {
                                    if ep == to {
                                        self.pieces.remove(
                                            &piece.get_position().increment(Up, 1).unwrap(),
                                        );
                                    }

                                    self.modifiers.en_passant = None;
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

                                self.modifiers.en_passant = None;
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
                    //warn!("Illegal move from {} to {}", from, to);
                    return false;
                }
            }
            Move::Castle { color, castle_type } => {
                let mut moves = vec![];
                for piece in self.pieces.values() {
                    if piece.get_color() == color && piece.get_piece_type() == Pieces::King {
                        moves = piece.get_all_legal_moves(self);
                    }
                }

                let moves = self.verify_checks(moves);

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
                    //warn!("Illegal castleing move");
                    return false;
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

                let moves = self.verify_checks(moves);

                if moves.contains(&mov) {
                    let to_x = to.get_x();
                    let to_y = to.get_y();

                    match to_x {
                        0 => {
                            if to_y == 0 {
                                self.modifiers.can_white_castle_queenside = false;
                            } else if to_y == 7 {
                                self.modifiers.can_black_castle_queenside = false;
                            }
                        }
                        7 => {
                            if to_y == 0 {
                                self.modifiers.can_white_castle_kingside = false;
                            } else if to_y == 7 {
                                self.modifiers.can_black_castle_kingside = false;
                            }
                        }
                        _ => (),
                    }

                    match self.turn {
                        White => {
                            match promotion {
                                Pieces::Queen => {
                                    self.pieces.insert(to, Piece::new(to, White, Pieces::Queen))
                                }
                                Pieces::Rook => {
                                    self.pieces.insert(to, Piece::new(to, White, Pieces::Rook))
                                }
                                Pieces::Bishop => self
                                    .pieces
                                    .insert(to, Piece::new(to, White, Pieces::Bishop)),
                                Pieces::Knight => self
                                    .pieces
                                    .insert(to, Piece::new(to, White, Pieces::Knight)),
                                _ => self.pieces.insert(to, Piece::new(to, White, Pieces::Queen)),
                            };
                        }
                        Black => {
                            match promotion {
                                Pieces::Queen => {
                                    self.pieces.insert(to, Piece::new(to, Black, Pieces::Queen))
                                }
                                Pieces::Rook => {
                                    self.pieces.insert(to, Piece::new(to, Black, Pieces::Rook))
                                }
                                Pieces::Bishop => self
                                    .pieces
                                    .insert(to, Piece::new(to, Black, Pieces::Bishop)),
                                Pieces::Knight => self
                                    .pieces
                                    .insert(to, Piece::new(to, Black, Pieces::Knight)),
                                _ => self.pieces.insert(to, Piece::new(to, Black, Pieces::Queen)),
                            };
                        }
                    }

                    self.pieces.remove(&from);
                } else {
                    //warn!("Illegal promotion move from {} to {}", from, to);
                    return false;
                }
            }
        }

        self.update_fen();

        self.turn = match self.turn {
            White => Black,
            Black => White,
        };

        if let Some(_) = self.get_modifiers().en_passant {
            //info!("En passant at {}", pos);
        }

        return true;
    }

    pub fn undo_move(&mut self, mov: Move) {
        match mov {
            Move::Normal { from, to } => {
                let mut piece = self.pieces.remove(&to).unwrap();
                piece.set_position(from);
                self.pieces.insert(from, piece);
            }
            _ => (),
        }
    }

    fn verify_checks(&self, pseudo_moves: Vec<Move>) -> Vec<Move> {
        let mut moves = Vec::new();

        for mov in pseudo_moves {
            match mov {
                Move::Normal { from, to } => {
                    let mut board = self.clone();
                    board.pieces.remove(&to);
                    let mut piece = board.pieces.remove(&from).unwrap();
                    piece.set_position(to);
                    board.pieces.insert(to, piece);

                    if !board.is_in_check(self.get_turn()) {
                        moves.push(mov);
                    }
                }
                Move::Castle { color, castle_type } => {
                    let mut board = self.clone();

                    match castle_type {
                        KingSide => {
                            let king_from = match color {
                                White => Position::new(4, 0),
                                Black => Position::new(4, 7),
                            };

                            let king_to = match color {
                                White => Position::new(5, 0),
                                Black => Position::new(5, 7),
                            };

                            let mut king = board.pieces.remove(&king_from).unwrap();
                            king.set_position(king_to);
                            board.pieces.insert(king_to, king);

                            if board.is_in_check(self.get_turn()) {
                                break;
                            }

                            let mut board = self.clone();

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

                            let mut king = board.pieces.remove(&king_from).unwrap();
                            king.set_position(king_to);
                            board.pieces.insert(king_to, king);

                            let mut rook = board.pieces.remove(&rook_from).unwrap();
                            rook.set_position(rook_to);
                            board.pieces.insert(rook_to, rook);
                        }
                        QueenSide => {
                            let king_from = match color {
                                White => Position::new(4, 0),
                                Black => Position::new(4, 7),
                            };

                            let king_to = match color {
                                White => Position::new(3, 0),
                                Black => Position::new(3, 7),
                            };

                            let mut board = self.clone();

                            let mut king = board.pieces.remove(&king_from).unwrap();
                            king.set_position(king_to);
                            board.pieces.insert(king_to, king);

                            if board.is_in_check(self.get_turn()) {
                                break;
                            }

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

                            let mut king = board.pieces.remove(&king_from).unwrap();
                            king.set_position(king_to);
                            board.pieces.insert(king_to, king);

                            let mut rook = board.pieces.remove(&rook_from).unwrap();
                            rook.set_position(rook_to);
                            board.pieces.insert(rook_to, rook);
                        }
                    }

                    if !board.is_in_check(self.get_turn()) {
                        moves.push(mov);
                    }
                }
                Move::Promotion {
                    from,
                    to,
                    promotion: _,
                } => {
                    let mut board = self.clone();

                    board.pieces.remove(&to);
                    let mut piece = board.pieces.remove(&from).unwrap();
                    piece.set_position(to);
                    board.pieces.insert(to, piece);

                    if !board.is_in_check(self.get_turn()) {
                        moves.push(mov);
                    }
                }
            }
        }

        moves
    }

    fn get_all_pseudo_legal_moves_for_white(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for piece in self.get_all_pieces().values() {
            if piece.get_color() == White {
                moves.append(&mut piece.get_all_legal_moves(self));
            }
        }
        moves
    }

    fn get_all_pseudo_legal_moves_for_black(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for piece in self.get_all_pieces().values() {
            if piece.get_color() == Black {
                moves.append(&mut piece.get_all_legal_moves(self));
            }
        }
        moves
    }

    fn get_all_pseudo_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for piece in self.get_all_pieces().values() {
            moves.append(&mut piece.get_all_legal_moves(self));
        }
        moves
    }

    fn update_fen(&mut self) {
        self.fen = self.generate_fen();
    }
}
