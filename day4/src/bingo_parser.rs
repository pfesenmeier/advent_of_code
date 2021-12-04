use crate::bingo_board::{BingoBoard, Ready};

pub fn bingo_parser(input: &str) -> (Vec<u32>, Vec<BingoBoard<Ready>>) { 
   let mut lines = input.lines().peekable();

   let rounds = lines.by_ref().take(2).collect::<Vec<&str>>()[0];
   let rounds = rounds.split_terminator(',').map(|num| str::parse::<u32>(num).unwrap()).collect();

   let mut boards: Vec<BingoBoard<Ready>> = vec![];

   while lines.peek() != None {
       let rows: Vec<Vec<u8>> = lines
        .by_ref()
        .inspect(|x| println!("rows: {}", x))
        .take_while(|line| *line != "")
        .map(|row| row.split_whitespace().map(|column| str::parse::<u8>(column).unwrap()).collect())
        .collect();
       dbg!(&rows);

       let mut board = BingoBoard::new();
       for row in rows {
          board.load(row.into_iter());
       }
       boards.push(board.complete());
    }
   (rounds, boards)
}

#[test]
fn test_fn() {
    let (rounds, boards) = bingo_parser(include_str!("parse_input.txt"));
    assert_eq!(rounds, vec![7,4,9,5,11,17]);
    assert_eq!(format!("{}", boards[0]), 
r"22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19");
}
