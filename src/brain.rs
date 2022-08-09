mod positions;

use crate::moveiterator::MoveIterator;
use crate::zob_return::{ZobristReturn, Flag};
use chess::{Board, ChessMove, Color, Square, EMPTY};
use fasthash::{xx, RandomState};
use positions::{get_flipped_board_index, get_piece_value};
use std::collections::{HashMap, VecDeque};

pub const INFINITY: f64 = f64::INFINITY;

#[allow(dead_code)]
pub fn calculate_distance(sq: Square, other: Square) -> i32 {
    let rank_sq = (sq.get_rank().to_index() + 1) as i32;
    let rank_other = (other.get_rank().to_index() + 1) as i32;
    let dist = (rank_sq - rank_other).abs();
    return dist;
}
#[inline]
pub fn calculate_position(board: &Board, _legal_moves: &MoveIterator, _color: f64) -> f64 {
    // Check if mate or check

    let mut white_eval = 0.0;
    let mut black_eval = 0.0;

    let white_pieces = board.color_combined(Color::White);
    let black_pieces = board.color_combined(Color::Black);

    //let white_pinned_pieces = board.pinned() & white_pieces;
    //let black_pinned_pieces = board.pinned() & black_pieces;

    for x in *white_pieces {
        //let is_pinned = (1u64 << x.to_int() & white_pinned_pieces.0) != 0;
        let square_index = get_flipped_board_index(x.to_int() as usize);
        white_eval += get_piece_value(board.piece_on(x).unwrap(), square_index, false);
    }

    for x in *black_pieces {
        let square_index = x.to_int() as usize;
        //let is_pinned = (1u64 << square_index & black_pinned_pieces.0) != 0;
        black_eval += get_piece_value(board.piece_on(x).unwrap(), square_index, false);
    }

    let eval = white_eval - black_eval;

    return eval;
}

//pub fn get_place_in_queue(total_moves_searched: u64, current_nodes_moves_searched: u64, capacity: usize) -> usize {
//return
//}

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
    debug: bool,
) -> Option<ChessMove> {
    let target_mask = *board.color_combined(!board.side_to_move());

    let masks = vec![target_mask, !EMPTY];

    let mut all_moves = MoveIterator::new_legal(&board, masks);

    all_moves.set_first_mask(None);

    let mut value = -INFINITY;

    //let mut current_best_move: Option<ChessMove> = None;
    let mut current_best_move: Option<ChessMove> = None;
    let mut last = ChessMove::new(Square::A1, Square::G8, None);

    loop {
        match all_moves.next() {
            None => {
                break;
            }
            Some(i) => {
                last = i;

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
                    capacity,
                    false,
                );

                if child_node > value {
                    value = child_node;
                    current_best_move = Some(i);
                }
                
                alpha = f64::max(alpha, child_node);

                if alpha >= beta {
                    break;
                }
            }
        }
    }

    if current_best_move == None {
        current_best_move = Some(last);
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
    _debug: bool,
) -> f64 {
    let masks = vec![*board.color_combined(!board.side_to_move()), !EMPTY];

    let attack_moves_mask_index = 0;
    let empty_moves_mask_index = 1;
    
    let original_alpha = alpha;

    let mut all_moves = MoveIterator::new_legal(&board, masks);

    match all_moves.len_all_moves() {
        0 => {
            if *board.checkers() == EMPTY {
                return 0.0;
            } else {
                return -INFINITY;
            }
        }
        _ => (),
    }

    if depth == 0 {
        return f64::from(color) * calculate_position(&board, &all_moves, color as f64);
    }

    let hash = board.get_hash();
    let mut value = -INFINITY;
    let mut possible_best_move: Option<ChessMove> = None;
    let mut mask_offset = 0;

    if let Some(zob_val) = zobrist_table.get(&hash) {
        let val = if zob_val.color_found != board.side_to_move() {-zob_val.value} else {zob_val.value};
        if zob_val.depth >= depth {
            if zob_val.flag == Flag::Exact {
                return val;
            } else if zob_val.flag == Flag::Lowerbound {
                alpha = f64::max(alpha, val);
            } else  {
                beta = f64::min(beta, val);
            }

            if alpha >= beta {
                return val;
            }
        }
        // If the depth of the zobval is less than current depth
        // We should check the zobval best move first because it
        // will greatly increase chance of pruning}
        possible_best_move = Some(zob_val.best_move);
        mask_offset += 1;
    }

    all_moves.set_first_mask(possible_best_move);

    let mut current_best_move: Option<ChessMove> = None;
    let mut last = ChessMove::new(Square::A1, Square::G8, None);


    loop {
        match all_moves.next() {
            None => {
                break;
            }
            Some(i) => {
                // Implementation of negamax search w/ alpha-beta pruning

                last = i;

                let new_board = board.make_move_new(i);

                let depth_to_pass = 
                    if depth == 1
                    && *board.checkers() != EMPTY
                    && max_iterative_deepening_depth > 1
                {
                    1
                } else if all_moves.get_mask_index() == attack_moves_mask_index + mask_offset
                    && depth == 1
                    && max_iterative_deepening_depth > 1
                {
                    1
                } else {
                    depth - 1
                };

                let child_node = -calc_move(
                    &new_board,
                    depth_to_pass,
                    max_iterative_deepening_depth - 1,
                    -color,
                    -beta,
                    -alpha,
                    zobrist_table,
                    scheduled_removal,
                    capacity,
                    false,
                );

                //value = f64::max(value, child_node);

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

    if current_best_move == None {
        current_best_move = Some(last);
    }

    let flag = 
        if value <= original_alpha { // This means
            Flag::Upperbound
        } else if value >= beta { // This means that this node had a prune take place
            Flag::Lowerbound
        } else {
            Flag::Exact
        };

    let zob_ret = ZobristReturn {
        value,
        depth,
        best_move: current_best_move.unwrap(),
        color_found: board.side_to_move(),
        flag
    };

    zobrist_table.insert(hash, zob_ret);
    scheduled_removal.push_back(hash);

    if scheduled_removal.len() >= capacity - 5 {
        if let Some(zobrist) = scheduled_removal.pop_front() {
            zobrist_table.remove(&zobrist);
        }
    }

    value
}
