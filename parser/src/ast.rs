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
    If{cond: Box<Node>, statement: Box<Node>}, // @Args cond, statement
    IfElse{cond: Box<Node>, if_statement: Box<Node>, else_statement: Box<Node>},

    Statement(Box<Node>, Box<Node>)
}