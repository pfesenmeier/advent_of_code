// iterate over 
// NumberSetBuilder
// One: Option<One>
// ...
// fn  (seven, one, four, eight, Vec<Vec<char>>) get_obvious()
// fn get_three(one, seven, Vec<char>) -> (three, Vec<Vec<char>>)
// fn get_six_nine(one, Vec<char>) -> (six, nine, Vec<char>)
// fn get_two_five(nine, Vec<Vec<char>) -> (two, five, Vec<Vec<char>>)
// displayBuilder::new(NumberSet{ self.One.unwrap() })
// fn display.set_top(one, seven) 
// fn disploy.set_middle(zero, eight)
// fn display.set_top_right(five, nine)
// fn disploy.set_bottom_left(five, six)
// fn display.set_bottom_right(two, three)
// fn display.set_bottom(four, nine) // must be after set top
// fn display.set_top_left()
//
// display.read_input()
// input_reader()
// is_one(Vec<char>)
// i.contains(self.top_right), etc..
// is_two()
// is_three()
// is_four()
// ..
// display.show(input: Vec<char>) -> u8 {
// }
// fn get_all

// len 
// 2 -> 1
// 3 -> 7
// 4 -> 4
// 5 -> 2,3,5
// 6 -> 6 , 9
// 7 -> 8


use std::char;

struct DisplayBuilder {
    Top,
    TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    BottomRight,
    Botton,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Digit {
    Zero,
    One(char,char),
    Two,
    Three,
    Four(char,char,char,char),
    Five,
    Six,
    Seven(char,char,char),
    Eight(char,char,char,char,char,char,char),
    Nine,
    Unknown,
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
    pub fn new() -> Self {
       Display::default()
    }
    
    pub fn set_top(mut self, one: Digit::One, seven: Digit::Seven) {
    }
}

impl AnalogDigit {
    pub fn new(signal_wires: Vec<char>) -> AnalogDigit {
        let possible_digits = match signal_wires[..] {
[f,s] => vec![Digit::One(f,s)],
            [f,s,t,h] => vec![Digit::Four(f,s,t,h)],
            [f,s,t] => vec![Digit::Seven(f,s,t)],
            [f,s,t,h,i,x,v] => vec![Digit::Eight(f,s,t,h,i,x,v)],
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
