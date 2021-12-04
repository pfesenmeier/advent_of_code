mod bingo_parser;
mod bingo_board;

use bingo_parser::bingo_parser;


fn main() {
    let (rounds, mut boards) = bingo_parser(include_str!("parse_input.txt"));

    for round in rounds {
        for board in &mut boards {
            board.set_square(round.try_into().unwrap());
            if board.check_bingo() {
                let sum_unmarked = board.sum_unmarked();
                assert_eq!(sum_unmarked, 188);
                assert_eq!(round, 24);
                let answer = sum_unmarked * round;
                assert_eq!(answer, 4512);
            }
        }
    }
    ()
}

