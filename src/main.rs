mod brain;
mod moveiterator;
mod pgn;
mod tests;
mod zob_return;

use brain::{find_move, INFINITY};
use chess::{Board, ChessMove, Color, Game, MoveGen, Square, EMPTY};
use chrono;
use fasthash::{xx, RandomState};
use moveiterator::MoveIterator;
use pgn::{create_board_from_pgn, generate_pgn};
use std::collections::{HashMap, VecDeque};
use std::fs::OpenOptions;
use std::io::Write;
use std::str::FromStr;
use std::time::Instant;
use std::{io, vec};
use zob_return::ZobristReturn;

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

fn main() {
    //test();
    run_benchmarks();
    //run_game_loop();
}

#[allow(dead_code)]
fn test() {
    let board = Board::from_str("2rk4/pp3p1N/8/1p1b4/5n2/8/PPP2P1P/3RK3 w - - 6 24").expect("er");

    //let board = Board::from_str("rnb1qk1r/pppp1ppp/3n4/4N1B1/6Q1/2P5/PPP2PPP/R3KB1R w KQ - 6 9").expect("er");

    let m = MoveGen::new_legal(&board);
    let masks = vec![*board.color_combined(!board.side_to_move()), !EMPTY];
    let mut o = MoveIterator::new_legal(&board, masks);

    o.set_first_mask(None);

    for i in o {
        println!("{}{}", i.get_source(), i.get_dest());
    }

    println!("\n\n");

    let base: u64 = 2;
    let pow: u32 = 19;

    let capacity = base.pow(pow) as usize;

    let color = board.side_to_move();

    let color = if color == Color::White { 1 } else { -1 };
    let depth = 6;
    let alpha = -INFINITY;
    let beta = INFINITY;
    let max_iterative_deepening_depth = 7;

    let s = RandomState::<xx::Hash64>::new();
    let mut zobrist_table: HashMap<u64, ZobristReturn, RandomState<xx::Hash64>> =
        HashMap::with_hasher(s);
    let mut scheduled_removal: VecDeque<u64> = VecDeque::with_capacity(capacity);

    find_move(
        &board,
        depth,
        max_iterative_deepening_depth,
        color,
        alpha,
        beta,
        &mut zobrist_table,
        &mut scheduled_removal,
        capacity,
        true,
    );
    
    println!("\n\n");

    find_move(
        &board,
        5,
        6,
        color,
        alpha,
        beta,
        &mut zobrist_table,
        &mut scheduled_removal,
        capacity,
        true,
    );

}

#[allow(dead_code)]
fn run_benchmarks() -> Board {
    // I calculated the average moves searched at a depth of 4 with
    // "iterative deepening," or whatever I should be calling cause it
    // isn't actually iterative deepenign, and I got a something like 64,000
    // I figured that doing around double the size of that should be okay
    // for a transposition table. 2^17 is the closest base 2 that is double of 64k
    let base: u64 = 2;
    let pow: u32 = 18;

    let capacity = base.pow(pow) as usize;

    let s = RandomState::<xx::Hash64>::new();

    // Zobrist table stores all the zobrist values as keys
    let mut zobrist_table: HashMap<u64, ZobristReturn, RandomState<xx::Hash64>> =
        HashMap::with_capacity_and_hasher(capacity, s);

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

    let mut color = if color == Color::White { 1 } else { -1 };
    let depth = 6;
    let alpha = -INFINITY;
    let beta = INFINITY;
    let max_iterative_deepening_depth = 7;
    let max_game_length = 200;

    let mut pgn = "".to_owned();
    let mut safe_pgn = "".to_owned();

    let mut total_time_milli: u128 = 0;

    let mut c = 0;

    for i in 0..max_game_length {
        if game.can_declare_draw() || !game.result().is_none() {
            break;
        }

        let board = game.current_position();

        let now = Instant::now();

        let calculated_move = find_move(
            &board,
            depth,
            max_iterative_deepening_depth,
            color,
            alpha,
            beta,
            &mut zobrist_table,
            &mut scheduled_removal,
            capacity,
            false
        );

        let e = now.elapsed().as_millis();

        total_time_milli += e;

        println!(
            "Table: {} -- Removal: {} -- Time for move: {}ms",
            zobrist_table.len(),
            scheduled_removal.len(),
            e
        );

        //zobrist_table.clear();
        //scheduled_removal.clear();

        safe_pgn = format!("{pgn}");
        pgn = generate_pgn(&board, &calculated_move, board.side_to_move(), &pgn, i);

        match calculated_move {
            Some(i) => {
                // Make move
                game.make_move(i);
                println!("{}{}", i.get_source(), i.get_dest());
            }
            None => {
                println!("chess move invalid");
                break;
            }
        }

        color = -color;
        c += 1;
    }

    let average_time_mili = total_time_milli / c;
    println!(
        "Total time (milli): {} -- Average time per move (milli): {}",
        total_time_milli, average_time_mili
    );
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
        final_pgn: pgn,
    };

    write_log_data(log_data);
    game.current_position()
}
#[allow(dead_code)]
fn run_game_loop() {
    loop {
        let mut pgn = String::new();
        println!("pgn: ");
        io::stdin()
            .read_line(&mut pgn)
            .expect("Failed to read line");

        let board = create_board_from_pgn(pgn);

        let color = board.side_to_move();

        let mut color = if color == Color::White { 1 } else { -1 };
        let depth = 6;
        let alpha = -INFINITY;
        let beta = INFINITY;
        let max_iterative_deepening_depth = 7;

        let base: u64 = 2;
        let pow: u32 = 18;

        let capacity = base.pow(pow) as usize;

        let s = RandomState::<xx::Hash64>::new();

        // Zobrist table stores all the zobrist values as keys
        let mut zobrist_table: HashMap<u64, ZobristReturn, RandomState<xx::Hash64>> =
            HashMap::with_hasher(s);

        let mut scheduled_removal: VecDeque<u64> = VecDeque::with_capacity(capacity);

        let calculated_move = find_move(
            &board,
            depth,
            max_iterative_deepening_depth,
            color,
            alpha,
            beta,
            &mut zobrist_table,
            &mut scheduled_removal,
            capacity,
            true
        );

        match calculated_move {
            Some(i) => {
                println!("{}{}", i.get_source(), i.get_dest());
            }
            _ => (),
        };
    }
}

fn write_log_data(data: LogData) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("log_data.txt")
        .expect("Failed to open file");

    let date_time = chrono::offset::Local::now();
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
}}"
    );
    let output = format!(
        "{dashes}{dashes}{dashes}{dashes}\n{dashes}{date_time}
                         {dashes}\n{log_data_formatted}\n"
    );
    println!(":{}", output);
    file.write_all(output.as_bytes())
        .expect("Something went wrong");
}
