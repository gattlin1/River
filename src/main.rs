pub mod engine;
use crate::engine::*;

fn main() {
    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
    board.get_board_state();
}
