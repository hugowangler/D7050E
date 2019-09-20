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
    VarValue(Box<Node>, Box<Node>),

    BinOp(Box<Node>, BinOpcode, Box<Node>),
    LogOp(Box<Node>, LogOpcode, Box<Node>),
    RelOp(Box<Node>, RelOpcode, Box<Node>),

    Let(Box<Node>, Box<Node>),
    If{cond: Box<Node>, statement: Box<Node>},
    IfElse{cond: Box<Node>, if_statement: Box<Node>, else_statement: Box<Node>},
    While{cond: Box<Node>, statement: Box<Node>},

    // Loop modifiers
    Break,
    Continue,

    Return(Box<Node>),

    Statement(Box<Node>, Box<Node>)
}