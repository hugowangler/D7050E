#[derive(Debug, PartialEq)]
pub enum BinOpcode {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Debug, PartialEq)]
pub enum BoolOpcode {
    AND,
    OR,
}