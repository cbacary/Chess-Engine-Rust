mod brain;

use std::str::FromStr;
use brain::{calculate_move, calculate_position, generate_pgn, breakdown_line, calc_move};
use chess::{Board, MoveGen, Color, BoardStatus, ChessMove, Square, Game};

fn run_tests() 
{
    let board = Board::from_str("4r2k/1rp2ppp/p7/8/1P6/8/P1R3PP/2KR4 w - - 15 40").expect("E");
    
    let initial_eval = calculate_position(&board);
    
    println!("Initial evaluation value: {}", initial_eval);

    let depth = 4;
    let player = true;
    let optimizing_color = Color::White;
    let alpha = -999999.0;
    let beta = -alpha;

    let best_move = calculate_move(&board, depth, depth, player, optimizing_color, alpha, beta);

    let other_best_move = calc_move(&board, depth, depth, optimizing_color, alpha, beta, true);
    
    // let breakdown = breakdown_line(&board, depth, depth, player, optimizing_color, alpha, beta);
    
    // println!("{:#?}", breakdown);
    
    println!("{:#?}", best_move);

    match other_best_move.best_move {
        Some(i) => println!("{}{}", i.get_source(), i.get_dest()),
        _ => ()
    };
}

fn main() {

    run_tests();

    // let mut board = Board::default();

    // let mut player = true;
    // let mut optimizing_color = Color::White;

    // let mut pgn = "".to_owned();
    // let mut safe_pgn = "".to_owned();

    // for x in 1..100 {

    //     if board.status() != BoardStatus::Ongoing {
    //         break;
    //     }

    //     let calculated_move = calculate_move(&board, 4, 4, player, optimizing_color, -999999.0, 999999.0);

    //     safe_pgn = format!("{pgn}");
    //     pgn = generate_pgn(&board, &calculated_move.best_move, optimizing_color, &pgn, x);

    //     match calculated_move.best_move {
    //         Some(i) => {
    //             // Make move
    //             board = board.make_move_new(i);
    //             println!("{}{}", i.get_source(), i.get_dest());
    //         },
    //         None => {println!("chess move invalid"); break;} 
    //     }
    //     player = !player;
    //     if optimizing_color == Color::White {optimizing_color = Color::Black;} else {optimizing_color = Color::White;}
    // }
    // println!("{}", format!("{board}"));
    // println!("{}", safe_pgn);
}