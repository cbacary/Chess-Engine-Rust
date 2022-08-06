#[derive(PartialEq)]
pub enum Flag {
    Exact,
    Upperbound,
    Lowerbound
}

pub struct ZobristReturn {
    pub value: f64,
    pub depth: u8,
    pub flag: Flag,
}
