use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(PartialEq, Debug)]
pub enum Expression {
    Digit(u8),
    Pair(Box<Expression>, Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Expression::Pair(l, r) => write!(f, "[{},{}]", l, r),
            Expression::Digit(d) => write!(f, "{}", d),
        }
    }
}

impl Add for Expression {
    type Output = Expression;
    fn add(self, rhs: Expression) -> <Self as Add<Expression>>::Output {
        Expression::Pair(Box::new(self), Box::new(rhs))
    }
}

impl Expression {
    pub fn reduce(&mut self, depth: u8) -> Option<()> {
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let exp1 = Expression::Pair(
            Box::new(Expression::Digit(4)),
            Box::new(Expression::Digit(5)),
        );
        let exp2 = Expression::Pair(
            Box::new(Expression::Digit(6)),
            Box::new(Expression::Digit(7)),
        );

        assert_eq!(format!("{}", exp1 + exp2), "[[4,5],[6,7]]");
    }

    #[test]
    fn simple() {
        let exp = Expression::Pair(
            Box::new(Expression::Digit(4)),
            Box::new(Expression::Digit(5)),
        );
        assert_eq!(format!("{}", exp), "[4,5]");
    }

    #[test]
    fn pairs() {
        let exp = Expression::Pair(
            Box::new(Expression::Pair(
                Box::new(Expression::Digit(4)),
                Box::new(Expression::Digit(5)),
            )),
            Box::new(Expression::Digit(5)),
        );
        assert_eq!(format!("{}", exp), "[[4,5],5]");
    }
}
