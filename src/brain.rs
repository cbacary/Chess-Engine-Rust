use std::thread::current;

use chess::{Board, Square, MoveGen, ChessMove, Color, Piece, EMPTY, File};

static PAWN_VALUES: &'static [f64] =    &[0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
                                        5.0,  5.0,  5.0,  5.0,  5.0,  5.0,  5.0,  5.0,
                                        1.0,  1.0,  2.0,  3.0,  3.0,  2.0,  1.0,  1.0,
                                        0.5,  0.5,  1.0,  2.5,  2.5,  1.0,  0.5,  0.5,
                                        0.0,  0.0,  0.0,  2.0,  2.0,  0.0,  0.0,  0.0,
                                        0.5, -0.5, -1.0,  0.0,  0.0, -1.0, -0.5,  0.5,
                                        0.5,  1.0, 1.0,  -2.0, -2.0,  1.0,  1.0,  0.5,
                                        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0];
static KNIGHT_VALUES: &'static [f64] =  &[-5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0,
                                        -4.0, -2.0,  0.0,  0.0,  0.0,  0.0, -2.0, -4.0,
                                        -3.0,  0.0,  1.0,  1.5,  1.5,  1.0,  0.0, -3.0,
                                        -3.0,  0.5,  1.5,  2.0,  2.0,  1.5,  0.5, -3.0,
                                        -3.0,  0.0,  1.5,  2.0,  2.0,  1.5,  0.0, -3.0,
                                        -3.0,  0.5,  1.0,  1.5,  1.5,  1.0,  0.5, -3.0,
                                        -4.0, -2.0,  0.0,  0.5,  0.5,  0.0, -2.0, -4.0,
                                        -5.0, -4.0, -3.0, -3.0, -3.0, -3.0, -4.0, -5.0];
static BISHOP_VALUES: &'static [f64] =  &[-2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0,
                                        -1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0,
                                        -1.0,  0.0,  0.5,  1.0,  1.0,  0.5,  0.0, -1.0,
                                        -1.0,  0.5,  0.5,  1.0,  1.0,  0.5,  0.5, -1.0,
                                        -1.0,  0.0,  1.0,  1.0,  1.0,  1.0,  0.0, -1.0,
                                        -1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0, -1.0,
                                        -1.0,  0.5,  0.0,  0.0,  0.0,  0.0,  0.5, -1.0,
                                        -2.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -2.0];
static ROOK_VALUES: &'static [f64] =    &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,  0.0,
                                        0.5, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,  0.5,
                                        -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
                                        -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
                                        -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
                                        -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
                                        -0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
                                        0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0,  0.0];
static QUEEN_VALUES: &'static [f64] =   &[-2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0,
                                        -1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0,
                                        -1.0,  0.0,  0.5,  0.5,  0.5,  0.5,  0.0, -1.0,
                                        -0.5,  0.0,  0.5,  0.5,  0.5,  0.5,  0.0, -0.5,
                                        -0.5,  0.0,  0.5,  0.5,  0.5,  0.5,  0.0, -0.5,
                                        -1.0,  0.5,  0.5,  0.5,  0.5,  0.5,  0.0, -1.0,
                                        -1.0,  0.0,  0.5,  0.0,  0.0,  0.0,  0.0, -1.0,
                                        -2.0, -1.0, -1.0, -0.5, -0.5, -1.0, -1.0, -2.0];
static KING_VALUES: &'static [f64] =    &[-3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0,
                                        -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0,
                                        -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0,
                                        -3.0, -4.0, -4.0, -5.0, -5.0, -4.0, -4.0, -3.0,
                                        -2.0, -3.0, -3.0, -4.0, -4.0, -3.0, -3.0, -2.0,
                                        -1.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -1.0,
                                        2.0,  2.0,  0.0,  0.0,  0.0,  0.0,  2.0,  2.0,
                                        2.0,  3.0,  1.0,  0.0,  0.0,  1.0,  3.0,  2.0];

