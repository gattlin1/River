mod bitboard;
mod board;
mod enums;
mod piece;
use bitboard::*;
use board::*;
use piece::*;

fn main() {
    let board =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".into());
    println!("Color to Move: {:?}", board.active_color);
    println!("Castling Rights: {:?}", board.castling_rights);
    println!("En Passant Square: {:?}", board.en_passant);
    println!("Half Move #: {}", board.halfmove_clock);
    println!("Full Move #: {}", board.fullmove_number);
    println!("Initial Board State\n{}", board.get_board_state());
}
