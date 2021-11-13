use std::fmt;
use std::ops::{BitAnd, BitOr, BitOrAssign};

use crate::enums::{File, Rank};

#[derive(Debug, Copy, Clone)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn new(bb: u64) -> Self {
        Self(bb)
    }

    pub fn from_square(file: File, rank: Rank) -> Self {
        Self(Self::square_to_bit(file, rank))
    }

    fn square_to_bit(file: File, rank: Rank) -> u64 {
        u64::pow(2, (file as u8 - 1).into()) + u64::pow(2, ((rank as u8 - 1) * 8).into())
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut decimal: u64 = self.0;
        let mut bits = String::new();

        while decimal > 0 {
            for _ in 0..8 {
                if decimal % 2 == 0 {
                    bits.push('0');
                } else {
                    bits.push('1');
                }
                decimal /= 2;
            }
            bits.push('\n');
        }

        while bits.len() < 64 {
            bits.push_str("00000000\n");
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
