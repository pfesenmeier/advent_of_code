use crate::fold_instructions::Fold;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct TranslucentPaper {
    dots: Vec<(u32, u32)>,
}

impl TranslucentPaper {
    pub fn new(dots: Vec<(u32, u32)>) -> Self {
        Self { dots }
    }

    pub fn fold(&mut self, instruction: Fold) {
        match instruction {
            Fold::Up(line) => self.fold_up(line),
            Fold::Left(line) => self.fold_left(line),
        }
    }

    fn fold_up(&mut self, line: u32) {
        for dot in self.dots.iter_mut() {
            if dot.1 > line {
                let diff = dot.1 - line;
                dot.1 = line - diff;
            }
        }
    }

    fn fold_left(&mut self, line: u32) {
        for dot in self.dots.iter_mut() {
            if dot.0 < line {
                let diff = line - dot.0;
                dot.0 = line + diff;
            }
        }
    }

    pub fn count_dots(&mut self) -> usize {
        self.dots.sort();
        self.dots.dedup();
        self.dots.iter().count()
    }

    pub fn read(&mut self) -> String {
        self.dots.sort();
        self.dots.dedup();

        let max_x = self
            .dots
            .iter()
            .max_by(|first, second| {
                if first.0 > second.0 {
                    Ordering::Greater
                } else if first.0 < second.0 {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .unwrap()
            .0;

        let min_x = self
            .dots
            .iter()
            .min_by(|first, second| {
                if first.0 > second.0 {
                    Ordering::Greater
                } else if first.0 < second.0 {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .unwrap()
            .0;

        let max_y = self
            .dots
            .iter()
            .max_by(|first, second| {
                if first.1 > second.1 {
                    Ordering::Greater
                } else if first.1 < second.1 {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .unwrap()
            .1;

        let mut display = "".to_string();
        for y in 0..=max_y {
            for x in min_x..=max_x {
                if self.dots.contains(&(x, y)) {
                    display.push('#');
                } else {
                    display.push('.');
                }
            }
            display.push('\n');
        }

        display
    }
}
