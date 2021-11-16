use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use crate::enums::Square;

#[derive(Debug, Copy, Clone)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn new(board: u64) -> Self {
        Self(board)
    }

    pub fn from_square(square: Square) -> Self {
        Self(Self::square_to_bit(square))
    }

    pub fn from_squares(squares: Vec<Square>) -> Self {
        let mut board: u64 = 0;
        for square in squares {
            board += Self::square_to_bit(square);
        }

        Self(board)
    }

    fn square_to_bit(square: Square) -> u64 {
        u64::pow(2, (square as u8).into())
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut decimal: u64 = self.0;
        let mut bits = String::new();

        while decimal > 0 {
            for _ in 0..8 {
                if decimal % 2 == 0 {
                    bits.push('-');
                } else {
                    bits.push('X');
                }
                decimal /= 2;
            }
            bits.push('\n');
        }

        while bits.len() < 64 {
            bits.push_str("--------\n");
        }

        write!(f, "{}", bits)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 | other.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, other: Bitboard) {
        self.0 |= other.0
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 & other.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, other: Bitboard) {
        self.0 &= other.0
    }
}
