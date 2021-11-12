mod bitboard;
mod board;
mod enums;
use bitboard::*;
use board::*;
use enums::Pieces;

fn main() {
    let board = Board::new();
    println!("Initial Board State\n{}", board.get_board_state());
}
