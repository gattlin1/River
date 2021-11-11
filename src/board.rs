pub struct Board {
    white_king_bb: u64,
    white_queen_bb: u64,
    white_rook_bb: u64,
    white_bishop_bb: u64,
    white_knight_bb: u64,
    white_pawn_bb: u64,

    black_king_bb: u64,
    black_queen_bb: u64,
    black_rook_bb: u64,
    black_bishop_bb: u64,
    black_knight_bb: u64,
    black_pawn_bb: u64,
}

impl Board {
    pub fn new() -> Self {
        Self {
            white_king_bb: 0,
            white_queen_bb: 0,
            white_rook_bb: 0,
            white_bishop_bb: 0,
            white_knight_bb: 0,
            white_pawn_bb: 0,

            black_king_bb: 0,
            black_queen_bb: 0,
            black_rook_bb: 0,
            black_bishop_bb: 0,
            black_knight_bb: 0,
            black_pawn_bb: 0,
        }
    }
}
