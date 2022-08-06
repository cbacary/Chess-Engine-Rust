mod positions;

use std::collections::{VecDeque, HashMap};
use crate::zob_return::{ZobristReturn, Flag};
use chess::{Board, ChessMove, Color, File, MoveGen, Piece, Rank, Square, EMPTY,};
use positions::{get_flipped_board_index, get_piece_value};
use fasthash::{RandomState, xx};

pub const INFINITY: f64 = f64::INFINITY;
pub static mut searched: u128 = 0;



#[inline]
pub fn calculate_position(board: &Board) -> f64 {
    // returns the numerical value of the position

    let mut white_eval = 0.0;
    let mut black_eval = 0.0;

    let white_pieces = board.color_combined(Color::White);
    let black_pieces = board.color_combined(Color::Black);

    for x in *white_pieces {
        let square_index = get_flipped_board_index(x.to_int() as usize);
        white_eval += get_piece_value(board.piece_on(x).unwrap(), square_index);
    }

    for x in *black_pieces {
        let square_index = x.to_int() as usize;
        black_eval += get_piece_value(board.piece_on(x).unwrap(), square_index);
    }

    return white_eval - black_eval;
}

#[inline]
pub fn find_move(
    board: &Board,
    depth: u8,
    max_iterative_deepening_depth: u8,
    color: i8,
    mut alpha: f64,
    beta: f64,
    zobrist_table: &mut HashMap<u64, ZobristReturn, RandomState<xx::Hash64>>,
    scheduled_removal: &mut VecDeque<u64>,
    capacity: usize,
) -> Option<ChessMove> {
 
    unsafe {searched += 1;};

    let mut all_moves = MoveGen::new_legal(&board);
    let mut value = -INFINITY;
    let mut current_best_move: Option<ChessMove> = None;

    let hash = board.get_hash();

    let mut attack_moves_completed = false;

    all_moves.set_iterator_mask(*board.color_combined(!board.side_to_move()));

    loop {
        match all_moves.next() {
            None => {
                if attack_moves_completed == false {
                    attack_moves_completed = true;
                    all_moves.set_iterator_mask(!EMPTY);
                } else {
                    break;
                }
            }
            Some(i) => {


                let new_board = board.make_move_new(i);


                let child_node = -calc_move(
                    &new_board,
                    depth - 1,
                    max_iterative_deepening_depth - 1,
                    -color, 
                    -beta, 
                    -alpha,
                    zobrist_table,
                    scheduled_removal,
                    capacity
                );

                let zob_ret = ZobristReturn {
                    value: value,
                    depth: depth,
                    flag: Flag::Exact
                };
            
                zobrist_table.insert(hash, zob_ret);
                scheduled_removal.push_back(hash);
            
            

                if child_node > value {
                    value = child_node;
                    current_best_move = Some(i);
                }

                alpha = f64::max(alpha, value);

                if alpha >= beta {
                    break;
                }
            }
        }
    }

    return current_best_move;
}

pub fn calc_move(
    board: &Board,
    depth: u8,
    max_iterative_deepening_depth: u8,
    color: i8,
    mut alpha: f64,
    mut beta: f64,
    zobrist_table: &mut HashMap<u64, ZobristReturn, RandomState<xx::Hash64>>,
    scheduled_removal: &mut VecDeque<u64>,
    capacity: usize,
) -> f64 {

    let alpha_original = alpha;


    // An inline implementation of board.status() basically.
    // Because board.status() uses the MoveGen::new_legal call anyway,
    // There is no need to waste the time calling board.status()
    let mut all_moves = MoveGen::new_legal(&board);
    unsafe {searched += 1};
    match all_moves.len() {
        0 => {
            if *board.checkers() == EMPTY {
                return 0.0;
            } else {
                return -INFINITY;
            }
        }
        _ => ()
    }
    if depth == 0 {
        return f64::from(color) * calculate_position(&board);
    } 
    
    let hash = board.get_hash();

    if let Some(zob_val) = zobrist_table.get(&hash) {
        if zob_val.depth >= depth {
            //if zob_val.flag == Flag::Exact {
            if zob_val.depth  % 2 != depth % 2 {
                return -zob_val.value;
            }
            return zob_val.value;
            //} else if zob_val.flag == Flag::Upperbound {
                //alpha = f64::max(zob_val.value, alpha);
            //} else {
                //beta = f64::min(beta, zob_val.value);
            //}
            //if alpha >= beta {
                //return zob_val.value;
            //}
        }
    }

    let mut value = -INFINITY;
    // let mut current_best_move: Option<ChessMove> = None;

    // best_alpha and best_beta are solely here for the debug option

    let mut attack_moves_completed = false;
    let mut empty_moves_completed = false;

    all_moves.set_iterator_mask(*board.color_combined(!board.side_to_move()));

    loop {
        match all_moves.next() {
            None => {
                // if we completed one iterator mask do the next
                // this is to improve alpha-beta pruning speed
                // doing attack moves first increases chance of pruning
                if attack_moves_completed == false {
                    attack_moves_completed = true;
                    all_moves.set_iterator_mask(!EMPTY);
                } else if empty_moves_completed == false {
                    break;
                }
            }
            Some(i) => {
                // Implementation of negamax search w/ alpha-beta pruning

                let new_board = board.make_move_new(i);

                let depth_to_pass = if !attack_moves_completed && depth == 1 && 
                    max_iterative_deepening_depth > 1 {
                        1
                    } else {
                        depth - 1
                    };

                let child_node = -calc_move(
                        &new_board, depth_to_pass, 
                        max_iterative_deepening_depth - 1,
                        -color, -beta, -alpha,
                        zobrist_table, scheduled_removal, capacity
                );

                value = f64::max(value, child_node);

                alpha = f64::max(alpha, value);
                
                if alpha >= beta {
                    break;
                }
            }
        }
    }

    let flag = Flag::Exact; 
        //value <= alpha_original  {
            //Flag::Upperbound
        //} else if value >= beta {
            //Flag::Lowerbound
        //} else {
            //Flag::Exact
        //}; 
 
    let zob_ret = ZobristReturn {
        value: value,
        depth: depth,
        flag: flag,
    };
 
    zobrist_table.insert(hash, zob_ret);
    scheduled_removal.push_back(hash); 
  
    while scheduled_removal.len() > capacity {
        if let Some(zobrist) = scheduled_removal.pop_front() {
            zobrist_table.remove(&zobrist);
        }
    }




    value
}
