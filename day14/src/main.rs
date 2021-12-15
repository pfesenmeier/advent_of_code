use std::{cmp::Ordering, collections::HashMap};

use crate::parser::parse_insertions;
use itertools::{Itertools, MinMaxResult::MinMax};

fn main() {
    println!("{}", part_1(include_str!("input.txt")));
}

fn part_1(input: &str) -> usize {
    let (template, steps) = input.split_once("\n\n").unwrap();
    let steps = parse_insertions(steps);

    let mut step_result = template.to_string();
    for step in 0..40 {
        println!("step {}", step);

        let mut building_template = step_result.chars().next().unwrap().to_string();
        for window in step_result.chars().tuple_windows::<(_, _)>() {
            for step in &steps {
                if (step.0, step.1) == window {
                    building_template.push(step.2);
                    break;
                }
            }
            building_template.push(window.1);
        }
        step_result = building_template;
    }

    let mut map = HashMap::new();

    for element in step_result.chars() {
        let count = map.entry(element).or_insert(0);
        *count += 1;
    }

    if let MinMax((_, min), (_, max)) = map.iter().minmax_by(|(_, first), (_, second)| {
        if first > second {
            Ordering::Greater
        } else if first < second {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }) {
        max - min
    } else {
         0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_pass_with_sample_input() {
        assert_eq!(part_1(include_str!("sample_input.txt")), 1588);
    }
}

mod pair_insertion {
    #[derive(Debug)]
    pub struct PairInsertion(pub char, pub char, pub char);

    impl PairInsertion {
        pub fn new(a: char, b: char, c: char) -> Self {
            Self(a, b, c)
        }
    }
}

mod parser {
    use crate::pair_insertion::PairInsertion;

    pub fn parse_insertions(steps: &str) -> Vec<PairInsertion> {
        steps
            .lines()
            .map(pair_insertion_parser::insertion)
            .map(Result::unwrap)
            // .inspect(|line| println!("{:?}", line))
            .collect::<Vec<PairInsertion>>()
    }

    peg::parser! {
      grammar pair_insertion_parser() for str {
        pub rule insertion() ->  PairInsertion
            = a:char() b:char() " -> " c:char() { PairInsertion::new(a, b, c) }
        rule char() -> char
            = h:$(['A'..='Z']) {? h.parse().or(Err("char")) }
      }
    }
}
