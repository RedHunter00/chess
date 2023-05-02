use chess_base as base;

use base::board::*;
use base::core::castles::Castles;
use base::core::color::Color;
use base::core::mov::Move;
use base::core::pieces::Pieces;
use base::core::position::Position;
use colored::*;
use fern::Dispatch;
use log::Level;
use log::LevelFilter;
use log::{debug, info};
use std::io;
use std::process::Command;
use std::thread;

fn setup_logger(log_level: LevelFilter) -> Result<(), fern::InitError> {
    let console_logger = Dispatch::new()
        .format(|out, message, record| {
            let mut level = format!("{}", record.level()).white();

            match record.level() {
                Level::Error => level = level.truecolor(240, 76, 76).bold(),
                Level::Warn => level = level.truecolor(245, 245, 67).bold(),
                Level::Debug => level = level.truecolor(79, 193, 255).bold(),
                Level::Info => level = level.truecolor(35, 209, 139).bold(),
                _ => (),
            };

            if record.level() == Level::Debug || record.level() == Level::Trace {
                out.finish(format_args!(
                    "{} [{}][{}][{}][{}] {}",
                    level,
                    chrono::Local::now().format("%H:%M:%S"),
                    thread::current().name().unwrap_or("<undefined>"),
                    record.file().unwrap_or("<undefined>"),
                    record.line().unwrap_or(0),
                    message
                ))
            } else {
                out.finish(format_args!("{} {}", level, message))
            }
        })
        .level(LevelFilter::Off)
        .level_for("chess_base", log_level)
        .level_for("chess_cli", log_level)
        .chain(io::stdout());
    //or stderr

    let logger = Dispatch::new().chain(console_logger);

    match logger.apply() {
        Ok(()) => Ok(()),
        Err(e) => Err(fern::InitError::SetLoggerError(e)),
    }
}

fn render_game(fen: String) {
    let x: Vec<&str> = fen.split("/").collect();

    for rank in (0..=7).rev() {
        print!("{}", (rank + 1).to_string().red());

        for char in x[rank].chars() {
            if char.is_digit(10) {
                for _ in 0..char.to_digit(10).unwrap() {
                    print!(" • ");
                }
            }

            #[cfg(target_os = "linux")]
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

            //not my fault cmd is shit
            //switch to linux
            #[cfg(target_os = "windows")]
            match char {
                'p' => print!(" {} ", "p".red()),
                'P' => print!(" {} ", "P".white()),
                'r' => print!(" {} ", "r".red()),
                'R' => print!(" {} ", "R".white()),
                'n' => print!(" {} ", "n".red()),
                'N' => print!(" {} ", "N".white()),
                'b' => print!(" {} ", "b".red()),
                'B' => print!(" {} ", "B".white()),
                'q' => print!(" {} ", "q".red()),
                'Q' => print!(" {} ", "Q".white()),
                'k' => print!(" {} ", "k".red()),
                'K' => print!(" {} ", "K".white()),
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
    #[cfg(target_os = "linux")]
    let _ = Command::new("clear").status();
    #[cfg(target_os = "windows")]
    let _ = Command::new("cls").status();

    board.make_move(mov);
    render_game(board.generate_fen());
}

/// Starts the game
/// Using a default board configuration
pub fn start_game() {
    let mut board = Board::new_arranged();

    render_game(board.generate_fen());

    loop {
        debug!(
            "number of moves ==> depth1: {} depth2: {}",
            board.calculate_nr_of_moves_with_depth(1),
            board.calculate_nr_of_moves_with_depth(2)
        );
        // debug!(
        //     "number of moves ==> depth3: {} depth4: {}",
        //     board.calculate_nr_of_moves_with_depth(3),
        //     board.calculate_nr_of_moves_with_depth(4)
        // );
        match board.get_turn() {
            Color::White => {
                if board.in_checkmate(Color::White) {
                    info!("Black wins by checkmate");
                    break;
                } else if board.in_stalemate(Color::White) {
                    info!("Stalemate");
                    break;
                }
            }
            Color::Black => {
                if board.in_checkmate(Color::Black) {
                    info!("White wins by checkmate");
                    break;
                } else if board.in_stalemate(Color::Black) {
                    info!("Stalemate");
                    break;
                }
            }
        }

        match board.get_turn() {
            Color::White => info!("White to move"),
            Color::Black => info!("Black to move"),
        }

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
                    "=Q" => piece_type = Pieces::Queen,
                    "=R" => piece_type = Pieces::Rook,
                    "=B" => piece_type = Pieces::Bishop,
                    "=Kn" => piece_type = Pieces::Knight,
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
                castle_type: Castles::QueenSide,
            };
            make_move(&mut board, mov);
            continue;
        }

        let kingside = input.contains("O-O");

        if kingside {
            let mov = Move::Castle {
                color: turn,
                castle_type: Castles::KingSide,
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

        make_move(&mut board, mov);
    }
}

fn main() {
    setup_logger(LevelFilter::Trace).unwrap();
    start_game();
}
