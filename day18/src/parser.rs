use peg;
use crate::snailfish_number::{Expression, SnailFishNumber};

pub fn parse_snailfish_number(number: &str) -> SnailFishNumber {
    snailfish_number::parse(number).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_digits() {
        assert_eq!(
            parse_snailfish_number("[4,5]"),
            SnailFishNumber {
                lhs: Expression::Digit(4),
                rhs: Expression::Digit(5)
            }
        )
    }

    #[test]
    fn one_digit_one_pair() {
        assert_eq!(
            parse_snailfish_number("[4,[2,3]]"),
            SnailFishNumber {
                lhs: Expression::Digit(4),
                rhs: Expression::Pair(
                    Box::new(Expression::Digit(2)),
                    Box::new(Expression::Digit(3))
                )
            }
        )
    }

    #[test]
    fn two_pairs() {
        assert_eq!(
            parse_snailfish_number("[[4,5],[2,3]]"),
            SnailFishNumber {
                lhs: Expression::Pair(
                    Box::new(Expression::Digit(4)),
                    Box::new(Expression::Digit(5))
                ),
                rhs: Expression::Pair(
                    Box::new(Expression::Digit(2)),
                    Box::new(Expression::Digit(3))
                )
            }
        )
    }

    #[test]
    fn sample() {
        assert_eq!(
            parse_snailfish_number("[[[[4,3],4],4],[7,[[8,4],9]]]"),
            SnailFishNumber {
                lhs: Expression::Pair(
                    Box::new(Expression::Pair(
                         Box::new(Expression::Pair(
                              Box::new(Expression::Digit(4)),
                              Box::new(Expression::Digit(3)),
                         )),
                         Box::new(Expression::Digit(4)),
                    )),
                    Box::new(Expression::Digit(4)),
                ),
                rhs: Expression::Pair(
                        Box::new(Expression::Digit(7)),
                        Box::new(Expression::Pair(
                                 Box::new(Expression::Pair(
                                    Box::new(Expression::Digit(8)),
                                    Box::new(Expression::Digit(4)),
                                 )),
                                 Box::new(Expression::Digit(9)),
                        )),
                    ),
             }
);
    }

}

peg::parser! {
   grammar snailfish_number() for str {
    pub rule parse() -> SnailFishNumber
        = "[" lhs:expression() "," rhs:expression() "]" { SnailFishNumber { lhs, rhs }}

    rule expression() -> Expression
        = pair() / digit()

    rule digit() -> Expression
        = d:$(['0'..='9']+) { Expression::Digit(d.parse().unwrap()) }
    rule pair() -> Expression
        = "[" lhs:expression() "," rhs:expression() "]" { Expression::Pair(Box::new(lhs), Box::new(rhs)) }
 }
}
