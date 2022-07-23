use chess::{Board, MoveGen, ChessMove, Square, Color, Piece, BoardStatus, BitBoard};
use gtk::gdk::keys::constants::R;
use std::cmp;

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
                      
                                        
static mut count: i32 = 0;

struct ReturnValue {
    best_move: Option<ChessMove>,
    position_evaluation: f64
}

fn calculate_position(board: &Board) -> f64 {

    let mut position_value = 0.0;

    let white_pieces = board.color_combined(Color::White);
    let black_pieces = board.color_combined(Color::Black);

    for x in *white_pieces {
        let square_index = x.to_int() as usize;
        let piece = board.piece_on(x);
        match piece {
            Some(Piece::Pawn) => {
                position_value += 10.0 + PAWN_VALUES[square_index];
            },
            Some(Piece::Knight) => {
                position_value += 35.0 + KNIGHT_VALUES[square_index];
            },
            Some(Piece::Bishop) => {
                position_value += 37.5 + BISHOP_VALUES[square_index];
            },
            Some(Piece::Rook)  => {
                position_value += 52.5 + ROOK_VALUES[square_index];
            },
            Some(Piece::Queen) => {
                position_value += 100.0 + QUEEN_VALUES[square_index];
            },
            Some(Piece::King) => {
                position_value += 1000.0 + KING_VALUES[square_index];
            },
            _ => ()
        }
    }
    for x in *black_pieces {
        let square_index = x.to_int() as usize;
        
        let a = (square_index / 8) + 1;
        let b = 8 - a;
        let c = 8 * b;
        let d = square_index % 8;

        let square_index = c + d;

        let piece = board.piece_on(x);
        match piece {
            Some(Piece::Pawn) => {
                position_value -= 10.0 + PAWN_VALUES[square_index];
            },
            Some(Piece::Knight) => {
                position_value -= 35.0 + KNIGHT_VALUES[square_index];
            },
            Some(Piece::Bishop) => {
                position_value -= 37.5 + BISHOP_VALUES[square_index];
            },
            Some(Piece::Rook)  => {
                position_value -= 52.5 + ROOK_VALUES[square_index];
            },
            Some(Piece::Queen) => {
                position_value -= 100.0 + QUEEN_VALUES[square_index];
            },
            Some(Piece::King) => {
                position_value -= 1000.0 + KING_VALUES[square_index];
            },
            _ => ()
        }
    }
    return position_value;
}

// In some cases we need to return the value of the node

fn calculate_move(board: &Board, depth: u8, original_depth: u8, player: bool, optimizing_color: Color, mut alpha: f64, mut beta: f64) -> ReturnValue {
    let status = board.status();
    unsafe {count += 1;}
    if depth == 0 || status != BoardStatus::Ongoing {
        if optimizing_color == Color::White {
            match status {
                BoardStatus::Ongoing => return ReturnValue  { 
                    best_move: None,
                    position_evaluation: calculate_position(board)
                },
                BoardStatus::Checkmate => return ReturnValue  { 
                    best_move: None,
                    position_evaluation: if player {999999.0} else {-999999.0}
                },
                BoardStatus::Stalemate => return ReturnValue {
                    best_move: None,
                    position_evaluation: 0.0
                }
            }
        } else {    
            match status {
                BoardStatus::Ongoing => return ReturnValue  { 
                    best_move: None,
                    position_evaluation: calculate_position(board)
                },
                BoardStatus::Checkmate => return ReturnValue  { 
                    best_move: None,
                    position_evaluation: if player {-999999.0} else {999999.0}
                },
                BoardStatus::Stalemate => return ReturnValue {
                    best_move: None,
                    position_evaluation: 0.0
                }
            }
        }
    }
    let mut current_best_move = Some(ChessMove::new(Square::A1, Square::H8, None));
    let movegen =  MoveGen::new_legal(&board);
    if player {
        let mut value = -999999.0;
        let mut best_value_move = -999999.0;
        
        for legal_move in movegen {

            // Make move
            let new_board = board.make_move_new(legal_move);

            // Calculate value of line
            let node_value = calculate_move(&new_board, depth - 1, original_depth, false, optimizing_color, alpha, beta);
            
            value = f64::max(value, node_value.position_evaluation);    

            alpha = f64::max(alpha, value);

            // Prune or not to prune
            if value >= beta {
                break;
            }

            if value > best_value_move {
                best_value_move = value;
                current_best_move = Some(legal_move);
            }

        }
        if depth == original_depth {
            println!("Black: {}", best_value_move);
        }
        return ReturnValue {
            best_move: current_best_move,
            position_evaluation: value
        }
    } else {
        let mut value = 999999.0;
        let mut best_value_move = 999999.0;

        for legal_move in movegen {
            
            // Make move
            let new_board = board.make_move_new(legal_move);

            // Calculate value of line
            let node_value = calculate_move(&new_board, depth - 1, original_depth, true, optimizing_color, alpha, beta);
            
            value = f64::min(value, node_value.position_evaluation);
            
            beta = f64::min(beta, value);

            // Prune or not to prune
            if value <= alpha {
                break;
            }

            if value < best_value_move {
                best_value_move = value;
                current_best_move = Some(legal_move);
            }


        }
        if depth == original_depth {
            println!("White: {}", best_value_move);
        }
        return ReturnValue {
            best_move: current_best_move,
            position_evaluation: value
        }
    }
}

fn main() {

    let mut board = Board::default();

    println!("peft 4: {}", MoveGen::movegen_perft_test(&board, 4));

    let mut player = true;
    let mut optimizing_color = Color::White;

    for i in 1..10 {
        let calculated_move = calculate_move(&board, 4, 4, player, optimizing_color, -999999.0, 999999.0);

        match calculated_move.best_move {
            Some(i) => {
                board = board.make_move_new(i);
                println!("{}{}", i.get_source(), i.get_dest());
            },
            None => {println!("error"); break;} 
        }
        player = !player;
        if optimizing_color == Color::White {optimizing_color = Color::Black;} else {optimizing_color = Color::White;}
        unsafe {println!("{}", count); count = 0;}
    }



}