mod parser;
mod snailfish_number;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

//    #[test]
//    fn part_1_should_pass_on_sample_input() {
//        assert_eq!(part_1(include_str!("sample_input.txt")), 42);
//    }
}
