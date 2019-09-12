#[derive(Debug, PartialEq)]
pub enum BinOpcode {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Debug, PartialEq)]
pub enum LogOpcode {
    AND,
    OR,
    NOT
}

#[derive(Debug, PartialEq)]
pub enum RelOpcode {
    EQ,
    NEQ,
    GT,
    LT,
    LEQ,
    GEQ
}