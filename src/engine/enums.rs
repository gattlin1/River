use num_derive::FromPrimitive;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Type {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Castling {
    WhiteKingSide,
    WhiteQueenSide,
    BlackKingSide,
    BlackQueenSide,
}

#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[derive(FromPrimitive, Debug)]
#[repr(u8)]
pub enum Square {
    a1 = 0, b1, c1, d1, e1, f1, g1, h1,
    a2, b2, c2, d2, e2, f2, g2, h2,
    a3, b3, c3, d3, e3, f3, g3, h3,
    a4, b4, c4, d4, e4, f4, g4, h4,
    a5, b5, c5, d5, e5, f5, g5, h5,
    a6, b6, c6, d6, e6, f6, g6, h6,
    a7, b7, c7, d7, e7, f7, g7, h7,
    a8, b8, c8, d8, e8, f8, g8, h8,
    None
}

impl FromStr for Square {
    type Err = ();

    fn from_str(input: &str) -> Result<Square, Self::Err> {
        match input {
            "a1" => Ok(Square::a1),
            "a2" => Ok(Square::a2),
            "a3" => Ok(Square::a3),
            "a4" => Ok(Square::a4),
            "a5" => Ok(Square::a5),
            "a6" => Ok(Square::a6),
            "a7" => Ok(Square::a7),
            "a8" => Ok(Square::a8),
            "b1" => Ok(Square::b1),
            "b2" => Ok(Square::b2),
            "b3" => Ok(Square::b3),
            "b4" => Ok(Square::b4),
            "b5" => Ok(Square::b5),
            "b6" => Ok(Square::b6),
            "b7" => Ok(Square::b7),
            "b8" => Ok(Square::b8),
            "c1" => Ok(Square::c1),
            "c2" => Ok(Square::c2),
            "c3" => Ok(Square::c3),
            "c4" => Ok(Square::c4),
            "c5" => Ok(Square::c5),
            "c6" => Ok(Square::c6),
            "c7" => Ok(Square::c7),
            "c8" => Ok(Square::c8),
            "d1" => Ok(Square::d1),
            "d2" => Ok(Square::d2),
            "d3" => Ok(Square::d3),
            "d4" => Ok(Square::d4),
            "d5" => Ok(Square::d5),
            "d6" => Ok(Square::d6),
            "d7" => Ok(Square::d7),
            "d8" => Ok(Square::d8),
            "e1" => Ok(Square::e1),
            "e2" => Ok(Square::e2),
            "e3" => Ok(Square::e3),
            "e4" => Ok(Square::e4),
            "e5" => Ok(Square::e5),
            "e6" => Ok(Square::e6),
            "e7" => Ok(Square::e7),
            "e8" => Ok(Square::e8),
            "f1" => Ok(Square::f1),
            "f2" => Ok(Square::f2),
            "f3" => Ok(Square::f3),
            "f4" => Ok(Square::f4),
            "f5" => Ok(Square::f5),
            "f6" => Ok(Square::f6),
            "f7" => Ok(Square::f7),
            "f8" => Ok(Square::f8),
            "g1" => Ok(Square::g1),
            "g2" => Ok(Square::g2),
            "g3" => Ok(Square::g3),
            "g4" => Ok(Square::g4),
            "g5" => Ok(Square::g5),
            "g6" => Ok(Square::g6),
            "g7" => Ok(Square::g7),
            "g8" => Ok(Square::g8),
            "h1" => Ok(Square::h1),
            "h2" => Ok(Square::h2),
            "h3" => Ok(Square::h3),
            "h4" => Ok(Square::h4),
            "h5" => Ok(Square::h5),
            "h6" => Ok(Square::h6),
            "h7" => Ok(Square::h7),
            "h8" => Ok(Square::h8),
            "-" => Ok(Square::None),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Rank {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum File {
    A = 1,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
