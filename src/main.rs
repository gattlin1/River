mod bitboard;
mod board;
use bitboard::*;
use board::*;

fn main() {
    let mut board = Board::new();
    board.get_board_state();
}
