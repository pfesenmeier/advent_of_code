mod bingo_board;
mod bingo_parser;

use bingo_parser::bingo_parser;

fn main() {
    part_1();
    part_2();
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

fn part_2() {
    let (rounds, mut boards) = bingo_parser(include_str!("input.txt"));

    let mut bingos: Vec<bool> = Vec::with_capacity(boards.len());
    for _ in 0..boards.len() {
        bingos.push(false);
    }

    fn num_bingos(bingos: &Vec<bool>) -> usize {
        bingos.iter().filter(|bingo| **bingo == true).count()
    }

    let num_boards = boards.len();
    let all_winners = |bingos: &Vec<bool>| num_bingos(bingos) == num_boards;

    for round in rounds {
        for (i, board) in boards.iter_mut().enumerate() {
            board.set_square(round.try_into().unwrap());

            if board.check_bingo() {
                bingos[i] = true;
            }

            if all_winners(&bingos) {
                let sum_unmarked = board.sum_unmarked();
                let answer = sum_unmarked * round;
                println!("Part 2 answer: {}", answer);
                return;
            }
        }
    }
}
