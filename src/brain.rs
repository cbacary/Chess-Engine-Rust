mod positions;

use chess::{BitBoard, Board, ChessMove, Color, File, MoveGen, Piece, Rank, Square, EMPTY};
use positions::*;

pub const INFINITY: f64 = f64::INFINITY;

/// Generates a pgn
pub fn generate_pgn(
    board: &Board,
    chess_move: &Option<ChessMove>,
    color: Color,
    current_pgn: &String,
    move_number: i32,
) -> String {
    match *chess_move {
        Some(i) => {
            // Check white or black, create beginning
            let beginning = match color {
                Color::White => {
                    let move_num = (move_number / 2) + 1 as i32;
                    format!("{move_num}. ")
                }
                Color::Black => {
                    format!(" ")
                }
            };

            // Check if castle
            if i.get_source() == Square::E1
                && i.get_dest() == Square::G1
                && board.piece_on(Square::E1) == Some(Piece::King)
            {
                let full_string = format!("{beginning}O-O ");
                let pgn = format!("{current_pgn}{full_string}");
                return pgn;
            } else if i.get_source() == Square::E8
                && i.get_dest() == Square::G8
                && board.piece_on(Square::E8) == Some(Piece::King)
            {
                let full_string = format!("{beginning}O-O ");
                let pgn = format!("{current_pgn}{full_string}");
                return pgn;
            } else if i.get_source() == Square::E1
                && i.get_dest() == Square::C1
                && board.piece_on(Square::E1) == Some(Piece::King)
            {
                let full_string = format!("{beginning}O-O-O ");
                let pgn = format!("{current_pgn}{full_string}");
                return pgn;
            } else if i.get_source() == Square::E8
                && i.get_dest() == Square::C8
                && board.piece_on(Square::E8) == Some(Piece::King)
            {
                let full_string = format!("{beginning}O-O-O ");
                let pgn = format!("{current_pgn}{full_string}");
                return pgn;
            }

            // Check if pawn promotion
            let mut promotion = "".to_owned();

            let promote = i.get_promotion();
            let promotion = match promote {
                None => "".to_owned(),
                Some(i) => format!("={i}"),
            };

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
                _ => "",
            };

            // Get rank
            let rank = match i.get_source().get_rank() {
                Rank::First => "1",
                Rank::Second => "2",
                Rank::Third => "3",
                Rank::Fourth => "4",
                Rank::Fifth => "5",
                Rank::Sixth => "6",
                Rank::Seventh => "7",
                Rank::Eighth => "8",
            };

            let file = file.to_owned();
            let rank = rank.to_owned();

            // Get piece being moved
            let piece_str = match board.piece_on(i.get_source()) {
                Some(Piece::Pawn) => "".to_owned(),
                Some(i) => format!("{i}").to_uppercase(),
                _ => "".to_owned(),
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
            let full_string = 
format!(
"{beginning}{piece_str}{file}{rank}{capture}{destination}{promotion}{check}"
            );

            let pgn = format!("{current_pgn}{full_string}");

            return pgn;
        }
        None => {
            println!("error");
            return String::from("");
        }
    }
}

#[derive(Debug)]
pub struct ReturnValue {
    pub best_move: Option<ChessMove>,
    pub position_evaluation: f64,
}

pub fn get_flipped_board_index(square_index: usize) -> usize {
    // Given an index on a 64 array board, converts it to a flipped version
    // This is most likely slower than just creating new static positions for
    // the opposite color.
    // Another option for this could be to create a static u8 array
    // that stores each index's equivalent so it doesn't have to be solved in run-time

    let a = (square_index / 8) + 1;
    let b = 8 - a;
    let c = 8 * b;
    let d = square_index % 8;

    return c + d;
}

