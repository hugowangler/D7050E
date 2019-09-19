use crate::types::{
    LiteralType,
    BoolType
};

use crate::operators::{
    BinOpcode,
    LogOpcode,
    RelOpcode
};

#[derive(Debug, PartialEq)]
pub enum Node {
    Number(i32),
    Bool(BoolType),

    Var(String),
    VarBinding(Box<Node>, LiteralType),

    BinOp(Box<Node>, BinOpcode, Box<Node>),
    LogOp(Box<Node>, LogOpcode, Box<Node>),
    RelOp(Box<Node>, RelOpcode, Box<Node>),

    Let(Box<Node>, Box<Node>),
    If(Box<Node>, Box<Node>),
    IfElse(Box<Node>, Box<Node>, Box<Node>),

    Statement(Box<Node>, Box<Node>)
}