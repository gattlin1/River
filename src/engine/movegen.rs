#[allow(dead_code)]
#[allow(unused_variables)]
use crate::engine::enums::{Piece, Square};
use crate::engine::Bitboard;
use crate::engine::Board;

#[allow(dead_code)]
pub struct MoveGen {}

impl MoveGen {
    pub fn gen_moves(board: &Board) {
        Self::get_king_moves(board.bitboards.get(&Piece::WhiteKing).unwrap());
        Self::get_knight_moves(board.bitboards.get(&Piece::WhiteKnight).unwrap());
    }

    pub fn get_king_moves(bitboard: &Bitboard) {}
    pub fn get_queen_moves(bitboard: &Bitboard) {}
    pub fn get_rook_moves(bitboard: &Bitboard) {}
    pub fn get_bishop_moves(bitboard: &Bitboard) {}
    pub fn get_knight_moves(bitboard: &Bitboard) {}
    pub fn get_pawn_moves(bitboard: &Bitboard) {}
}
