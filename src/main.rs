mod brain;
mod tests;
mod zob_return;
mod pgn;

use pgn::{generate_pgn, create_board_from_pgn};
use brain::{find_move, INFINITY, searched};
use zob_return::ZobristReturn;
use chess::{Board, MoveGen, Color, ChessMove, Square, Game};
use std::io;
use std::time::Instant;
use std::fs::{OpenOptions};
use std::io::Write;
use chrono::offset;
use std::collections::{VecDeque, HashMap};
use fasthash::{RandomState, xx};

struct LogData {
    depth: u8,
    max_iterative_deepening_depth: u8,
    average_time_per_move: u128,
    max_game_length: i32,
    final_game_length: u128,
    initial_fen: String,
    final_fen: String,
    final_pgn: String,

}

// fn run_game_loop() {
//     loop {
//         let mut pgn = String::new();
//         println!("pgn: ");
//         io::stdin().read_line(&mut pgn).expect("Failed to read line");

//         let board = create_board_from_pgn(pgn);

//         let color = board.side_to_move();

//         let mut color = if color == Color::White {1} else {-1};
//         let depth = 4;
//         let alpha = -INFINITY;
//         let beta = INFINITY;
//         let max_iterative_deepening_depth = 6;

//         let calculated_move = find_move(&board, depth, max_iterative_deepening_depth, color, alpha, beta);

//         match calculated_move {
//             Some(i) => {
//                 println!("{}{}", i.get_source(), i.get_dest());
//             },
//             _ => ()
//         };
//     }
// }

fn write_log_data(data: LogData) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("log_data.txt")
        .expect("Failed to open file");

    let date_time = offset::Local::now();
    let dashes = "---------------";
    
    let depth = data.depth;
    let max_iterative_deepening_depth = data.max_iterative_deepening_depth;
    let average_time_per_move = data.average_time_per_move;
    let max_game_length = data.max_game_length;
    let initial_fen = data.initial_fen;
    let final_fen = data.final_fen;
    let final_pgn = data.final_pgn;
    let final_game_length = data.final_game_length;

    let log_data_formatted = format!(
"{{
Depth: {depth}
Max iterative depth: {max_iterative_deepening_depth}
Average time per move (milli): {average_time_per_move}
Max game length: {max_game_length}
Actual game length: {final_game_length}
Initial fen: {initial_fen}
Final fen: {final_fen}
Final PGN: \n{final_pgn}
}}");
    let output = format!("{dashes}{dashes}{dashes}{dashes}\n{dashes}{date_time}
                         {dashes}\n{log_data_formatted}\n");
    println!(":{}", output);
    file.write_all(output.as_bytes()).expect("Something went wrong");
}

fn main() {


    //run_game_loop();

    // I calculated the average moves searched at a depth of 4 with 
    // "iterative deepening," or whatever I should be calling cause it 
    // isn't actually iterative deepenign, and I got a something like 64,000
    // I figured that doing around double the size of that should be okay
    // for a transposition table. 2^17 is the closest base 2 that is double of 64k
    let base: u64 = 2;
    let pow: u32 = 14;

    let capacity = base.pow(pow) as usize;

    let s = RandomState::<xx::Hash64>::new(); 
    
    // Zobrist table stores all the zobrist values as keys    
    let mut zobrist_table: HashMap<u64, ZobristReturn, RandomState<xx::Hash64>> = HashMap::with_hasher(s);

    // This queue will contain all the same zobrists keys stored in 
    // zobrist_table but when this table fills up and we pop_back from this table
    // we will pass the popped value into zobrist_table.remove(popped_value).
    // This avoids indexing zobrists keys.
    let mut scheduled_removal: VecDeque<u64> = VecDeque::with_capacity(capacity);

    let mut game = Game::new();

    let board = game.current_position();

    let instant = Instant::now();

    let _f = MoveGen::movegen_perft_test(&board, 5);

    let e = instant.elapsed();

    println!("{}", e.as_millis());

    let initial_fen = format!("{board}");

    let color = board.side_to_move();

    let mut color = if color == Color::White {1} else {-1};
    let depth = 5;
    let alpha = -INFINITY;
    let beta = INFINITY;
    let max_iterative_deepening_depth = 6;
    let max_game_length = 200;

    let mut pgn = "".to_owned();
    let mut safe_pgn = "".to_owned();

    let mut total_time_milli: u128 = 0;

    let mut c = 0;

    for i in 0..max_game_length {

        if game.can_declare_draw() || !game.result().is_none() {break;}

        let board = game.current_position();

        let now = Instant::now();

        let calculated_move = find_move(&board, depth, 
            max_iterative_deepening_depth, color, alpha, beta, 
            &mut zobrist_table, &mut scheduled_removal, capacity);


        println!("Table: {} -- Removal: {}", zobrist_table.len(), scheduled_removal.len());

        total_time_milli += now.elapsed().as_millis();

        safe_pgn = format!("{pgn}");
        pgn = generate_pgn(&board, &calculated_move, board.side_to_move(), &pgn, i);

        match calculated_move {
             Some(i) => {
                 // Make move
                 game.make_move(i);
                 println!("{}{}", i.get_source(), i.get_dest());
             },
             None => {println!("chess move invalid"); break;} 
        }

        color = -color;
        c += 1;
    }
    

    let average_time_mili = total_time_milli / c;
    unsafe {
        let average_searched = searched / c;
        println!("Average nodes searched per depth: {}", average_searched);
    }
    println!("Total time (milli): {} -- Average time per move (milli): {}", total_time_milli, average_time_mili);
    let board = game.current_position();
    let final_fen = format!("{board}");
    let log_data = LogData {
        depth,
        max_iterative_deepening_depth,
        average_time_per_move: average_time_mili,
        max_game_length,
        final_game_length: c,
        initial_fen,
        final_fen,
        final_pgn: safe_pgn
    };

    write_log_data(log_data);
}
