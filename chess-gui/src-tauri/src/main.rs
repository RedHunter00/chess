#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chess_base as base;

use base::board::*;
// use base::core::castles::Castles;
// use base::core::color::Color;
use base::core::mov::Move;
use base::core::position::Position;
use std::sync::Mutex;
use tauri::command;

#[command]
fn get_fen() -> String {
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string()
}

#[derive(serde::Deserialize)]
struct MoveData {
    from: String,
    to: String,
}

#[command]
fn make_move(move_data: MoveData, board: tauri::State<Mutex<Board>>) -> bool {
    let mut board = board.lock().unwrap();
    let from_position = Position::from_an(
        move_data.from.chars().nth(0).unwrap(),
        move_data.from.chars().nth(1).unwrap().to_digit(10).unwrap() as u8,
    );
    let to_position = Position::from_an(
        move_data.to.chars().nth(0).unwrap(),
        move_data.to.chars().nth(1).unwrap().to_digit(10).unwrap() as u8,
    );

    println!("from: {:?}, to: {:?}", from_position, to_position);

    let mov = Move::Normal {
        from: from_position,
        to: to_position,
    };

    //no way, am too lazy to implement this

    // if from_position.get_x() == 4
    //     && from_position.get_y() == 0
    //     && to_position.get_x() == 6
    //     && to_position.get_y() == 0
    // {
    //     mov = Move::Castle {
    //         color: Color::White,
    //         castle_type: Castles::KingSide,
    //     };
    // } else if from_position.get_x() == 4
    //     && from_position.get_y() == 0
    //     && to_position.get_x() == 2
    //     && to_position.get_y() == 0
    // {
    //     mov = Move::Castle {
    //         color: Color::White,
    //         castle_type: Castles::QueenSide,
    //     };
    // } else if from_position.get_x() == 4
    //     && from_position.get_y() == 7
    //     && to_position.get_x() == 6
    //     && to_position.get_y() == 7
    // {
    //     mov = Move::Castle {
    //         color: Color::Black,
    //         castle_type: Castles::KingSide,
    //     };
    // } else if from_position.get_x() == 4
    //     && from_position.get_y() == 7
    //     && to_position.get_x() == 2
    //     && to_position.get_y() == 7
    // {
    //     mov = Move::Castle {
    //         color: Color::Black,
    //         castle_type: Castles::QueenSide,
    //     };
    // }

    board.make_move(mov)
}

fn main() {
    // ... setup logger and start_game code

    let board = Board::new_arranged();
    let board = Mutex::new(board);

    tauri::Builder::default()
        .manage(board)
        .invoke_handler(tauri::generate_handler![get_fen, make_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
