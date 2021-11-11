use std::ops::{BitOr, BitXor};

use crate::Bitboard;

pub struct Board {
    board_state: Vec<Bitboard>,
    white_king_bb: Bitboard,
    white_queen_bb: Bitboard,
    white_rook_bb: Bitboard,
    white_bishop_bb: Bitboard,
    white_knight_bb: Bitboard,
    white_pawn_bb: Bitboard,

    black_king_bb: Bitboard,
    black_queen_bb: Bitboard,
    black_rook_bb: Bitboard,
    black_bishop_bb: Bitboard,
    black_knight_bb: Bitboard,
    black_pawn_bb: Bitboard,
}

impl Board {
    pub fn new() -> Self {
        Self {
            white_king_bb: Bitboard::new(u64::pow(2, 4)),
            white_queen_bb: Bitboard::new(u64::pow(2, 3)),
            white_rook_bb: Bitboard::new(u64::pow(2, 0) + u64::pow(2, 7)),
            white_bishop_bb: Bitboard::new(u64::pow(2, 1) + u64::pow(2, 6)),
            white_knight_bb: Bitboard::new(u64::pow(2, 2) + u64::pow(2, 5)),
            white_pawn_bb: Bitboard::new(
                u64::pow(2, 8)
                    + u64::pow(2, 9)
                    + u64::pow(2, 10)
                    + u64::pow(2, 11)
                    + u64::pow(2, 12)
                    + u64::pow(2, 13)
                    + u64::pow(2, 14)
                    + u64::pow(2, 15),
            ),

            black_king_bb: Bitboard::new(u64::pow(2, 60)),
            black_queen_bb: Bitboard::new(u64::pow(2, 59)),
            black_rook_bb: Bitboard::new(u64::pow(2, 56) + u64::pow(2, 63)),
            black_bishop_bb: Bitboard::new(u64::pow(2, 58) + u64::pow(2, 61)),
            black_knight_bb: Bitboard::new(u64::pow(2, 57) + u64::pow(2, 62)),
            black_pawn_bb: Bitboard::new(
                u64::pow(2, 48)
                    + u64::pow(2, 49)
                    + u64::pow(2, 50)
                    + u64::pow(2, 51)
                    + u64::pow(2, 52)
                    + u64::pow(2, 53)
                    + u64::pow(2, 54)
                    + u64::pow(2, 55),
            ),
            board_state: vec![],
        }
    }

    pub fn get_board_state(&mut self) -> Bitboard {
        let mut result: Bitboard = Bitboard::new(0);
        self.board_state = vec![
            self.white_king_bb,
            self.white_queen_bb,
            self.white_rook_bb,
            self.white_bishop_bb,
            self.white_knight_bb,
            self.white_pawn_bb,
            self.black_king_bb,
            self.black_queen_bb,
            self.black_rook_bb,
            self.black_bishop_bb,
            self.black_knight_bb,
            self.black_pawn_bb,
        ];

        for board in self.board_state.iter() {
            result = board.bitor(result);
        }

        return result;
    }
}