const INFINITY: f64 = f64::INFINITY;

pub static mut count: i32 = 0;
pub static mut pruned: i32 = 0;

pub fn generate_pgn(board: &Board, chess_move: &Option<ChessMove>, color: Color, current_pgn: &String, move_number: i32) -> String{
    match *chess_move {
        Some(i) => {

                // Check white or black, create beginning
            let beginning = match color {
                Color::White => {
                    let move_num = (move_number / 2) + 1 as i32;
                    format!("{move_num}. ")
                },
                Color::Black => {format!(" ")}
            };

            // Check if castle
            if i.get_source() == Square::E1 && i.get_dest() == Square::G1 && board.piece_on(Square::E1) == Some(Piece::King) {
                let full_string = format!("{beginning}o-o");
                let pgn = format!("{current_pgn}{full_string}");
                return pgn;
            } else if i.get_source() == Square::E8 && i.get_dest() == Square::G8 && board.piece_on(Square::E8) == Some(Piece::King) {
                let full_string = format!("{beginning}o-o");
                let pgn = format!("{current_pgn}{full_string}");
                return pgn;
            } else if i.get_source() == Square::E1 && i.get_dest() == Square::C1 && board.piece_on(Square::E1) == Some(Piece::King) {
                let full_string = format!("{beginning}o-o-o");
                let pgn = format!("{current_pgn}{full_string}");
                return pgn;
            } else if i.get_source() == Square::E8 && i.get_dest() == Square::C8 && board.piece_on(Square::E8) == Some(Piece::King) {
                let full_string = format!("{beginning}o-o-o");
                let pgn = format!("{current_pgn}{full_string}");
                return pgn;
            } 

            let destination = i.get_dest().to_string();

            // Get file
            let file = match i.get_source().get_file() {
                File::A => "a",
                File::B => "b",
                File::C => "c",
                File::D => "d",
                File::E => "e",
                File::F => "f",
                File::G => "g",
                File::H => "h",
                _ => ""
            };

            let file = file.to_owned();

            // Get piece being moved
            let piece_str = match board.piece_on(i.get_source()) {
                Some(Piece::Pawn) => "".to_owned(),
                Some(i) => {
                    format!("{i}").to_uppercase()
                },
                _ => "".to_owned()
            };

            // Check if a capture
            let capture = match board.piece_on(i.get_dest()) {
                Some(i) => "x",
                None => "",
            };

            // Make move
            let new_board = board.make_move_new(i);

            // Check if a check
            let checkers = new_board.checkers();
            let mut check = "".to_owned();
            if checkers.0 > 0 {
                check = "+".to_owned();
            }
            let full_string = format!("{beginning}{piece_str}{file}{capture}{destination}{check} ");

            let pgn = format!("{current_pgn}{full_string}");

            return pgn;
        },
        None => {println!("error"); return String::from("")} 
    }
    

}

#[derive(Debug)]
pub struct ReturnValue {
    pub best_move: Option<ChessMove>,
    pub position_evaluation: f64
}

pub fn get_flipped_board_index(square_index: usize) -> usize {

    let a = (square_index / 8) + 1;
    let b = 8 - a;
    let c = 8 * b;
    let d = square_index % 8;

    return c + d;
}

