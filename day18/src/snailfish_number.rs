#[derive(PartialEq, Debug)]
pub struct SnailFishNumber {
  pub lhs: Expression,
  pub rhs: Expression,
}

#[derive(PartialEq, Debug)]
pub enum Expression {
   Digit(u8),
   Pair(Box<Expression>, Box<Expression>)
}

impl Expression {}
