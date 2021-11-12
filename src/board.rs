use crate::Bitboard;
use crate::Pieces;
use std::{collections::HashMap, ops::BitOr};

pub struct Board {
    bitboards: HashMap<Pieces, Bitboard>,
}

impl Board {
    pub fn new() -> Self {
        let mut initial_board_state: HashMap<Pieces, Bitboard> = HashMap::new();
        initial_board_state.insert(Pieces::WhiteKing, Bitboard::new(u64::pow(2, 4)));
        initial_board_state.insert(Pieces::WhiteQueen, Bitboard::new(u64::pow(2, 3)));
        initial_board_state.insert(
            Pieces::WhiteRook,
            Bitboard::new(u64::pow(2, 0) + u64::pow(2, 7)),
        );
        initial_board_state.insert(
            Pieces::WhiteBishop,
            Bitboard::new(u64::pow(2, 1) + u64::pow(2, 6)),
        );
        initial_board_state.insert(
            Pieces::WhiteKnight,
            Bitboard::new(u64::pow(2, 2) + u64::pow(2, 5)),
        );
        initial_board_state.insert(
            Pieces::WhitePawn,
            Bitboard::new(
                u64::pow(2, 8)
                    + u64::pow(2, 9)
                    + u64::pow(2, 10)
                    + u64::pow(2, 11)
                    + u64::pow(2, 12)
                    + u64::pow(2, 13)
                    + u64::pow(2, 14)
                    + u64::pow(2, 15),
            ),
        );

        initial_board_state.insert(Pieces::BlackKing, Bitboard::new(u64::pow(2, 60)));
        initial_board_state.insert(Pieces::BlackQueen, Bitboard::new(u64::pow(2, 59)));
        initial_board_state.insert(
            Pieces::BlackRook,
            Bitboard::new(u64::pow(2, 56) + u64::pow(2, 63)),
        );
        initial_board_state.insert(
            Pieces::BlackBishop,
            Bitboard::new(u64::pow(2, 58) + u64::pow(2, 61)),
        );
        initial_board_state.insert(
            Pieces::BlackKnight,
            Bitboard::new(u64::pow(2, 57) + u64::pow(2, 62)),
        );
        initial_board_state.insert(
            Pieces::BlackPawn,
            Bitboard::new(
                u64::pow(2, 48)
                    + u64::pow(2, 49)
                    + u64::pow(2, 50)
                    + u64::pow(2, 51)
                    + u64::pow(2, 52)
                    + u64::pow(2, 53)
                    + u64::pow(2, 54)
                    + u64::pow(2, 55),
            ),
        );

        Self {
            bitboards: initial_board_state,
        }
    }

    pub fn get_board_state(self) -> Bitboard {
        let mut result: Bitboard = Bitboard::new(0);

        for (_, board) in self.bitboards {
            result = board.bitor(result);
        }

        return result;
    }
}
