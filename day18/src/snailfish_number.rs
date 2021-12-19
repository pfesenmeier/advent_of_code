#[derive(PartialEq, Debug)]
pub enum Expression {
   Digit(u8),
   Pair(Box<Expression>, Box<Expression>)
}

impl Expression {
    
}
