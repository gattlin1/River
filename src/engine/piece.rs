#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum Piece {
    WhiteKing = b'K',
    WhiteQueen = b'Q',
    WhiteRook = b'R',
    WhiteBishop = b'B',
    WhiteKnight = b'N',
    WhitePawn = b'P',
    BlackKing = b'k',
    BlackQueen = b'q',
    BlackRook = b'r',
    BlackBishop = b'b',
    BlackKnight = b'n',
    BlackPawn = b'p',
}

impl Piece {
    pub fn to_char(self) -> char {
        self as u8 as char
    }

    pub fn from(ch: char) -> Result<Piece, char> {
        match ch {
            'k' => Ok(Piece::BlackKing),
            'q' => Ok(Piece::BlackQueen),
            'r' => Ok(Piece::BlackRook),
            'b' => Ok(Piece::BlackBishop),
            'n' => Ok(Piece::BlackKnight),
            'p' => Ok(Piece::BlackPawn),
            'K' => Ok(Piece::WhiteKing),
            'Q' => Ok(Piece::WhiteQueen),
            'R' => Ok(Piece::WhiteRook),
            'B' => Ok(Piece::WhiteBishop),
            'N' => Ok(Piece::WhiteKnight),
            'P' => Ok(Piece::WhitePawn),
            _ => Err(ch),
        }
    }
}
