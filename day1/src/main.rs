use itertools::Itertools;

fn main() {
    println!("{:?}", part_2());
}

fn part_2() -> u32 {
    include_str!("input.txt")
        .lines()
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .tuple_windows::<(_, _, _)>()
        .map(|window| window.0 + window.1 + window.2)
        .tuple_windows::<(_, _)>()
        .fold(
            0,
            |acc, (first, second): (u32, u32)| {
                if first < second {
                    acc + 1
                } else {
                    acc
                }
            },
        )
}