pub fn calculate_position(board: &Board) -> f64 {

    // returns the numerical value of the positiong

    let mut white_eval = 0.0;
    let mut black_eval = 0.0;

    let white_pieces = board.color_combined(Color::White);
    let black_pieces = board.color_combined(Color::Black);

    for x in *white_pieces {
        let square_index = get_flipped_board_index(x.to_int() as usize);
        let piece = board.piece_on(x);
        match piece {
            Some(Piece::Pawn) => { 
                white_eval += 10.0 + PAWN_VALUES[square_index]/* * 2.0 */;
            },
            Some(Piece::Knight) => {
                white_eval += 35.0 + KNIGHT_VALUES[square_index]/* * 2.0 */;
            },
            Some(Piece::Bishop) => {
                white_eval += 35.0 + BISHOP_VALUES[square_index]/* * 2.0 */;
            },
            Some(Piece::Rook)  => {
                white_eval += 52.5 + ROOK_VALUES[square_index]/* * 2.0 */;
            },
            Some(Piece::Queen) => {
                white_eval += 100.0 + QUEEN_VALUES[square_index]/* * 2.0 */;
            },
            Some(Piece::King) => {
                white_eval += 1000.0 + KING_VALUES[square_index]/* * 2.0 */;
            },
            _ => ()
        }
    }

    for x in *black_pieces {
        let square_index = x.to_int() as usize;

        let piece = board.piece_on(x);
        match piece {
            Some(Piece::Pawn) => {
                black_eval += 10.0 + PAWN_VALUES[square_index]/* * 2.0 */;
            },
            Some(Piece::Knight) => {
                black_eval += 35.0 + KNIGHT_VALUES[square_index]/* * 2.0 */;
            },
            Some(Piece::Bishop) => {
                black_eval += 35.0 + BISHOP_VALUES[square_index]/* * 2.0 */;
            },
            Some(Piece::Rook)  => {
                black_eval += 52.5 + ROOK_VALUES[square_index]/* * 2.0 */;
            },
            Some(Piece::Queen) => {
                black_eval += 100.0 + QUEEN_VALUES[square_index]/* * 2.0 */;
            },
            Some(Piece::King) => {
                black_eval += 1000.0 + KING_VALUES[square_index]/* * 2.0 */;
            },
            _ => ()
        }
    }
    return white_eval - black_eval;
}
    
pub fn calc_move(board: &Board, depth: u8, original_depth: u8, color: i8, mut alpha: f64, mut beta: f64, debug: bool) -> ReturnValue {
    
    unsafe {count += 1;}

    // An inline implementation of board.status() basically.
    // Because board.status() uses the MoveGen::new_legal call anyway,
    // There is no need to waste the time calling board.status()
    let all_moves = MoveGen::new_legal(&board);

    match all_moves.len() {
        0 => {
            if *board.checkers() == EMPTY {
                return ReturnValue {
                    best_move: None,
                    position_evaluation: 0.0          
                }
            } else {
                return ReturnValue {
                    best_move: None,
                    position_evaluation: f64::from(color) * INFINITY
                }
            }
        },
        _ => ()
    };
    
    if depth == 0 {
        return ReturnValue {
            best_move: None,
            position_evaluation: f64::from(color) * calculate_position(&board)
        };
    }
    

    let mut value = -INFINITY;
    let mut current_best_move: Option<ChessMove> = None;
    let mut best_alpha = alpha;
    let mut best_beta = beta;
    
    for legal_move in all_moves {

        let new_board = board.make_move_new(legal_move);

        let child_node = calc_move(&new_board, depth - 1, original_depth, -color, -beta, -alpha, false);

        if -child_node.position_evaluation > value {
            value = -child_node.position_evaluation;
            current_best_move = Some(legal_move);
        }

        if value > alpha {
            alpha = value;
            best_alpha = alpha;
        }

        // if alpha >= beta {
        //     unsafe {pruned += 1;}
        //     break;
        // }
    }

    if debug {
        let new_board = match current_best_move {
            Some(i) => {
                println!("{} -- Depth: {} -- Move: {}{} -- Value: {} -- Initial Value: {}", 
                    match color {1 => "White", -1 => "Black", _ => "Uh-oh"},
                    depth, i.get_source(), i.get_dest(), value, calculate_position(&board)); 
                board.make_move_new(i)
            }
            _ => {
                println!("error");
                Board::default()
            }
        };
        let f = calc_move(&new_board, depth - 1, original_depth, -color, -best_beta, -best_alpha, true);
    }

    return ReturnValue { 
        best_move: current_best_move, 
        position_evaluation: value
    };
}