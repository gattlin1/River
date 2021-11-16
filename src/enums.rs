use num_derive::FromPrimitive;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Piece {
    WhiteKing,
    WhiteQueen,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    WhitePawn,
    BlackKing,
    BlackQueen,
    BlackRook,
    BlackBishop,
    BlackKnight,
    BlackPawn,
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
#[derive(FromPrimitive)]
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
}

// #[derive(Copy, Clone, Debug)]
// #[repr(u8)]
// pub enum Rank {
//     One = 0,
//     Two,
//     Three,
//     Four,
//     Five,
//     Six,
//     Seven,
//     Eight,
// }
//
// #[derive(Copy, Clone, Debug)]
// #[repr(u8)]
// pub enum File {
//     A = 0,
//     B,
//     C,
//     D,
//     E,
//     F,
//     G,
//     H,
// }
