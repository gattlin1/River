use super::{Bitboard, Castling, Color, MoveGen, Piece, Square};
use num_traits::FromPrimitive;
use std::{collections::HashMap, str::FromStr};

pub struct Board {
    pub bitboards: HashMap<Piece, Bitboard>,
    pub white: Bitboard,
    pub black: Bitboard,
    pub active_color: Color,
    pub castling_rights: HashMap<Castling, bool>,
    pub en_passant: Square,
    pub halfmove_clock: i32,
    pub fullmove_number: i32,
}

impl Board {
    pub fn new() -> Self {
        let starting_pos = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0";
        Self::from_fen(starting_pos)
    }

    pub fn from_fen(fen: &str) -> Self {
        let fen_pieces: Vec<&str> = fen.split_whitespace().collect();

        let bitboards = Self::fen_get_bitboards(fen_pieces[0].split("/").collect());
        let active_color = Self::fen_get_active_color(fen_pieces[1]).unwrap();
        let castling_rights = Self::fen_get_castling_rights(fen_pieces[2]);
        let en_passant = Self::fen_get_en_passant(fen_pieces[3]);
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
                match Piece::from(piece) {
                    Ok(piece) => Self::add_to_piece_list(&mut piece_list, piece, square),
                    Err(non_piece) => {
                        match non_piece {
                            '1'..='8' => {
                                let empty_squares = piece.to_digit(10).unwrap() as i8;
                                square -= empty_squares - 1; // -1 because we subtract after the match statement
                            }
                            _ => {}
                        }
                    }
                }
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

    fn fen_get_en_passant(en_passant_square: &str) -> Square {
        if en_passant_square == "-" {
            return Square::None;
        }

        match Square::from_str(en_passant_square) {
            Ok(square) => square,
            _ => panic!("Unknown en passant square was supplied"),
        }
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

        let mut board_rep = String::from(" +---+---+---+---+---+---+---+---+\n");

        let mut square = 63;
        for rank in (1..9).rev() {
            board_rep.push_str(&rank.to_string());
            for _ in 0..8 {
                let mut piece_rep = ' ';
                for (piece, board) in self.bitboards.iter() {
                    if board.0 & (1u64 << square) == (1u64 << square) {
                        piece_rep = piece.to_char();
                        break;
                    }
                }
                board_rep.push_str(&format!("| {} ", piece_rep.to_string()));
                square -= 1;
            }

            board_rep.push_str("|\n +---+---+---+---+---+---+---+---+\n");
        }

        board_rep.push_str("   A   B   C   D   E   F   G   H");

        println!("{}", board_rep);
    }

    pub fn move_gen(&self) {
        MoveGen::gen_moves(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_starting_position() {
        let board = Board::new();

        let mut castling_rights: HashMap<Castling, bool> = HashMap::new();
        castling_rights.insert(Castling::WhiteKingSide, true);
        castling_rights.insert(Castling::WhiteQueenSide, true);
        castling_rights.insert(Castling::BlackKingSide, true);
        castling_rights.insert(Castling::BlackQueenSide, true);

        assert_eq!(
            board.white,
            Bitboard::from_squares(vec![
                Square::a1,
                Square::b1,
                Square::c1,
                Square::d1,
                Square::e1,
                Square::f1,
                Square::g1,
                Square::h1,
                Square::a2,
                Square::b2,
                Square::c2,
                Square::d2,
                Square::e2,
                Square::f2,
                Square::g2,
                Square::h2,
            ])
        );
        assert_eq!(
            board.black,
            Bitboard::from_squares(vec![
                Square::a8,
                Square::b8,
                Square::c8,
                Square::d8,
                Square::e8,
                Square::f8,
                Square::g8,
                Square::h8,
                Square::a7,
                Square::b7,
                Square::c7,
                Square::d7,
                Square::e7,
                Square::f7,
                Square::g7,
                Square::h7,
            ])
        );
        assert_eq!(board.active_color, Color::White);
        assert_eq!(board.castling_rights, castling_rights);
        assert_eq!(board.en_passant, Square::None);
        assert_eq!(board.halfmove_clock, 0);
        assert_eq!(board.fullmove_number, 0);
    }

    #[test]
    fn it_parses_white_active_color() {
        let white = Board::fen_get_active_color("w").unwrap();
        let black = Board::fen_get_active_color("b").unwrap();

        assert_eq!(white, Color::White);
        assert_eq!(black, Color::Black);
    }

    #[test]
    fn it_parses_en_passant_square() {
        let no_en_passant = Board::fen_get_en_passant("-");
        let g4_en_passant = Board::fen_get_en_passant("g4");

        assert_eq!(no_en_passant, Square::None);
        assert_eq!(g4_en_passant, Square::g4);
    }

    #[test]
    fn it_parses_castling_rights() {
        let castling_rights = Board::fen_get_castling_rights("KQkq");

        let mut expected: HashMap<Castling, bool> = HashMap::new();
        expected.insert(Castling::WhiteKingSide, true);
        expected.insert(Castling::WhiteQueenSide, true);
        expected.insert(Castling::BlackKingSide, true);
        expected.insert(Castling::BlackQueenSide, true);

        assert_eq!(castling_rights, expected)
    }
}
