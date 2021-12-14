use crate::fold_instructions::Fold;
use crate::translucent_paper::TranslucentPaper;
use lazy_static::lazy_static;
use regex::Regex;

pub fn input_parser(input: &str) -> (TranslucentPaper, Vec<Fold>) {
    let (paper, folds) = input.split_once("\n\n").unwrap();

    (paper_parser(paper), instructions_parser(folds))
}

fn paper_parser(paper_input: &str) -> TranslucentPaper {
    let dots = paper_input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (str::parse::<u32>(x).unwrap(), str::parse::<u32>(y).unwrap()))
        .collect::<Vec<(u32, u32)>>();

    TranslucentPaper::new(dots)
}

fn instructions_parser(folds_input: &str) -> Vec<Fold> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^fold along ([xy])=([0-9]+)$").unwrap();
    }

    folds_input
        .lines()
        .map(|line| RE.captures(line).unwrap())
        .map(|cap| (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()))
        .map(|(dir, location)| {
            (
                str::parse::<char>(dir).unwrap(),
                str::parse::<u32>(location).unwrap(),
            )
        })
        .map(|(dir, location)| match dir {
            'x' => Fold::Left(location),
            'y' => Fold::Up(location),
            _ => unreachable!(),
        })
        .collect::<Vec<Fold>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_throw() {
        let p = input_parser(include_str!("sample_input.txt"));
        dbg!(p);
    }
}
