mod lib;

use lib::Checker;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let mut risk: usize = 0;

    let input: Vec<&[u8]> = input.lines().map(|line| line.trim_end().as_bytes()).collect();
    for line in &input {
       for byte in line.iter() {
           let c: char = (*byte).into();
       println!("{}", c);
        }
    }
    let checker = Checker::new(&input);
    for width in 0..input[0].len() {
        for length in 0..input.len() {
            if let Some(local_min) = checker.check_local_min(width, length) {
                risk += local_min as usize + 1;
            }
        }
    }
    risk
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(include_str!("sample_input.txt")), 15);
}
