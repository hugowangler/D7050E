use crate::ast::Node;

#[derive(Debug, PartialEq)]
pub struct Function {
    Name(String),
    Params(Vec<Box<Node>>),
    Args(Vec<Box<Node>>),
    ReturnType(LiteralType)
}