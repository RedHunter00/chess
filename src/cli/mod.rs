/// The cli module contains the code for the command line interface
use crate::board::*;
use crate::positions::mov::CastleTypes;
use crate::positions::mov::Move;
use crate::positions::mov::PieceTypes;
use crate::positions::position::Position;
use colored::*;
use std::process::Command;

//TODO: make squares white and black instead of dots
/// Renders the game to the terminal (cli)
fn render_game(fen: String) {
    let x: Vec<&str> = fen.split("/").collect();

    #[cfg(target_os = "linux")]
    let _ = Command::new("clear").status();

    #[cfg(target_os = "windows")]
    let _ = Command::new("cls").status();

    for rank in (0..=7).rev() {
        print!("{}", (rank + 1).to_string().red());

        for char in x[rank].chars() {
            if char.is_digit(10) {
                for _ in 0..char.to_digit(10).unwrap() {
                    print!(" • ");
                }
            }
            match char {
                'p' => print!(" {} ", "♟".red()),
                'P' => print!(" {} ", "♟".white()),
                'r' => print!(" {} ", "♜".red()),
                'R' => print!(" {} ", "♜".white()),
                'n' => print!(" {} ", "♞".red()),
                'N' => print!(" {} ", "♞".white()),
                'b' => print!(" {} ", "♝".red()),
                'B' => print!(" {} ", "♝".white()),
                'q' => print!(" {} ", "♛".red()),
                'Q' => print!(" {} ", "♛".white()),
                'k' => print!(" {} ", "♚".red()),
                'K' => print!(" {} ", "♚".white()),
                _ => (),
            }
        }

        println!();
    }

    print!(" ");
    let mut letter = 'a';
    for _ in 0..=7 {
        let f = String::from(letter);
        let f = f.red();
        print!(" {} ", f);
        letter = (letter as u8 + 1) as char;
    }

    println!();
}

/// Makes a move on the board and re-renders the game
fn make_move(board: &mut Board, mov: Move) {
    board.make_move(mov);
    board.update_fen();
    render_game(board.get_fen());
}

/// Starts the game
/// Using a default board configuration
pub fn start_game() {
    let mut board = Board::new_arranged();

    board.update_fen();
    render_game(board.get_fen());

    loop {
        println!("\nenter move:");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let turn = board.get_turn();

        let promotions = vec!["=Q", "=R", "=B", "=Kn"];

        for pat in promotions {
            if input.contains(pat) {
                let input: Vec<&str> = input.split_whitespace().collect();

                let from = Position::from_an(
                    input[0].chars().nth(0).unwrap(),
                    input[0].chars().nth(1).unwrap().to_digit(10).unwrap() as u8,
                );
                let to = Position::from_an(
                    input[1].chars().nth(0).unwrap(),
                    input[1].chars().nth(1).unwrap().to_digit(10).unwrap() as u8,
                );

                let piece_type;

                match pat {
                    "=Q" => piece_type = PieceTypes::Queen,
                    "=R" => piece_type = PieceTypes::Rook,
                    "=B" => piece_type = PieceTypes::Bishop,
                    "=Kn" => piece_type = PieceTypes::Knight,
                    _ => panic!("Invalid promotion"),
                }

                let mov = Move::Promotion {
                    from,
                    to,
                    promotion: piece_type,
                };

                make_move(&mut board, mov);
                continue;
            }
        }

        let queenside = input.contains("O-O-O");

        if queenside {
            let mov = Move::Castle {
                color: turn,
                castle_type: CastleTypes::QueenSide,
            };
            make_move(&mut board, mov);
            continue;
        }

        let kingside = input.contains("O-O");

        if kingside {
            let mov = Move::Castle {
                color: turn,
                castle_type: CastleTypes::KingSide,
            };
            make_move(&mut board, mov);
            continue;
        }

        let input: Vec<&str> = input.split_whitespace().collect();

        let from = Position::from_an(
            input[0].chars().nth(0).unwrap(),
            input[0].chars().nth(1).unwrap().to_digit(10).unwrap() as u8,
        );
        let to = Position::from_an(
            input[1].chars().nth(0).unwrap(),
            input[1].chars().nth(1).unwrap().to_digit(10).unwrap() as u8,
        );
        let mov = Move::Normal { from, to };

        println!("{} {}", from, to);
        make_move(&mut board, mov);
    }
}
