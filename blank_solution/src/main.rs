fn main() {
    println!("Hello, world!");
}

fn part_1(input: &str) -> usize {
    todo!()
}

fn part_2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_pass_on_sample_input() {
        assert_eq!(part_1(include_str!("sample_input.txt")), 42);
    }

    #[test]
    fn part_2_should_pass_on_sample_input() {
        assert_eq!(part_2(include_str!("sample_input.txt")), 42);
    }
}
