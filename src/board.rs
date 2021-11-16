use crate::enums::{Piece, Square};
use crate::Bitboard;
use num_traits::FromPrimitive;
use std::collections::HashMap;

pub struct Board {
    bitboards: HashMap<Piece, Bitboard>,
    active_color: String,
    castling_rights: String,
    en_passant: String, // TODO: make a Square
    halfmove_clock: i32,
    fullmove_number: i32,
}

impl Board {
    pub fn new() -> Self {
        let starting_pos = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into();
        Self::from_fen(starting_pos)
    }

    pub fn from_fen(fen: String) -> Self {
        let mut piece_list: HashMap<Piece, Vec<Square>> = HashMap::new();
        let mut bitboards: HashMap<Piece, Bitboard> = HashMap::new();

        let fen_pieces: Vec<&str> = fen.split_whitespace().collect();
        let ranks: Vec<&str> = fen_pieces[0].split("/").collect();
        let active_color = fen_pieces[1].to_string();
        let castling_rights = fen_pieces[2].to_string();
        let en_passant = fen_pieces[3].to_string();
        let halfmove_clock: i32 = fen_pieces[4].parse().unwrap();
        let fullmove_number: i32 = fen_pieces[5].parse().unwrap();

        let mut square: i8 = 63;
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
                    '1'..='8' => {
                        let empty_squares = piece.to_digit(10).unwrap() as i8;
                        square -= empty_squares - 1; // -1 because we subtract after the match statement
                    }
                    _ => {}
                };
                square -= 1;
            }
        }

        for (piece, squares) in piece_list {
            bitboards.insert(piece, Bitboard::from_squares(squares));
        }

        Self {
            bitboards,
            active_color,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_number,
        }
    }

    fn add_to_piece_list(piece_list: &mut HashMap<Piece, Vec<Square>>, piece: Piece, square: i8) {
        let piece_squares = piece_list.entry(piece).or_insert(vec![]);
        piece_squares.push(FromPrimitive::from_i8(square).unwrap());
    }

    pub fn get_board_state(self) -> Bitboard {
        let mut result: Bitboard = Bitboard::new(0);

        for (_, board) in self.bitboards {
            result |= board;
        }

        return result;
    }
}
