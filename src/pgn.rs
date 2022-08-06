use chess::{Square, Piece, Rank, File, Board, ChessMove, Color};

/// Generates a pgn
pub fn generate_pgn(
    board: &Board,
    chess_move: &Option<ChessMove>,
    color: Color,
    current_pgn: &String,
    move_number: i32,) -> String {
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
            let full_string = format!(
                "{beginning}{piece_str}{file}{rank}{capture}{destination}{promotion}{check} ");
            let pgn = format!("{current_pgn}{full_string}");

            return pgn;
        }
        None => {
            println!("error");
            return String::from("");
        }
    }
}

pub fn create_board_from_pgn(pgn: String) -> Board{
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