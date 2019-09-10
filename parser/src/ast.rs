use std::fmt::{Debug, Error, Formatter};


#[derive(Debug)]
pub enum Node {
    Number(i32),
    Var(String),
    Op(Box<Node>, Opcode, Box<Node>),
    Let{var: Box<Node>, expr: Box<Node>},
    Error
}

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}