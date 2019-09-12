use crate::types::LiteralType;
use crate::operators::{
    BinOpcode,
    BoolOpcode
};

#[derive(Debug, PartialEq)]
pub enum Node {
    Number(i32),
    Var(String),
    AssignVar{name: String, var_type: LiteralType},
    BinOp(Box<Node>, BinOpcode, Box<Node>),
    Let{var: Box<Node>, expr: Box<Node>},
}