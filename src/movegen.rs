use crate::Board;

pub struct MoveGen {
    board: Board,
    move_list: Vec<String>,
}

impl MoveGen {
    pub fn new(board: Board, move_list: Vec<String>) -> Self {
        Self { board, move_list }
    }
}
