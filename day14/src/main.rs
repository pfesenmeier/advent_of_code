use std::{cmp::Ordering, collections::HashMap};

use crate::parser::parse_insertions;
use itertools::{Itertools, MinMaxResult::MinMax};

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_2(input: &str) -> usize {
    let (template, insertions) = input.split_once("\n\n").unwrap();
    let insertions = parse_insertions(insertions);

    let mut pair_map = HashMap::new();
    for window in template.chars().tuple_windows::<(_, _)>() {
        let value = pair_map.entry(window).or_insert(0);
        *value += 1;
    }

    for _ in 0..40 {
        let mut adds = HashMap::new();
        let mut subs = HashMap::new();
        for insertion in &insertions {
            let pattern = (insertion.0, insertion.1);
            if let Some(value) = pair_map.get(&pattern) {
                if *value > 0 {
                    let occurences = (*value).clone();
                    let value = subs.entry(pattern).or_insert(0);
                    *value += occurences;
                    let value = adds.entry((insertion.0, insertion.2)).or_insert(0);
                    *value += occurences;
                    let value = adds.entry((insertion.2, insertion.1)).or_insert(0);
                    *value += occurences;
                }
            }
        }

        for (pattern, amt) in subs.into_iter() {
            let value = pair_map.entry(pattern).or_insert(0);
            *value -= amt;
        }

        for (pattern, amt) in adds.into_iter() {
            let value = pair_map.entry(pattern).or_insert(0);
            *value += amt;
        }
    }

    let mut count_map = HashMap::new();

    for ((first, last), amt) in pair_map.iter() {
        let value = count_map.entry(first).or_insert(0);
        *value += amt;
        let value = count_map.entry(last).or_insert(0);
        *value += amt;
    }

    if let MinMax((a, min), (b, max)) = count_map.iter().minmax_by(|(_, first), (_, second)| {
        if first > second {
            Ordering::Greater
        } else if first < second {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }) {
        let first_char = template.chars().into_iter().next().unwrap();
        let last_char = template.chars().into_iter().rev().next().unwrap();
        println!("First char is {}", first_char);
        println!("Last char is {}", last_char);
        let mut min_adjustment = 0;
        let mut max_adjustment = 0;
        if **a == first_char {
            min_adjustment += 1
        }
        if **a == last_char {
            min_adjustment += 1
        }
        if **b == first_char {
            max_adjustment += 1
        }
        if **b == last_char {
            max_adjustment += 1
        }
        let min = (min + min_adjustment) / 2;
        let max = (max + max_adjustment) / 2;
        println!("Min {} occurred {} times", a, min);
        println!("Max {} occurred {} times", b, max);
        max - min
    } else {
        0
    }
}

fn part_1(input: &str) -> usize {
    let (template, steps) = input.split_once("\n\n").unwrap();
    let steps = parse_insertions(steps);

    let mut step_result = template.to_string();
    for _ in 0..10 {
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

    #[test]
    fn part_2_should_pass_with_sample_input() {
        assert_eq!(part_2(include_str!("sample_input.txt")), 2188189693529);
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
