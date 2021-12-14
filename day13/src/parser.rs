use crate::fold_instructions::FoldInstructions;
use crate::translucent_paper::TranslucentPaper;
// get string, split on "\n\n"
// iterate over pairs, add to map
// write regex for fold instructions

pub fn input_parser(input: &str) -> (TranslucentPaper, FoldInstructions) {
    let (paper, folds) = input.split_once("\n\n").unwrap();

    (paper_parser(paper), instructions_parser(folds))
}

fn paper_parser(paper_input: &str) -> TranslucentPaper {
    todo!()
}

fn instructions_parser(folds_input: &str) -> FoldInstructions {
    todo!()
}
