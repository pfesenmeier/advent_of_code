use crate::fold_instructions::Fold;

pub struct TranslucentPaper {
    dots: Vec<(u32, u32)>,
}

impl TranslucentPaper {
    pub fn new(dots: &[(u32, u32)]) -> Self {
        todo!()
    }

    pub fn fold(&mut self, instruction: Fold) {
        match instruction {
            Fold::Up(line) => todo!(),
            Fold::Down(line) => todo!(),
        }
    }

    pub fn count_dots(&self) -> u32 {
        todo!()
    }
}

// what is folding
// take dots below or to the left of the line, and put them on other side
// if dot as at x = 1, fold is at x = 3, then location is at 5
// 3 - 1 = 2. 3 + 2 = 5
// if at 0, and fold is at 4, 4 - 0 = 4. 4 + 4 = 8
// array.iter_mut()
// for (foo in array.iter_mut()) {
//    if *foo.0 > x,
//    *foo.0 += abc,
// }
