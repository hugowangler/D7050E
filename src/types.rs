use crate::ast::Node;

#[derive(Debug, PartialEq)]
pub enum LiteralType {
    Bool,
    Char,

    // Singed
    I8,
    I16,
    I32,
    I64,
    Isize,

    // Unsigned
    U8,
    U16,
    U32,
    U64,
    Usize,

    // Floats
    F32,
    F64,

    // Strings
    Str,

    None
}

#[derive(Debug, PartialEq)]
pub enum Function {
    Name(String),
    Params(Vec<Box<Node>>),
    Args(Vec<Box<Node>>),
    ReturnType(LiteralType)
}