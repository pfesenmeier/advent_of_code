use itertools::Itertools;

fn main() {
    let sweep_depths = include_str!("input.txt");
    let sweep_increments: u32 = sweep_depths
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows::<(_,_,_)>()
        .map(|window| window.0 + window.1 + window.2)
        .tuple_windows::<(_,_)>()
        .fold(
            0,
            |acc, (first, second): (u32, u32)| {
                if first < second {
                    acc + 1
                } else {
                    acc
                }
            },
        );
   
    println!("{:?}", sweep_increments);
}
