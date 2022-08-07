mod positions;

use crate::zob_return::{ZobristReturn};
use chess::{Board, ChessMove, Color, MoveGen, Square, EMPTY};
use crate::moveiterator::MoveIterator;
use fasthash::{xx, RandomState};
use positions::{get_flipped_board_index, get_piece_value};
use std::collections::{HashMap, VecDeque};

pub const INFINITY: f64 = f64::INFINITY;
pub static mut searched: u128 = 0;

#[allow(dead_code)]
pub fn calculate_distance(sq: Square, other: Square) -> i32 {
    let rank_sq = (sq.get_rank().to_index() + 1) as i32;
    let rank_other = (other.get_rank().to_index() + 1) as i32;
    let dist = (rank_sq - rank_other).abs();
    return dist;
}
#[inline]
pub fn calculate_position(board: &Board, legal_moves: &MoveGen, color: f64) -> f64 {
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


    let eval = (white_eval - black_eval);

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
) -> Option<ChessMove> {
    unsafe {
        searched += 1;
    };

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
                );

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
    unsafe { searched += 1 };

    let mut all_moves = MoveIterator::new_legal(&board, masks);

    match all_moves.len() {
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

    if let Some(zob_val) = zobrist_table.get(&hash) {
        if zob_val.depth >= depth {
            if zob_val.depth % 2 != depth % 2 {
                return -zob_val.value;
            }
            return zob_val.value;
        }
        possible_best_move = Some(zob_val.best_move);
        // If the depth of the zobval is less than current depth
        // We should check the zobval best move first because it
        // will greatly increase chance of pruning
        //all_moves.set_iterator_mask(BitBoard::from_square(zob_val.best_move.get_source()) ^
        //BitBoard::from_square(zob_val.best_move.get_dest()));
        //best_move_first = true;
    }

    all_moves.set_first_mask(possible_best_move);

    //if let Some(m) = possible_best_move {
        //let new_board = board.make_move_new(m);

        //let depth_to_pass =
            //if depth == 1 && *board.checkers() != EMPTY && max_iterative_deepening_depth > 1 {
                //1
            //} else {
                //depth - 1
            //};

        //let child_node = -calc_move(
            //&new_board,
            //depth_to_pass,
            //max_iterative_deepening_depth - 1,
            //-color,
            //-beta,
            //-alpha,
            //zobrist_table,
            //scheduled_removal,
            //capacity,
        //);

        //value = f64::max(value, child_node);

        //let zob_ret = ZobristReturn {
            //value,
            //depth,
            //best_move: m,
        //};

        //zobrist_table.insert(hash, zob_ret);
        //scheduled_removal.push_back(hash);

        //if scheduled_removal.len() >= capacity {
            //if let Some(zobrist) = scheduled_removal.pop_front() {
                //zobrist_table.remove(&zobrist);
            //}
        //}
        //alpha = f64::max(alpha, value);

        //if alpha >= beta {
            //return value;
        //}
    //}

    all_moves.set_iterator_mask(*board.color_combined(!board.side_to_move()));

    let mut current_best_move: Option<ChessMove> = None;
    let mut last = ChessMove::new(Square::A1, Square::G8, None);
    let mut attack_moves_completed = false;
    let mut empty_moves_completed = false;

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

                last = i;

                let new_board = board.make_move_new(i);

                let depth_to_pass = if depth == 1
                    && *board.checkers() != EMPTY
                    && max_iterative_deepening_depth > 1
                {
                    1
                } else if !attack_moves_completed && depth == 1 && max_iterative_deepening_depth > 1
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

    let zob_ret = ZobristReturn {
        value,
        depth,
        best_move: current_best_move.unwrap(),
    };

    zobrist_table.insert(hash, zob_ret);
    scheduled_removal.push_back(hash);

    if scheduled_removal.len() >= capacity {
        if let Some(zobrist) = scheduled_removal.pop_front() {
            zobrist_table.remove(&zobrist);
        }
    }

    value
}

//pub fn sort_moves(board: &Board, moves: &MoveGen, first_move: Option<ChessMove>) -> MoveGen {

//let move_list = moves.get_moves();
//let promotion_index = moves.get_promotion_index();

//let mut final_move_list = NoDrop::new(ArrayVec::<[SquareAndBitBoard; 18]>::new());

//let first_move_vec_form =
//if let Some(m) = first_move {
//Some(SquareAndBitBoard::new(m.get_source(), BitBoard::from_square(m.get_dest()), if let Some(b) = m.get_promotion() {true} else {false}))
//} else {
//None
//};

//let mut index_best = 999;
//// Find the move that should be repositioned
//if let Some(first_m) = first_move_vec_form {
//final_move_list.push(first_m);
//for i in 0..move_list.len() {
//if first_m == move_list[i] {
//index_best = i;
//}
//}
//}

//let iterator_mask = *board.color_combined(!board.side_to_move());

//let c = convert_u64_to_bin(iterator_mask.0);

//assert_eq!(c, iterator_mask.0);

//// next, find each element past i where the moves are used, and store
//// that in i.  Then, increment i to point to a new unused slot.
//for j in 0..move_list.len() {
//if j == index_best {continue;}
//if *move_list[j].get_square() == Square::G6 {
//let p = move_list[j].get_bitboard().0;
//convert_u64_to_bin(p);
//convert_u64_to_bin(p & iterator_mask.0);
//}
////if move_list[j].get_bitboard() & iterator_mask != EMPTY {
////final_move_list.push(move_list[j]);
////let sq_d = move_list[j].get_bitboard().to_square();
////let sq_s = move_list[j].get_square();
////convert_u64_to_bin(move_list[j].get_bitboard().0);
////println!("{}{}", sq_s, sq_d);
////}
////let sq_d = move_list[j].get_bitboard().to_square();
////println!("{}", sq_d);
//}

//let iterator_mask = !EMPTY;

//for j in 0..move_list.len() {
//if j == index_best {continue;}
//if move_list[j].get_bitboard() & iterator_mask != EMPTY {
//final_move_list.push(move_list[j]);
//}

//}

//let r = MoveGen::new(final_move_list, promotion_index);

//return r;
//}
