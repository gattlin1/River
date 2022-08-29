use crate::enums::{Color, Type};

pub struct Piece {
    pub rank: Type,
    pub color: Color,
}

impl Piece {
    fn new(rank: Type, color: Color) -> Self {
        Self { rank, color }
    }
}
