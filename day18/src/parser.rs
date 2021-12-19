use crate::snailfish_number::{Expression};
use peg;

pub fn parse_snailfish_number(number: &str) -> Expression {
    snailfish_number::parse(number).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_digits() {
        assert_eq!(
            parse_snailfish_number("[4,5]"),
            Expression::Pair(
                Box::new(Expression::Digit(4)),
                Box::new(Expression::Digit(5))
            )
        )
    }

    #[test]
    fn one_digit_one_pair() {
        assert_eq!(
            parse_snailfish_number("[4,[2,3]]"),
            Expression::Pair(
                Box::new(Expression::Digit(4)),
                Box::new(Expression::Pair(
                    Box::new(Expression::Digit(2)),
                    Box::new(Expression::Digit(3))
                ))
            )
        )
    }

    #[test]
    fn two_pairs() {
        assert_eq!(
            parse_snailfish_number("[[4,5],[2,3]]"),
            Expression::Pair(
                Box::new(Expression::Pair(
                    Box::new(Expression::Digit(4)),
                    Box::new(Expression::Digit(5))
                )),
                Box::new(Expression::Pair(
                    Box::new(Expression::Digit(2)),
                    Box::new(Expression::Digit(3))
                ))
            )
        )
    }

    #[test]
    fn sample() {
        assert_eq!(
            parse_snailfish_number("[[[[4,3],4],4],[7,[[8,4],9]]]"),
            Expression::Pair(
                Box::new(Expression::Pair(
                    Box::new(Expression::Pair(
                        Box::new(Expression::Pair(
                            Box::new(Expression::Digit(4)),
                            Box::new(Expression::Digit(3)),
                        )),
                        Box::new(Expression::Digit(4)),
                    )),
                    Box::new(Expression::Digit(4)),
                )),
                Box::new(Expression::Pair(
                    Box::new(Expression::Digit(7)),
                    Box::new(Expression::Pair(
                        Box::new(Expression::Pair(
                            Box::new(Expression::Digit(8)),
                            Box::new(Expression::Digit(4)),
                        )),
                        Box::new(Expression::Digit(9)),
                    )),
                )),
            )
        );
    }
}

peg::parser! {
   grammar snailfish_number() for str {
    pub rule parse() -> Expression
        = p:pair() { p }

    rule expression() -> Expression
        = pair() / digit()

    rule digit() -> Expression
        = d:$(['0'..='9']+) { Expression::Digit(d.parse().unwrap()) }

    rule pair() -> Expression
        = "[" lhs:expression() "," rhs:expression() "]" { Expression::Pair(Box::new(lhs), Box::new(rhs)) }
 }
}
