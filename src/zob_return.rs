use chess::ChessMove;

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
}
