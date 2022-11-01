use super::Square;
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Shl, Shr};

#[derive(Debug, Copy, Clone, PartialEq)]
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
        let mut board = String::from(" +---+---+---+---+---+---+---+---+\n");

        let mut square = 63;
        for rank in (1..9).rev() {
            board.push_str(&rank.to_string());
            for _ in 0..8 {
                if self.0 & (1u64 << square) == (1u64 << square) {
                    board.push_str("| X ")
                } else {
                    board.push_str("|   ")
                }
                square -= 1;
            }

            board.push_str("|\n +---+---+---+---+---+---+---+---+\n");
        }

        board.push_str("   A   B   C   D   E   F   G   H");

        write!(f, "{}", board)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 | other.0)
    }
}

impl BitOr for &Bitboard {
    type Output = Bitboard;

    fn bitor(self, other: &Bitboard) -> Bitboard {
        Bitboard(self.0 | other.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, other: Bitboard) {
        self.0 |= other.0
    }
}

impl BitOrAssign<&Bitboard> for Bitboard {
    fn bitor_assign(&mut self, other: &Bitboard) {
        self.0 |= other.0;
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 & other.0)
    }
}

impl BitAnd for &Bitboard {
    type Output = Bitboard;

    fn bitand(self, other: &Bitboard) -> Bitboard {
        Bitboard(self.0 & other.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, other: Bitboard) {
        self.0 &= other.0
    }
}

impl BitAndAssign<&Bitboard> for Bitboard {
    fn bitand_assign(&mut self, other: &Bitboard) {
        self.0 |= other.0;
    }
}

impl Shr<usize> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shl<usize> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self(self.0 << rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_new_bitboard() {
        let bitboard: Bitboard = Bitboard::new(10u64);

        assert_eq!(bitboard.0, 10u64);
    }

    #[test]
    fn it_creates_from_square() {
        let bitboard: Bitboard = Bitboard::from_square(Square::a1);

        assert_eq!(bitboard.0, 1u64);
    }

    #[test]
    fn it_creates_from_squares() {
        let squares: Vec<Square> = vec![Square::a1, Square::b1, Square::c1];
        let bitboard: Bitboard = Bitboard::from_squares(squares);

        // a1 + b1 + c1 = 2^0 + 2^1 + 2^2 = 1 + 2 + 4 = 7
        assert_eq!(bitboard.0, 7u64);
    }

    #[test]
    fn it_does_bit_or() {
        let a1 = Bitboard::from_square(Square::a1);
        let a2 = Bitboard::from_square(Square::a2);

        assert_eq!(
            a1 | a2,
            Bitboard::from_squares(vec![Square::a1, Square::a2])
        );
    }

    #[test]
    fn it_does_bit_or_assign() {
        let a1 = Bitboard::from_square(Square::a1);
        let mut a2 = Bitboard::from_square(Square::a2);
        a2 |= a1;

        assert_eq!(a2, Bitboard::from_squares(vec![Square::a1, Square::a2]));
    }

    #[test]
    fn it_does_bit_or_same_square() {
        let board1 = Bitboard::from_square(Square::h8);
        let board2 = Bitboard::from_square(Square::h8);

        assert_eq!(board1 | board2, Bitboard::from_square(Square::h8));
    }

    #[test]
    fn it_does_bit_or_assign_same_square() {
        let board1 = Bitboard::from_square(Square::d4);
        let mut board2 = Bitboard::from_square(Square::d4);
        board2 |= board1;

        assert_eq!(board2, Bitboard::from_square(Square::d4));
    }

    #[test]
    fn it_does_bit_and() {
        let board1 = Bitboard::from_square(Square::a1);
        let board2 = Bitboard::from_square(Square::a1);

        assert_eq!(board1 & board2, Bitboard::from_square(Square::a1));
    }

    #[test]
    fn it_does_bit_and_different_square() {
        let board1 = Bitboard::from_square(Square::a8);
        let board2 = Bitboard::from_square(Square::e3);

        assert_eq!(board1 & board2, Bitboard::new(0u64));
    }

    #[test]
    fn it_does_bit_and_assign() {
        let board1 = Bitboard::from_square(Square::c3);
        let mut board2 = Bitboard::from_square(Square::c3);
        board2 &= board1;

        assert_eq!(board2, Bitboard::from_square(Square::c3));
    }

    #[test]
    fn it_does_bit_and_assign_different_square() {
        let board1 = Bitboard::from_square(Square::f2);
        let mut board2 = Bitboard::from_square(Square::b5);
        board2 &= board1;

        assert_eq!(board2, Bitboard::new(0u64));
    }

    #[test]
    fn it_left_shifts_one_file() {
        let mut board = Bitboard::from_square(Square::a1);
        board = board << 1;

        assert_eq!(board, Bitboard::from_square(Square::b1))
    }

    #[test]
    fn it_left_shifts_to_next_row() {
        let mut board = Bitboard::from_square(Square::c5);
        board = board << 8;

        assert_eq!(board, Bitboard::from_square(Square::c6))
    }

    #[test]
    fn it_right_shifts_one_file() {
        let mut board = Bitboard::from_square(Square::h8);
        board = board >> 1;

        assert_eq!(board, Bitboard::from_square(Square::g8))
    }

    #[test]
    fn it_right_shifts_to_prev_row() {
        let mut board = Bitboard::from_square(Square::e5);
        board = board >> 8;

        assert_eq!(board, Bitboard::from_square(Square::e4))
    }
}
