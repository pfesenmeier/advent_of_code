use itertools::Itertools;

fn main() {
    let sweep_depths = include_str!("input.txt");
    let sweep_increments: u32 = sweep_depths
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows()
        .fold(
            0,
            |acc, window: (u32, u32)| {
                if window.0 < window.1 {
                    acc + 1
                } else {
                    acc
                }
            },
        );
    println!("{:?}", sweep_increments);
}
