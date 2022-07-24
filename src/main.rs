use chess::{Board, MoveGen, ChessMove, Square, Color, Piece, BoardStatus, EMPTY};

static PAWN_VALUES: &'static [f64] =    &[0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
                                        5.0,  5.0,  5.0,  5.0,  5.0,  5.0,  5.0,  5.0,
                                        2.0,  1.0,  2.0,  3.0,  3.0,  2.0,  1.0,  1.0,
                                        1.0,  0.5,  1.0,  2.5,  2.5,  1.0,  0.5,  0.5,
                                        0.75,  0.0,  0.0,  2.0,  2.0,  0.0,  0.0,  0.0,
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
                position_value += 10.0 + PAWN_VALUES[square_index] * 2.0;
            },
            Some(Piece::Knight) => {
                position_value += 35.0 + KNIGHT_VALUES[square_index] * 2.0;
            },
            Some(Piece::Bishop) => {
                position_value += 37.5 + BISHOP_VALUES[square_index] * 2.0;
            },
            Some(Piece::Rook)  => {
                position_value += 52.5 + ROOK_VALUES[square_index] * 2.0;
            },
            Some(Piece::Queen) => {
                position_value += 100.0 + QUEEN_VALUES[square_index] * 2.0;
            },
            Some(Piece::King) => {
                position_value += 1000.0 + KING_VALUES[square_index] * 2.0;
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
                position_value -= 10.0 + PAWN_VALUES[square_index] * 2.0;
            },
            Some(Piece::Knight) => {
                position_value -= 35.0 + KNIGHT_VALUES[square_index] * 2.0;
            },
            Some(Piece::Bishop) => {
                position_value -= 37.5 + BISHOP_VALUES[square_index] * 2.0;
            },
            Some(Piece::Rook)  => {
                position_value -= 52.5 + ROOK_VALUES[square_index] * 2.0;
            },
            Some(Piece::Queen) => {
                position_value -= 100.0 + QUEEN_VALUES[square_index] * 2.0;
            },
            Some(Piece::King) => {
                position_value -= 1000.0 + KING_VALUES[square_index] * 2.0;
            },
            _ => ()
        }
    }
    return position_value;
}

// In some cases we need to return the value of the node

fn calculate_move(board: &Board, depth: u8, original_depth: u8, player: bool, optimizing_color: Color, mut alpha: f64, mut beta: f64) -> ReturnValue {
    let status = board.status();
    if depth == 0 || status != BoardStatus::Ongoing {
        match status {
            BoardStatus::Ongoing => return ReturnValue  { 
                best_move: None,
                position_evaluation: calculate_position(board)
            },
            BoardStatus::Checkmate => {
                // Optimizing for white
                //  find mate when player is true (meaning white gets mated):
                //      - return lowest possible number, since bad for white
                //  find mate when player is false (meaning black gets mated by white):
                //      - return highest possible number, since good for wwhite
                
                // Optimizing for black
                //  find mate when player is true (meaning white gets mated):
                //      - return lowest possible number, since good for black
                //  find mate when player is false (meaning black gets mated by white):
                //      - return highest possible number, since bad for blacck
                // let mut eval = 0.0;
                // if optimizing_color == Color::White {
                //     eval = if player {-999999.0} else {999999.0} 
                // } else {
                //     eval = if player {d}
                // }
                return ReturnValue  { 
                    best_move: None,
                    position_evaluation: if player {-999999.0} else {999999.0}
                }
            },
            BoardStatus::Stalemate => return ReturnValue {
                best_move: None,
                position_evaluation: 0.0
            }
        }
    }
    let mut current_best_move = None;
    
    let targets = board.color_combined(!board.side_to_move());

    let mut attack_moves =  MoveGen::new_legal(&board);
    attack_moves.set_iterator_mask(*targets);

    let mut empty_moves = MoveGen::new_legal(&board);
    empty_moves.set_iterator_mask(!EMPTY);


    if player {
        let mut value = -999999.0;
        let mut best_value_move = -999999.0;
        
        for set in [attack_moves, empty_moves] {
            for legal_move in set {

                if current_best_move == None {
                    current_best_move = Some(legal_move);
                }

                // Make move
                let new_board = board.make_move_new(legal_move);

                // Calculate value of line
                let node_value = calculate_move(&new_board, depth - 1, original_depth, false, optimizing_color, alpha, beta);

                value = f64::max(value, node_value.position_evaluation);    
                
                // Prune or not to prune
                if beta <= alpha {
                    break;
                }

                if value > best_value_move {
                    best_value_move = value;
                    current_best_move = Some(legal_move);
                }

                alpha = f64::max(alpha, value);
            }
        }
        if depth == original_depth {
            println!("White: {}", best_value_move);
        }
        return ReturnValue {
            best_move: current_best_move,
            position_evaluation: value
        }
    } else {
        let mut value = 999999.0;
        let mut best_value_move = 999999.0;

        for set in [attack_moves, empty_moves] {
            for legal_move in set {

                if current_best_move == None {
                    current_best_move = Some(legal_move);
                }

                // Make move
                let new_board = board.make_move_new(legal_move);

                // Calculate value of line
                let node_value = calculate_move(&new_board, depth - 1, original_depth, true, optimizing_color, alpha, beta);
                
                value = f64::min(value, node_value.position_evaluation);
                

                // Prune or not to prune
                if beta <= alpha {
                    break;
                }

                if value < best_value_move {
                    best_value_move = value;
                    current_best_move = Some(legal_move);
                }

                beta = f64::min(beta, value);
            }
        }
        if depth == original_depth {
            println!("Black: {}", best_value_move);
        }
        return ReturnValue {
            best_move: current_best_move,
            position_evaluation: value
        }
    }
}

fn generate_pgn(board: &Board, chess_move: &Option<ChessMove>, color: Color, current_pgn: &String, move_number: i32) -> String{
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
            let full_string = format!("{beginning}{piece_str}{capture}{destination}{check} ");

            let pgn = format!("{current_pgn}{full_string}");

            return pgn;
        },
        None => {println!("error"); return String::from("")} 
    }
    

}

fn main() {

    let mut board = Board::default();

    let mut player = true;
    let mut optimizing_color = Color::White;

    let mut pgn = "".to_owned();

    for x in 1..200 {

        if board.status() == BoardStatus::Checkmate || board.status() == BoardStatus::Stalemate {
            break;
        }

        let calculated_move = calculate_move(&board, 4, 4, player, optimizing_color, -999999.0, 999999.0);

        pgn = generate_pgn(&board, &calculated_move.best_move, optimizing_color, &pgn, x);

        match calculated_move.best_move {
            Some(i) => {
                // Make move
                board = board.make_move_new(i);
            },
            None => {println!("chess move invalid"); break;} 
        }
        player = !player;
        if optimizing_color == Color::White {optimizing_color = Color::Black;} else {optimizing_color = Color::White;}
    }

    println!("{}", format!("{board}"));

    println!("{}", pgn);


}