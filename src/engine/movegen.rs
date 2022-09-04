use super::{Bitboard, Board, Color, Piece};
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
                Self::get_wpawn_attacks(bitboard);
            }
            Color::Black => {
                //Self::get_black_pawn_attacks(bitboard);
            }
        };

        attacks
    }

    fn get_wpawn_attacks(pawns: Bitboard) -> Bitboard {
        let pawn_attacks =
            Self::get_wpawn_east_attacks(pawns) | Self::get_wpawn_west_attacks(pawns);

        pawn_attacks
    }

    fn get_wpawn_east_attacks(pawns: Bitboard) -> Bitboard {
        let mut east_attacks = pawns << 9;
        let no_a_file =
            Bitboard::new(0b1111111011111110111111101111111011111110111111101111111011111110);
        east_attacks &= no_a_file;

        east_attacks
    }

    fn get_wpawn_west_attacks(pawns: Bitboard) -> Bitboard {
        let mut west_attacks = pawns << 7;
        let no_h_file =
            Bitboard::new(0b0111111101111111011111110111111101111111011111110111111101111111);
        west_attacks &= no_h_file;

        west_attacks
    }
}
