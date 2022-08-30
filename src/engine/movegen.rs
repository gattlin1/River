use super::enums::Piece;
use super::Bitboard;
use super::Board;
use super::Color;

pub struct MoveGen {}

impl MoveGen {
    pub fn gen_moves(board: &Board) {
        Self::get_knight_moves(board.bitboards.get(&Piece::WhiteKnight).unwrap().clone());
        Self::get_king_moves(board.bitboards.get(&Piece::WhiteKing).unwrap().clone());
        Self::get_pawn_moves(
            board.bitboards.get(&Piece::WhiteKing).unwrap().clone(),
            board.active_color,
        );
    }

    pub fn get_king_moves(_bitboard: Bitboard) {}
    pub fn get_queen_moves(_bitboard: Bitboard) {}
    pub fn get_rook_moves(_bitboard: Bitboard) {}
    pub fn get_bishop_moves(_bitboard: Bitboard) {}
    pub fn get_knight_moves(_bitboard: Bitboard) {}
    pub fn get_pawn_moves(_bitboard: Bitboard, _color: Color) {}

    pub fn get_pawn_attacks(bitboard: Bitboard, color: Color) {
        let attacks = match color {
            Color::White => {
                Self::get_white_pawn_attacks(bitboard);
            }
            Color::Black => {
                Self::get_black_pawn_attacks(bitboard);
            }
        };

        attacks
    }

    fn get_white_pawn_attacks(bitboard: Bitboard) -> Bitboard {
        let mut attacks = Bitboard::new(0u64);
        let mut poss_attacks = Bitboard::new(0u64);
        poss_attacks |= bitboard << 7; // get right attacks
        poss_attacks |= bitboard << 2; // get left attacks

        attacks
    }

    fn get_black_pawn_attacks(bitboard: Bitboard) -> Bitboard {
        let mut attacks = Bitboard::new(0u64);
        attacks |= bitboard >> 7; // get left attacks
        attacks |= bitboard >> 2; // get right attacks

        attacks
    }
}
