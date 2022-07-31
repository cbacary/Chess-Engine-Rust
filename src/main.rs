mod brain;

use std::str::FromStr;
use brain::{calculate_position, generate_pgn, calc_move, count, pruned, INFINITY};
use chess::{Board, MoveGen, Color, BoardStatus, ChessMove, Square, Game};

fn run_tests() 
{
    let board = Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").expect("E");
    
    let test_pos_board = Board::from_str("4r2k/1rp2ppp/p7/8/1P6/P7/2R3PP/2KR4 b - - 0 40").expect("E");

    let initial_eval = calculate_position(&test_pos_board);
    
    println!("Initial evaluation value: {}", initial_eval);

    let depth = 4;
    let optimizing_color = board.side_to_move();
    let alpha = -999999.0;
    let beta = -alpha;

    let color = if optimizing_color == Color::White {1} else {-1};

    let other_best_move = calc_move(&board, depth, color, alpha, beta, true);

    unsafe {println!("Moves searched: {} -- Nodes pruned: {}", count, pruned); count = 0; pruned = 0;}
    
    // let breakdown = breakdown_line(&board, depth, depth, player, optimizing_color, alpha, beta); 
    
    // println!("{:#?}", breakdown);
}

fn main() {

    // run_tests();

    let mut board = Board::default();

    let color = board.side_to_move();

    let mut color = if color == Color::White {1} else {-1};
    let depth = 4;
    let alpha = -INFINITY;
    let beta = INFINITY;

    let mut pgn = "".to_owned();
    let mut safe_pgn = "".to_owned();

    for i in 0..200 {

        let calculated_move = calc_move(&board, depth, color, alpha, beta, false);

        safe_pgn = format!("{pgn}");
        pgn = generate_pgn(&board, &calculated_move.best_move, board.side_to_move(), &pgn, i);

        match calculated_move.best_move {
            Some(i) => {
                // Make move
                board = board.make_move_new(i);
                println!("{}{}", i.get_source(), i.get_dest());
            },
            None => {println!("chess move invalid"); break;} 
        }
        color = -color;
    }
    println!("{}", format!("{board}"));
    println!("{}", safe_pgn);
}