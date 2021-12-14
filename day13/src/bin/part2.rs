use day13::parser::input_parser;

fn main() {
    println!("{}", part_2("./src/input.txt"));
}

fn part_2(path: &str) -> String {
    let (mut translucent_paper, fold_instructions) =
        input_parser(&std::fs::read_to_string(path).unwrap());

    for fold in fold_instructions.into_iter() {
        translucent_paper.fold(fold);
    }

    translucent_paper.read()
}
