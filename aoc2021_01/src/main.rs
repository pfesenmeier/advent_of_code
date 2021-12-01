use itertools::Itertools;

fn main() {
    let sweep_depths = include_str!("input.txt");
    let sweep_increments: u32 = sweep_depths
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows::<(_,_,_)>()
        .tuple_windows()
        .fold(
            0,
            |acc, (first, second): ((u32, u32, u32), (u32,u32,u32))| {
                let first_sum = first.0 + first.1 + first.2;
                let second_sum = second.0 + second.1 + second.2;
                println!("first: {}", first_sum);
                println!("second: {}", second_sum);
                if first_sum < second_sum {
                    acc + 1
                } else {
                    acc
                }
            },
        );
   
    println!("{:?}", sweep_increments);
}
