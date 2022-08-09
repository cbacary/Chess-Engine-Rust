use chess::{ChessMove, Color};

#[derive(PartialEq)]
pub enum Flag {
    Exact,
    Upperbound,
    Lowerbound,
}

pub struct ZobristReturn {
    pub value: f64,
    pub depth: u8,
    pub best_move: ChessMove,
    pub color_found: Color,
    pub flag: Flag
}
