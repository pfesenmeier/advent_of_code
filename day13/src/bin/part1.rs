use day13::parser::input_parser;

fn main() {
    println!("{}", part_1("./src/input.txt"));
}

fn part_1(path: &str) -> usize {
    let (mut translucent_paper, fold_instructions) =
        input_parser(&std::fs::read_to_string(path).unwrap());

    // for fold in fold_instructions.into_iter() {
    // translucent_paper.fold(fold);
    // }

    let fold = fold_instructions.into_iter().next().unwrap();

    translucent_paper.fold(fold);

    translucent_paper.count_dots()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("./src/sample_input.txt"), 17);
    }
}
