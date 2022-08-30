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
                //Self::get_black_pawn_attacks(bitboard);
            }
        };

        attacks
    }

    fn get_white_pawn_attacks(bitboard: Bitboard) -> Bitboard {
        let pawn_attacks = Self::get_white_pawn_east_attacks(bitboard)
            | Self::get_white_pawn_west_attacks(bitboard);

        pawn_attacks
    }

    fn get_white_pawn_east_attacks(bitboard: Bitboard) -> Bitboard {
        let mut east_attacks = bitboard << 9;
        let no_a_file =
            Bitboard::new(0b1111111011111110111111101111111011111110111111101111111011111110);
        east_attacks &= no_a_file;

        east_attacks
    }

    fn get_white_pawn_west_attacks(bitboard: Bitboard) -> Bitboard {
        let mut west_attacks = bitboard << 7;
        let no_h_file =
            Bitboard::new(0b0111111101111111011111110111111101111111011111110111111101111111);
        west_attacks &= no_h_file;

        west_attacks
    }
}
