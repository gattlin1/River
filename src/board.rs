use crate::enums::{Piece, Square};
use crate::Bitboard;
use num_traits::FromPrimitive;
use std::collections::HashMap;

pub struct Board {
    bitboards: HashMap<Piece, Bitboard>,
}

impl Board {
    pub fn new() -> Self {
        let assigned_piece_squares = vec![
            (Piece::WhiteKing, vec![Square::e1]),
            (Piece::WhiteQueen, vec![Square::d1]),
            (Piece::WhiteRook, vec![Square::a1, Square::h1]),
            (Piece::WhiteBishop, vec![Square::c1, Square::f1]),
            (Piece::WhiteKnight, vec![Square::b1, Square::g1]),
            (
                Piece::WhitePawn,
                vec![
                    Square::a2,
                    Square::b2,
                    Square::c2,
                    Square::d2,
                    Square::e2,
                    Square::f2,
                    Square::g2,
                    Square::h2,
                ],
            ),
            (Piece::BlackKing, vec![Square::e8]),
            (Piece::BlackQueen, vec![Square::d8]),
            (Piece::BlackRook, vec![Square::a8, Square::h8]),
            (Piece::BlackBishop, vec![Square::c8, Square::f8]),
            (Piece::BlackKnight, vec![Square::b8, Square::g8]),
            (
                Piece::BlackPawn,
                vec![
                    Square::a7,
                    Square::b7,
                    Square::c7,
                    Square::d7,
                    Square::e7,
                    Square::f7,
                    Square::g7,
                    Square::h7,
                ],
            ),
        ];
        let mut initial_board_state: HashMap<Piece, Bitboard> = HashMap::new();
        for (piece, squares) in assigned_piece_squares {
            initial_board_state.insert(piece, Bitboard::from_squares(squares));
        }

        Self {
            bitboards: initial_board_state,
        }
    }

    pub fn from_fen(fen: String) -> Self {
        let mut piece_list: HashMap<Piece, Vec<Square>> = HashMap::new();
        let mut bitboards: HashMap<Piece, Bitboard> = HashMap::new();
        let fen_pieces: Vec<&str> = fen.split_whitespace().collect();
        let ranks: Vec<&str> = fen_pieces[0].split("/").collect();
        let active_color = fen_pieces[1];
        let castling_rights = fen_pieces[2];
        let en_passant = fen_pieces[3];
        let halfmove_clock: i32 = fen_pieces[4].parse().unwrap();
        let fullmove_number: i32 = fen_pieces[5].parse().unwrap();

        println!("{:?}", fen_pieces);

        let mut square: u8 = 63;
        for pieces in ranks {
            for piece in pieces.chars() {
                match piece {
                    'k' => Self::add_to_piece_list(&mut piece_list, Piece::BlackKing, square),
                    'q' => Self::add_to_piece_list(&mut piece_list, Piece::BlackQueen, square),
                    'r' => Self::add_to_piece_list(&mut piece_list, Piece::BlackRook, square),
                    'b' => Self::add_to_piece_list(&mut piece_list, Piece::BlackBishop, square),
                    'n' => Self::add_to_piece_list(&mut piece_list, Piece::BlackKnight, square),
                    'p' => Self::add_to_piece_list(&mut piece_list, Piece::BlackPawn, square),
                    'K' => Self::add_to_piece_list(&mut piece_list, Piece::WhiteKing, square),
                    'Q' => Self::add_to_piece_list(&mut piece_list, Piece::WhiteQueen, square),
                    'R' => Self::add_to_piece_list(&mut piece_list, Piece::WhiteRook, square),
                    'B' => Self::add_to_piece_list(&mut piece_list, Piece::WhiteBishop, square),
                    'N' => Self::add_to_piece_list(&mut piece_list, Piece::WhiteKnight, square),
                    'P' => Self::add_to_piece_list(&mut piece_list, Piece::WhitePawn, square),
                    _ => {}
                };
                square -= 1;
            }
        }

        println!("{:#?}", piece_list);

        for (piece, squares) in piece_list {
            bitboards.insert(piece, Bitboard::from_squares(squares));
        }

        Self { bitboards }
    }

    fn add_to_piece_list(piece_list: &mut HashMap<Piece, Vec<Square>>, piece: Piece, square: u8) {
        let piece_squares = piece_list.entry(piece).or_insert(vec![]);
        piece_squares.push(FromPrimitive::from_u8(square).unwrap());
    }

    pub fn get_board_state(self) -> Bitboard {
        let mut result: Bitboard = Bitboard::new(0);

        for (_, board) in self.bitboards {
            result |= board;
        }

        return result;
    }
}
