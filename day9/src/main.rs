mod lib;

use lib::Checker;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_2(input: &str) -> usize {
    let mut risk: usize = 0;

    let input: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let checker = Checker::new(&input);

    for width in 0..input[0].len() {
        for length in 0..input.len() {
            if let Some(local_min) = checker.check_local_min(width, length) {
                risk += local_min + 1;
            }
        }
    }

    risk
}

fn part_1(input: &str) -> usize {
    let mut risk: usize = 0;

    let input: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let checker = Checker::new(&input);

    for width in 0..input[0].len() {
        for length in 0..input.len() {
            if let Some(local_min) = checker.check_local_min(width, length) {
                risk += local_min + 1;
            }
        }
    }

    risk
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(include_str!("sample_input.txt")), 15);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(include_str!("sample_input.txt")), 1134);
}
