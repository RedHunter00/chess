#![allow(dead_code)]
mod board;
mod pieces;
mod positions;
mod utilities;
mod cli;

fn main() {
    cli::start_game();
}