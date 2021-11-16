mod bitboard;
mod board;
mod enums;
use bitboard::*;
use board::*;

fn main() {
    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into());
    println!("Initial Board State\n{}", board.get_board_state());
}
