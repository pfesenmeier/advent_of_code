use std::fmt::Display;
use std::fmt::Formatter;

pub struct BingoBoard<Stage = Loading> {
    stage: std::marker::PhantomData<Stage>,
    board: [[(u8, bool); 5]; 5],
    num_rows: usize,
}

pub struct Loading;
pub struct Ready;

impl Default for BingoBoard<Loading> {
    fn default() -> Self {
        BingoBoard::<Loading> {
            stage: std::marker::PhantomData::<Loading>,
            board: [[(0, false); 5]; 5],
            num_rows: 0,
        }
    }
}

impl BingoBoard<Loading> {
    pub fn new() -> Self {
        BingoBoard::default()
    }

    pub fn load(&mut self, row: impl Iterator<Item = u8>) {
        for (i, column) in row.enumerate() {
            self.board[self.num_rows][i] = (column, false)
        }
        assert_eq!(self.board[self.num_rows].len(), 5);
        self.num_rows += 1;
    }

    pub fn complete(self) -> BingoBoard<Ready> {
        if self.num_rows != 5 {
            panic!("Board attempted to be completed before 5 rows were entered");
        }
        BingoBoard::<Ready> {
            stage: std::marker::PhantomData::<Ready>,
            board: self.board,
            num_rows: self.num_rows,
        }
    }
}

impl BingoBoard<Ready> {
    pub fn set_square(&mut self, value: u8) {
        for row in 0..5 {
            for column in 0..5 {
                if self.board[row][column].0 == value {
                   self.board[row][column].1 = true;
                }
            }
        }
    }
    pub fn check_bingo(&self) -> bool {
        self.check_bingo_rows() || self.check_bingo_columns()
    }
    fn check_bingo_rows(&self) -> bool {
        self.board
            .iter()
            .any(|row| row.iter().all(|column| column.1 == true))
    }
    fn check_bingo_columns(&self) -> bool {
        for column in 0..5 {
            let mut bingo = true;
            for row in 0..5 {
                if self.board[row][column].1 == false {
                    bingo = false;
                }
            }
            if bingo == true {
                return true;
            }
        }
        false
    }

    fn get_unmarked(&self) -> Vec<u8> {
        self.board
            .into_iter()
            .flatten()
            .filter(|column| column.1 == false)
            .map(|(number, _)| number)
            .collect()
    }

    pub fn sum_unmarked(&self) -> u32 {
        self.get_unmarked().into_iter()
        .map(|num| num as u32)
        .collect::<Vec<u32>>()
        .into_iter()
        .sum()
    }
}

impl<T> Display for BingoBoard<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut output = "".to_string();
        for row in self.board {
            let mut tile: String = "".to_string();
            for column in row {
                let num = match column.0 {
                    0..=9 => " ".to_owned() + &column.0.to_string(),
                    _ => column.0.to_string(),
                };
                tile.push_str(&num);
                tile.push('[');
                let mark = match column.1 {
                    true => 'x',
                    false => ' ',
                };
                tile.push(mark);
                tile.push(']');
                tile.push(' ');
            }
            tile.pop();
            output.push_str(&tile);
            output.push('\n');
        }
        output.pop();
        write!(f, "{}", output)
    }
}

#[test]
fn test_sample() {
    let (rounds, mut boards) = bingo_parser(include_str!("sample_input.txt"));

    for round in rounds {
        for board in &mut boards {
            board.set_square(round.try_into().unwrap());
            if board.check_bingo() {
                let sum_unmarked = board.sum_unmarked();
                assert_eq!(sum_unmarked, 188);
                assert_eq!(round, 24);
                let answer = sum_unmarked * round;
                assert_eq!(answer, 4512);
            }
        }
    }
}
