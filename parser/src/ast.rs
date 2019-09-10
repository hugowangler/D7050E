#[derive(Debug, PartialEq)]
pub enum Node {
    Number(i32),
    Var(String),
    Op(Box<Node>, Opcode, Box<Node>),
    Let{var: Box<Node>, expr: Box<Node>},
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}