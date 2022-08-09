use chess::{Board, MoveGen};
use std::time::Instant;

#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn run_tests() {
    let board = Board::default();
    //run_perft_test(&mut board, 5);
    run_perft_test_correct(&board, 5);

    let now = Instant::now();
    for _ in 0..3 {
        run_perft_test_correct(&board, 5);
    }
    let elapsed_time = now.elapsed();
    println!("{}", elapsed_time.as_nanos() / 1_000_000);

    let now = Instant::now();
    for _ in 0..3 {
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

#[allow(dead_code)]
pub fn convert_u64_to_bin(num: u64) -> u64 {
    let mut num = num;
    let mut s = String::from("");
    while num > 0 {
        if num % 2 == 0 {
            s.push('0');
        } else {
            s.push('1');
        }
        num /= 2;
    }
    let sf: String = s.chars().rev().collect();
    println!("{}", sf);
    let mut index = 0;
    for i in s.chars() {
        print!(" | {} |", i);
        index += 1;
        if index == 8 {
            index = 0;
            println!("");
        }
    }
    for _ in 0..(64 - sf.len()) {
        print!(" | 0 |");
        index += 1;
        if index == 8 {
            index = 0;
            println!("");
        }
    }
    println!("");
    let mut result: u64 = 0;
    let mut index = 0;
    let base: u64 = 2;
    for i in sf.chars().rev() {
        let a = i.to_digit(10).unwrap();
        if a == 1 {
            result += base.pow(index);
        }
        index += 1;
    }
    return result;
}
