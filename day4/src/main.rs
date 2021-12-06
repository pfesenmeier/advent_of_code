mod bingo_parser;
mod bingo_board;

use bingo_parser::bingo_parser;

fn main() {
    part_1();
} 

fn part_1() {
    let (rounds, mut boards) = bingo_parser(include_str!("input.txt"));

    for round in rounds {
        for board in &mut boards {
            board.set_square(round.try_into().unwrap());
            if board.check_bingo() {
                let sum_unmarked = board.sum_unmarked();
                let answer = sum_unmarked * round;
                println!("Part 1 answer: {}", answer);
                return;
            }
        }
    }
}

