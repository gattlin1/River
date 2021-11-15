use crate::enums::Squares;
use crate::Bitboard;
use crate::Pieces;
use std::collections::HashMap;

pub struct Board {
    bitboards: HashMap<Pieces, Bitboard>,
}

impl Board {
    pub fn new() -> Self {
        let assigned_piece_squares = vec![
            (Pieces::WhiteKing, vec![Squares::e1]),
            (Pieces::WhiteQueen, vec![Squares::d1]),
            (Pieces::WhiteRook, vec![Squares::a1, Squares::h1]),
            (Pieces::WhiteBishop, vec![Squares::c1, Squares::f1]),
            (Pieces::WhiteKnight, vec![Squares::b1, Squares::g1]),
            (
                Pieces::WhitePawn,
                vec![
                    Squares::a2,
                    Squares::b2,
                    Squares::c2,
                    Squares::d2,
                    Squares::e2,
                    Squares::f2,
                    Squares::g2,
                    Squares::h2,
                ],
            ),
            (Pieces::BlackKing, vec![Squares::e8]),
            (Pieces::BlackQueen, vec![Squares::d8]),
            (Pieces::BlackRook, vec![Squares::a8, Squares::h8]),
            (Pieces::BlackBishop, vec![Squares::c8, Squares::f8]),
            (Pieces::BlackKnight, vec![Squares::b8, Squares::g8]),
            (
                Pieces::BlackPawn,
                vec![
                    Squares::a7,
                    Squares::b7,
                    Squares::c7,
                    Squares::d7,
                    Squares::e7,
                    Squares::f7,
                    Squares::g7,
                    Squares::h7,
                ],
            ),
        ];
        let mut initial_board_state: HashMap<Pieces, Bitboard> = HashMap::new();
        for (piece, squares) in assigned_piece_squares {
            initial_board_state.insert(piece, Bitboard::from_squares(squares));
        }

        Self {
            bitboards: initial_board_state,
        }
    }

    pub fn get_board_state(self) -> Bitboard {
        let mut result: Bitboard = Bitboard::new(0);

        for (_, board) in self.bitboards {
            result |= board;
        }

        return result;
    }
}
