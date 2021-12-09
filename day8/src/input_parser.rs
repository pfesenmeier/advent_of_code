use crate::number::AnalogDigit;

pub fn input_parser(input: &str) -> Vec<(Vec<AnalogDigit>, Vec<AnalogDigit>)> {
    input
        .trim_end()
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|line| {
            (
                line.0.split_whitespace().collect(),
                line.1.split_whitespace().collect(),
            )
        })
        .map(|line: (Vec<&str>, Vec<&str>)| {
            (
                line.0
                    .into_iter()
                    .map(|nums| AnalogDigit::new(nums.chars().collect()))
                    .collect(),
                line.1
                    .into_iter()
                    .map(|nums| AnalogDigit::new(nums.chars().collect()))
                    .collect(),
            )
        })
        .collect()
}
