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
    Var(String),
    Bool(BoolType),
    VarBinding{name: String, var_type: LiteralType},
    BinOp(Box<Node>, BinOpcode, Box<Node>),
    LogOp(Box<Node>, LogOpcode, Box<Node>),
    RelOp(Box<Node>, RelOpcode, Box<Node>),
    Let{var: Box<Node>, expr: Box<Node>},
    If(Box<Node>, Box<Node>),
    IfElse(Box<Node>, Box<Node>, Box<Node>)
}