pub struct FoldInstructions {
    folds: Vec<Fold>,
}

pub enum Fold {
    Up(u32),
    Down(u32),
}

impl FoldInstructions {
    pub fn new(tuples: &[(char,char)]) -> Self {
        todo!()
    }
}

impl Iterator for FoldInstructions {
    type Item = Fold;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> { todo!() }
}
