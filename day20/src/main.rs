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

    #[test]
    fn part_1_should_pass_on_sample_input() {
        assert_eq!(part_1(include_str!("sample_input.txt")), 42);
    }
}

mod image {
    pub struct Image {}
    pub enum Brightness {
        Dark,
        Light,
    }
}

mod parser {
    use crate::image::{Brightness, Image};
    use std::collections::HashMap;

    pub fn parser(input: &str) -> (HashMap<u32, Brightness>, Image) {
        todo!()
    }
}
