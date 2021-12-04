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
        todo! {}
    }
    pub fn check_bingo(&self) -> bool {
        todo! {}
    }
    fn check_bingo_rows(&self) -> bool {
        todo! {}
    }
    fn check_bingo_columns(&self) -> bool {
        todo! {}
    }
    fn get_unmarked(&self) -> Vec<u8> {
        todo! {}
    }
    pub fn sum_unmarked(&self) -> u32 {
        todo! {}
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
