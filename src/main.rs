mod bitboard;
mod board;
use bitboard::*;
use board::*;

fn main() {
    let board = Board::new();
    println!("Initial Board State\n{}", board.get_board_state());
}
