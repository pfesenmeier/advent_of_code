use std::char;

#[derive(Clone, Copy, PartialEq)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

pub struct AnalogDigit {
    signal_wires: Vec<char>,
    possible_digits: Vec<Digit>,
}

#[derive(Default)]
pub struct Display {
    top: Option<char>,
    top_left: Option<char>,
    top_right: Option<char>,
    middle: Option<char>,
    bottom_left: Option<char>,
    bottom_right: Option<char>,
    botton: Option<char>,
}

impl Display {
    fn new() -> Self {
       Display::default()
    }
}

impl AnalogDigit {
    pub fn new(signal_wires: Vec<char>) -> AnalogDigit {
        let possible_digits = match signal_wires.len() {
            2 => vec![Digit::One],
            4 => vec![Digit::Four],
            3 => vec![Digit::Seven],
            7 => vec![Digit::Eight],
            _ => vec![
                Digit::Zero,
                Digit::Two,
                Digit::Three,
                Digit::Five,
                Digit::Six,
                Digit::Nine,
            ],
        };
        AnalogDigit {
            possible_digits,
            signal_wires,
        }
    }

    pub fn get_display(&self) -> Option<Digit> {
        match self.possible_digits.len() {
            1 => Some(self.possible_digits[0]),
            _ => None,
        }
    }
}
