mod input_parser;
mod number;

use input_parser::input_parser;
use number::{Digit, AnalogDigit};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let input = input_parser(input);
    input
        .into_iter()
        .map(|(_, displayed_digits)| {
            displayed_digits
                .into_iter()
                .filter_map(|digit| digit.get_display())
                .collect::<Vec<Digit>>()
        })
        .flatten()
        .count()
}

fn part_2(input: &str) -> usize {
    //let input = input_parser(input);
    //input
    //    .into_iter()
    //    .map(|(ten_unique_flashes, final_four)| {
    //        let obvious_digits = ten_unique_flashes
    //            .into_iter()
    //            .filter(|digit| digit.get_display() != None)
    //            .collect::<Vec<AnalogDigit>>();

     //   })
     //   .flatten()
     //   .count()
    todo!()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(include_str!("sample_input.txt")), 26);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(include_str!("sample_input.txt")), 61229);
}
