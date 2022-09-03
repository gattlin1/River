use crate::engine::enums::{Castling, Color, Piece, Square};
use crate::engine::movegen::MoveGen;
use crate::engine::Bitboard;
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Board {
    pub bitboards: HashMap<Piece, Bitboard>,
    pub white: Bitboard,
    pub black: Bitboard,
    pub active_color: Color,
    pub castling_rights: HashMap<Castling, bool>,
    pub en_passant: Result<Square, ()>,
    pub halfmove_clock: i32,
    pub fullmove_number: i32,
}

impl Board {
    pub fn new() -> Self {
        let starting_pos = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        Self::from_fen(starting_pos)
    }

    pub fn from_fen(fen: &str) -> Self {
        let fen_pieces: Vec<&str> = fen.split_whitespace().collect();

        let bitboards = Self::fen_get_bitboards(fen_pieces[0].split("/").collect());
        let active_color = Self::fen_get_active_color(fen_pieces[1]).unwrap();
        let castling_rights = Self::fen_get_castling_rights(fen_pieces[2]);
        let en_passant = Square::from_str(fen_pieces[3]);
        let halfmove_clock: i32 = fen_pieces[4].parse().unwrap();
        let fullmove_number: i32 = fen_pieces[5].parse().unwrap();

        let mut white = Bitboard::new(0);
        let mut black = Bitboard::new(0);

        for (piece, board) in bitboards.iter() {
            match *piece {
                Piece::WhiteKing
                | Piece::WhiteQueen
                | Piece::WhiteRook
                | Piece::WhiteBishop
                | Piece::WhiteKnight
                | Piece::WhitePawn => white |= board,

                Piece::BlackKing
                | Piece::BlackQueen
                | Piece::BlackRook
                | Piece::BlackBishop
                | Piece::BlackKnight
                | Piece::BlackPawn => black |= board,
            }
        }

        Self {
            bitboards,
            white,
            black,
            active_color,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_number,
        }
    }

    fn fen_get_bitboards(fen_pieces: Vec<&str>) -> HashMap<Piece, Bitboard> {
        let mut bitboards: HashMap<Piece, Bitboard> = HashMap::new();
        let mut piece_list: HashMap<Piece, Vec<Square>> = HashMap::new();

        let mut square: i8 = 63;
        for pieces in fen_pieces {
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

        bitboards
    }

    fn add_to_piece_list(piece_list: &mut HashMap<Piece, Vec<Square>>, piece: Piece, square: i8) {
        let piece_squares = piece_list.entry(piece).or_insert(vec![]);
        piece_squares.push(FromPrimitive::from_i8(square).unwrap());
    }

    fn fen_get_castling_rights(fen_castling: &str) -> HashMap<Castling, bool> {
        let mut castling_rights: HashMap<Castling, bool> = HashMap::new();
        castling_rights.insert(Castling::WhiteKingSide, false);
        castling_rights.insert(Castling::WhiteQueenSide, false);
        castling_rights.insert(Castling::BlackKingSide, false);
        castling_rights.insert(Castling::BlackQueenSide, false);

        for individual_castling in fen_castling.split("") {
            match individual_castling {
                "K" => {
                    castling_rights.insert(Castling::WhiteKingSide, true);
                }
                "Q" => {
                    castling_rights.insert(Castling::WhiteQueenSide, true);
                }
                "k" => {
                    castling_rights.insert(Castling::BlackKingSide, true);
                }
                "q" => {
                    castling_rights.insert(Castling::BlackQueenSide, true);
                }
                _ => {}
            }
        }

        castling_rights
    }

    fn fen_get_active_color(fen_color: &str) -> Result<Color, &str> {
        match fen_color {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => {
                panic!("Unknown active color was supplied")
            }
        }
    }

    pub fn get_board_state(self) {
        println!("Color to Move: {:?}", self.active_color);
        println!("Castling Rights: {:?}", self.castling_rights);
        println!("En Passant Square: {:?}", self.en_passant);
        println!("Half Move #: {}", self.halfmove_clock);
        println!("Full Move #: {}", self.fullmove_number);

        for (piece, board) in self.bitboards.iter() {
            println!("{:?}\n {}", piece, board);
        }
    }

    pub fn move_gen(&self) {
        MoveGen::gen_moves(self);
    }
}
