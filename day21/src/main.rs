fn main() {
    let input = "Player 1 starting position: 4
Player 2 starting position: 8";
    println!("{}", part_1(4, 8));
}

fn part_1(pos1: u32, pos2: u32) -> u32 {
    let die = die::DeterministicDie::new();
    let mut board = game_board::GameBoard::new(die, pos1, pos2);

    loop {
        if let Some((losing_score, rolls)) = board.round() {
            return losing_score * rolls;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_pass_on_sample_input() {
        assert_eq!(part_1(8, 6), 739785);
    }
}

mod game_board {
    use crate::die::{DeterministicDie, Die};
    use std::iter::Sum;

    pub struct Pawn {
        score: u32,
    }

    impl Pawn {
        fn new() -> Self {
            Self {
                score: Default::default(),
            }
        }

        fn add_total(&mut self, amt: u32) {
            self.score += amt
        }

        fn get_score(&self) -> u32 {
            self.score
        }
    }

    pub struct GameBoard<T: Die + Iterator> {
        die: T,
        rolls: u32,
        player1: (Pawn, u32),
        player2: (Pawn, u32),
        winning_score: u32,
    }

    impl<T: Die + Iterator> GameBoard<T>
    where
        u32: Sum<<T as Iterator>::Item>,
    {
        pub fn new(die: T, pos1: u32, pos2: u32) -> Self {
            Self {
                die: die.iter_mut(),
                player1: (Pawn::new(), pos1),
                player2: (Pawn::new(), pos2),
                rolls: Default::default(),
                winning_score: 1000,
            }
        }

        fn calculate_location(pos: u32, roll: u32) -> u32 {
            let modo = (pos + roll) % 10;
            if modo == 0 {
                return 10;
            }
            modo
        }

        pub fn round(&mut self) -> Option<(u32, u32)> {

            let roll1 = GameBoard::<T>::calculate_location(self.player1.1, die.take(3).sum());
            dbg!(roll1);
            self.player1.0.add_total(roll1);
            self.player1.1 = roll1;
            self.rolls += 3;
            if self.player1.0.get_score() >= self.winning_score {
                return Some((self.player2.0.get_score(), self.rolls));
            }

            let roll2 = GameBoard::<T>::calculate_location(self.player2.1, die.take(3).sum());
            dbg!(roll2);
            self.player2.0.add_total(roll2);
            self.player2.1 = roll2;
            self.rolls += 3;
            if self.player2.0.get_score() >= self.winning_score {
                return Some((self.player1.0.get_score(), self.rolls));
            }

            None
        }
    }

    #[test]
    fn modulos() {
        assert_eq!(GameBoard::<DeterministicDie>::calculate_location(4, 6), 10);
        assert_eq!(GameBoard::<DeterministicDie>::calculate_location(8, 15), 3);
    }
}

mod die {
    pub trait Die {
        fn roll(&mut self) -> u32;
    }

    #[derive(Debug)]
    pub struct DeterministicDie {
        next_roll: u32,
    }

    impl DeterministicDie {
        pub fn new() -> Self {
            DeterministicDie { next_roll: 1 }
        }
    }

    impl Die for DeterministicDie {
        fn roll(&mut self) -> u32 {
            let result = self.next_roll;
            self.next_roll = self.next_roll % 100 + 1;

            result
        }
    }

    impl Iterator for DeterministicDie {
        type Item = u32;
        fn next(&mut self) -> Option<<Self as Iterator>::Item> {
            Some(self.roll())
        }
    }

    #[test]
    fn iterator() {
        let mut die = DeterministicDie::new().into_iter();
        assert_eq!(die.next(), Some(1));
    }
}
