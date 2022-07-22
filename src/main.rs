use chess::{Board, MoveGen, ChessMove, Square, Color, Piece, BoardStatus};
use std::cmp;

struct ReturnValue {
    best_move: ChessMove,
    position_evaluation: i32
}

fn calculate_position(board: &Board) -> i32 {

    let mut position_value = 0;

    let white_pieces = board.color_combined(Color::White);
    let black_pieces = board.color_combined(Color::Black);

    let pawns = board.pieces(Piece::Pawn);
    let rooks = board.pieces(Piece::Rook);
    let knights = board.pieces(Piece::Knight);
    let bishops = board.pieces(Piece::Bishop);
    let kings = board.pieces(Piece::King);
    let queens = board.pieces(Piece::King);

    // Calculate for white pieces
    for _ in white_pieces & pawns {
        position_value += 1;
    }
    for _ in white_pieces & rooks {
        position_value += 5;
    }
    for _ in white_pieces & knights {
        position_value += 3;
    }
    for _ in white_pieces & bishops {
        position_value += 3;
    }
    for _ in white_pieces & kings {
        position_value += 10000;
    }
    for _ in white_pieces & queens {
        position_value += 10;
    }

    for _ in black_pieces & pawns {
        position_value -= 1;
    }
    for _ in black_pieces & rooks {
        position_value -= 5;
    }
    for _ in black_pieces & knights {
        position_value -= 3;
    }
    for _ in black_pieces & bishops {
        position_value -= 3;
    }
    for _ in black_pieces & kings {
        position_value -= 10000;
    }
    for _ in black_pieces & queens {
        position_value -= 10;
    }

    return position_value;

}

// In some cases we need to return the value of the node

fn calculate_move(mut board: &Board, depth: u8, original_depth: u8, player: bool, mut alpha: i32, mut beta: i32) -> ReturnValue {
    let status = board.status();
    if depth == 0 || board.status() != BoardStatus::Ongoing {
        if player {
            match status {
                BoardStatus::Ongoing => return ReturnValue  { 
                    best_move: ChessMove::new(Square::A1, Square::A2, Option::None),
                    position_evaluation: calculate_position(board)
                },
                BoardStatus::Checkmate => return ReturnValue  { 
                    best_move: ChessMove::new(Square::A1, Square::A2, Option::None),
                    position_evaluation: -999999
                },
                BoardStatus::Stalemate => { return ReturnValue {
                    best_move: ChessMove::new(Square::A1, Square::A2, Option::None),
                    position_evaluation: 0
                } 
                }
            }
        }
        else {
            match status {
                BoardStatus::Ongoing => return ReturnValue  { 
                    best_move: ChessMove::new(Square::A1, Square::A2, Option::None),
                    position_evaluation: calculate_position(board)
                },
                BoardStatus::Checkmate => return ReturnValue  { 
                    best_move: ChessMove::new(Square::A1, Square::A2, Option::None),
                    position_evaluation: 999999
                },
                BoardStatus::Stalemate => return ReturnValue {
                    best_move: ChessMove::new(Square::A1, Square::A2, Option::None),
                    position_evaluation: 0
                } // Stalemate
            }
        }
    }
    let mut current_best_move = ChessMove::new(Square::A1, Square::H1, Option::None);
    let movegen =  MoveGen::new_legal(&board);
    if player {
        let mut value = -999999;
        let mut best_value_move = -999999;
        
        for legal_move in movegen {

            let original_board = board.clone();
            
            // Make move
            board.make_move(legal_move, &mut board);

            // Calculate value of line
            let node_value = calculate_move(&mut board, depth - 1, original_depth, true, alpha, beta);
            
            value = cmp::max(value, node_value.position_evaluation);

            
            // Prune or not to prune
            if value >= beta {
                break;
            }

            alpha = cmp::max(alpha, value);

            if value >= best_value_move {
                best_value_move = value;
                current_best_move = legal_move;
            }

            // Unmake move
            board = &original_board;
        }
        return ReturnValue {
            best_move: current_best_move,
            position_evaluation: value
        }
    } else {
        let mut value = 999999;
        let mut best_value_move = 999999;

        for legal_move in movegen {
            
            let original_board = board.clone();

            // Make move
            board.make_move(legal_move, &mut board);

            // Calculate value of line
            let node_value = calculate_move(&mut board, depth - 1, original_depth, true, alpha, beta);
            
            value = cmp::min(value, node_value.position_evaluation);
            
            // Prune or not to prune
            if value <= alpha {
                break;
            }
            beta = cmp::min(beta, value);

            if value <= best_value_move {
                best_value_move = value;
                current_best_move = legal_move;
            }

            // Unmake move
            board = &original_board;
        }
        return ReturnValue {
            best_move: current_best_move,
            position_evaluation: value
        }
    }
}

fn main() {
    

    let mut board = Board::default();

    println!("{}", calculate_position(&board));

    let mut player = true;
    // for i in 1..10 {
    //     let calculated_move = calculate_move(&board, 4, 4, player, -999999, 999999);

    //     board = board.make_move_new(calculated_move.best_move);

    //     println!("{}{}", calculated_move.best_move.get_source(), calculated_move.best_move.get_dest());

    //     player = !player;
    // }

}