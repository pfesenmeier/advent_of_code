use day13::fold_instructions::{self, Fold, FoldInstructions};
use day13::parser::input_parser;
use day13::translucent_paper::TranslucentPaper;

fn main() {
    println!("{}", part_1("../input.txt"));
}

fn part_1(path: &str) -> u32 {
    let (mut translucent_paper, fold_instructions) = input_parser(&std::fs::read_to_string(path).unwrap());

    for fold in fold_instructions.into_iter() {
        translucent_paper.fold(fold);
    }

    translucent_paper.count_dots()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("sample_input.txt"), 17);
    }
}