#[inline]
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
            }
            Some(Piece::Knight) => {
                white_eval += 35.0 + KNIGHT_VALUES[square_index]/* * 2.0 */;
            }
            Some(Piece::Bishop) => {
                white_eval += 35.0 + BISHOP_VALUES[square_index]/* * 2.0 */;
            }
            Some(Piece::Rook) => {
                white_eval += 52.5 + ROOK_VALUES[square_index]/* * 2.0 */;
            }
            Some(Piece::Queen) => {
                white_eval += 100.0 + QUEEN_VALUES[square_index]/* * 2.0 */;
            }
            Some(Piece::King) => {
                white_eval += 1000.0 + KING_VALUES[square_index]/* * 2.0 */;
            }
            _ => (),
        }
    }

    for x in *black_pieces {
        let square_index = x.to_int() as usize;

        let piece = board.piece_on(x);
        match piece {
            Some(Piece::Pawn) => {
                black_eval += 10.0 + PAWN_VALUES[square_index]/* * 2.0 */;
            }
            Some(Piece::Knight) => {
                black_eval += 35.0 + KNIGHT_VALUES[square_index]/* * 2.0 */;
            }
            Some(Piece::Bishop) => {
                black_eval += 35.0 + BISHOP_VALUES[square_index]/* * 2.0 */;
            }
            Some(Piece::Rook) => {
                black_eval += 52.5 + ROOK_VALUES[square_index]/* * 2.0 */;
            }
            Some(Piece::Queen) => {
                black_eval += 100.0 + QUEEN_VALUES[square_index]/* * 2.0 */;
            }
            Some(Piece::King) => {
                black_eval += 1000.0 + KING_VALUES[square_index]/* * 2.0 */;
            }
            _ => (),
        }
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
    debug: bool,
) -> Option<ChessMove> {
    let mut all_moves = MoveGen::new_legal(&board);

    if all_moves.len() == 0 {
        if *board.checkers() == EMPTY {
            return None;
        } else {
            return None;
        }
    }

    let mut value = -INFINITY;
    let mut current_best_move: Option<ChessMove> = None;

    // best_alpha and best_beta are solely here for the debug option
    // let mut best_alpha = alpha;
    // let mut best_beta = beta;

    let mut attack_moves_completed = false;
    let mut empty_moves_completed = false;

    all_moves.set_iterator_mask(*board.color_combined(!board.side_to_move()));

    loop {
        match all_moves.next() {
            None => {
                if attack_moves_completed == false {
                    attack_moves_completed = true;
                    all_moves.set_iterator_mask(!EMPTY);
                } else if empty_moves_completed == false {
                    empty_moves_completed = true;
                    break;
                }
            }
            Some(i) => {
                let new_board = board.make_move_new(i);

                let child_node = calc_move(
                    &new_board,
                    depth - 1,
                    max_iterative_deepening_depth - 1,
                    -color,
                    -beta,
                    -alpha,
                    false,
                );

                if -child_node > value {
                    value = -child_node;
                    current_best_move = Some(i);
                }

                if value > alpha {
                    alpha = value;
                    // best_alpha = alpha;
                }

                if alpha >= beta {
                    break;
                }
            }
        }
    }
    return current_best_move;
}

#[inline]
pub fn calc_move(
    board: &Board,
    depth: u8,
    max_iterative_deepening_depth: u8,
    color: i8,
    mut alpha: f64,
    beta: f64,
    debug: bool,
) -> f64 {
    // An inline implementation of board.status() basically.
    // Because board.status() uses the MoveGen::new_legal call anyway,
    // There is no need to waste the time calling board.status()
    let mut all_moves = MoveGen::new_legal(&board);

    if all_moves.len() == 0 {
        if *board.checkers() == EMPTY {
            return 0.0;
        } else {
            return -INFINITY;
        }
    }

    if depth == 0 {
        return f64::from(color) * calculate_position(&board);
    }

    let mut value = -INFINITY;
    // let mut current_best_move: Option<ChessMove> = None;

    // best_alpha and best_beta are solely here for the debug option
    // let mut best_alpha = alpha;
    // let mut best_beta = beta;

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
                    empty_moves_completed = true;
                    break;
                }
            }
            Some(i) => {
                // Implementation of negamax search w/ alpha-beta pruning

                let new_board = board.make_move_new(i);

                // This helps the AI to solve if a trade is actually worth it.
                // Rather than stopping a search half-way through the trade, we either
                // wait till the position reaches a point where there are no attack moves
                // or we reach the max depth.
                let depth_to_pass =
                    if !attack_moves_completed && depth == 1 && max_iterative_deepening_depth > 1 {
                        1
                    } else {
                        depth - 1
                    };

                let child_node = calc_move(
                    &new_board,
                    depth_to_pass,
                    max_iterative_deepening_depth - 1,
                    -color,
                    -beta,
                    -alpha,
                    false,
                );

                value = f64::max(value, -child_node);
                alpha = f64::max(alpha, value);

                // if -child_node > value {
                //     value = -child_node;
                // current_best_move = Some(i);
                // }

                // if value > alpha {
                //     alpha = value;
                //     best_alpha = alpha;
                // }

                if alpha >= beta {
                    break;
                }
            }
        }
    }

    // This allows you to get a peek into what the AI thinks was the best line
    // if debug {
    //     let new_board = match current_best_move {
    //         Some(i) => {
    //             println!("{} -- Depth: {} -- Move: {}{} -- Value: {} -- Initial Value: {}",
    //                 match color {1 => "White", -1 => "Black", _ => "Uh-oh"},
    //                 depth, i.get_source(), i.get_dest(), value, calculate_position(&board));
    //             board.make_move_new(i)
    //         }
    //         _ => {
    //             println!("error");
    //             Board::default()
    //         }
    //     };
    //     let f = calc_move(&new_board, depth - 1, max_iterative_deepening_depth - 1, -color, -best_beta, -best_alpha, true);
    // }
    value
}
