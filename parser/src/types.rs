#[derive(Debug, PartialEq)]
#[allow(dead_code)]
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
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum BoolType {
    True,
    False
}