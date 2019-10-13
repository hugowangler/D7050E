use std::fmt;

use crate::{operators::Opcode, types::LiteralType};

#[derive(Debug)]
pub struct TypeErrors {
    pub errors: Vec<ErrorKind>,
}

#[derive(Debug)]
pub enum ErrorKind {
    OpWrongType { op: Opcode, typ: LiteralType },
    MismatchedTypes{var: String, expected: LiteralType, found: LiteralType}
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::OpWrongType { op, typ } => match op {
                Opcode::Add | Opcode::Div | Opcode::Sub | Opcode::Mul => write!(
                    f,
                    "Binary operation '{}' cannot be applied to type '{}'",
                    op.to_string(),
                    typ.to_string()
                ),
                Opcode::AND | Opcode::OR => write!(
                    f,
                    "Logical operation '{}' cannot be applied to '{}'",
                    op.to_string(),
                    typ.to_string()
                ),
                Opcode::EQ | Opcode::NEQ | Opcode::GT | Opcode::LT | Opcode::LEQ | Opcode::GEQ => {
                    write!(
                        f,
                        "Relational operation '{}' cannot be applied to '{}'",
                        op.to_string(),
                        typ.to_string()
                    )
                }
            },
            ErrorKind::MismatchedTypes{var, expected, found} => write!(
                f,
                "Mismatched type for variable '{}': expected {}, found {}",
                var,
                expected.to_string(),
                found.to_string()
            )
        }
    }
}

impl TypeErrors {
    pub fn new() -> TypeErrors {
        TypeErrors { errors: vec![] }
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn insert_err(&mut self, err: ErrorKind) {
        self.errors.push(err);
    }
}
