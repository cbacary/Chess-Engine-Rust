use chess::{Board, Color, MoveGen};
use std::time::Instant;

static mut C: i64 = 0;
static mut D: i64 = 0;

#[inline]
//fn run_perft_test(board: &mut Board, depth: u8) {
    ////unsafe {
        ////C += 1;
    ////};
    //if depth == 0 {
        //return;
    //}

    //let moves = MoveGen::new_legal(&board);
    //// Get move color
    //// Get the enemy bitboard
    //// if move.dest() is on enemy bitboard
    //for i in moves {
        //let capture = board.piece_on(i.get_dest());
        //*board = board.make_move_new(i);
        //let ep = board.en_passant();
        //run_perft_test(board, depth - 1);
        //*board = board.unmake_move_new(i, capture, ep);
    //}

    

//}

#[inline]
fn run_perft_test_correct(board: &Board, depth: u8) {
    //unsafe {
        //D += 1;
    //}
    if depth == 0 {
        return;
    }

    let moves = MoveGen::new_legal(&board);

    for m in moves {
        let b = board.make_move_new(m);
        run_perft_test_correct(&b, depth - 1);

    }
}
pub fn run_tests() {
    let mut board = Board::default();
    //run_perft_test(&mut board, 5);
    run_perft_test_correct(&board, 5);

    let now = Instant::now();
    for i in 0..3 {
        run_perft_test_correct(&board, 5);
    }
    let elapsed_time = now.elapsed();
    println!("{}", elapsed_time.as_nanos() / 1_000_000);

    
    let now = Instant::now();
    for i in 0..3 {
        //run_perft_test(&mut board, 5);
    }
    let elapsed_time = now.elapsed();
    println!("{}", elapsed_time.as_nanos() / 1_000_000);
    //unsafe {
        //println!("{}", C);
    //};
    //unsafe {
        //println!("{}", D);
    //}
    assert_eq!(board, Board::default());
}
