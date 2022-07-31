mod brain;

use std::str::FromStr;
use brain::{calculate_position, generate_pgn, calc_move, count, pruned, INFINITY};
use chess::{Board, MoveGen, Color, BoardStatus, ChessMove, Square, Game};
use std::io;

fn run_tests() 
{
    let board = Board::from_str("r1k5/p4r1p/2np4/4p3/8/1PB1nP2/P1P1N2P/R1R3K1 b - - 0 24").expect("E");
    
    let test_pos_board = Board::from_str("r1k5/p6p/2np4/4p3/8/1PB1nr2/P1P1N2P/R1R3K1 w - - 0 25").expect("E");

    let initial_eval = calculate_position(&test_pos_board);
    
    println!("Initial evaluation value: {}", initial_eval);

    let depth = 4;
    let optimizing_color = board.side_to_move();
    let alpha = -999999.0;
    let beta = -alpha;

    let color = if optimizing_color == Color::White {1} else {-1};

    println!("--------- Actual played move ----------");

    let other_best_move = calc_move(&board, depth, 6, color, alpha, beta, true);

    println!("--------- Checked-out move ------------");

    let test_pos_move = calc_move(&test_pos_board, depth - 1, 6, -color, alpha, beta, true);

    unsafe {println!("Moves searched: {} -- Nodes pruned: {}", count, pruned); count = 0; pruned = 0;}
    
    // let breakdown = breakdown_line(&board, depth, depth, player, optimizing_color, alpha, beta); 
    
    // println!("{:#?}", breakdown);
}

fn create_board_from_pgn(pgn: String) -> Board{
    let mut board = Board::default();
    let moves = pgn.split_whitespace();
    let mut index = 0;
    for i in moves {
        if index % 3 == 0 {index += 1; continue;}
        let m = ChessMove::from_san(&board, i).expect("e");
        board = board.make_move_new(m);
        index += 1;
    }
    return board;
}

fn run_game_loop() {
    loop {
        let mut pgn = String::new();
        println!("pgn: ");
        io::stdin().read_line(&mut pgn).expect("Failed to read line");

        let board = create_board_from_pgn(pgn);

        let color = board.side_to_move();

        let mut color = if color == Color::White {1} else {-1};
        let depth = 5;
        let alpha = -INFINITY;
        let beta = INFINITY;
        let max_iterative_deepening_depth = 6;

        let calculated_move = calc_move(&board, depth, max_iterative_deepening_depth, color, alpha, beta, false);

        match calculated_move.best_move {
            Some(i) => {
                println!("{}{}", i.get_source(), i.get_dest());
            },
            _ => ()
        };
    }
}

fn main() {

    // run_tests();
    
    let mut board = Board::default();

    let color = board.side_to_move();

    let mut color = if color == Color::White {1} else {-1};
    let depth = 4;
    let alpha = -INFINITY;
    let beta = INFINITY;
    let max_iterative_deepening_depth = 6;

    let mut pgn = "".to_owned();
    let mut safe_pgn = "".to_owned();

    for i in 0..200 {

        let calculated_move = calc_move(&board, depth, max_iterative_deepening_depth, color, alpha, beta, false);

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