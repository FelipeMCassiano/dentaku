// change that shit later
#[derive(Debug)]
pub enum Operators {
    Plus,
    Minus,
    Times,
    Division,
    Invalid,
}

pub struct Expression {
    pub left: i32,
    pub operator: Operators,
    pub right: i32,
}
