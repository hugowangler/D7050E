use crate::types::Type;

#[derive(Debug, PartialEq)]
pub enum Node {
    Number(i32),
    Var(String),
    AssignVar{name: String, var_type: Type},
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