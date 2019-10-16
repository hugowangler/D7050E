use std::fmt;

use crate::{operators::Opcode, types::LiteralType};

#[derive(Debug, PartialEq)]
pub struct TypeErrors {
    pub errors: Vec<ErrorKind>,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    OpWrongType {
        op: Opcode,
        typ: LiteralType,
    },
    MismatchedTypesVar {
        var: String,
        expected: LiteralType,
        found: LiteralType,
    },
    MismatchedTypesOp {
        op: Opcode,
        expected: LiteralType,
        found: LiteralType,
    },
    VarNotInScope {
        var: String,
    },
    VarImmut {
        var: String,
    },
    FnNotInScope {
        name: String,
    },
    FnNumParamMismatch {
        name: String,
        takes: usize,
        supplied: usize,
    },
    FnParamTypeMismatch {
        name: String,
        param: String,
        expected: LiteralType,
        found: LiteralType,
    },
	FnReturnMismatch {
		name: String,
		expected: LiteralType,
		found: LiteralType,
	}
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
                    "Logical operation '{}' cannot be applied to type '{}'",
                    op.to_string(),
                    typ.to_string()
                ),
                Opcode::EQ | Opcode::NEQ | Opcode::GT | Opcode::LT | Opcode::LEQ | Opcode::GEQ => {
                    write!(
                        f,
                        "Relational operation '{}' cannot be applied to type '{}'",
                        op.to_string(),
                        typ.to_string()
                    )
                }
            },
            ErrorKind::MismatchedTypesVar {
                var,
                expected,
                found,
            } => write!(
                f,
                "Mismatched type for variable '{}': expected {}, found {}",
                var,
                expected.to_string(),
                found.to_string()
            ),
            ErrorKind::MismatchedTypesOp {
                op,
                expected,
                found,
            } => write!(
                f,
                "Mismatched type for operation '{}': expected {}, found {}",
                op.to_string(),
                expected.to_string(),
                found.to_string()
            ),
            ErrorKind::VarNotInScope { var } => {
                write!(f, "Cannot find value '{}' in this scope", var.to_string())
            }
            ErrorKind::VarImmut { var } => {
                write!(f, "Cannot assign twice to immutable variable '{}'", var)
            }
            ErrorKind::FnNotInScope { name } => {
                write!(f, "Cannot find function '{}' in scope", name.to_string())
            }
            ErrorKind::FnNumParamMismatch {
                name,
                takes,
                supplied,
            } => write!(
                f,
                "Function '{}' takes '{}' parameters but '{}' was supplied",
                name, takes, supplied
            ),
            ErrorKind::FnParamTypeMismatch {
                name,
                param,
                expected,
                found,
            } => write!(
				f,
				"Mismatched type of parameter '{}' when calling function '{}': expected '{}' but found '{}'",
				param,
				name,
				expected.to_string(),
				found.to_string(),
			),
			ErrorKind::FnReturnMismatch {
				name,
				expected,
				found,
			} => write!(
				f,
				"Mismatched type for function '{}': expected '{}' because of return type but found '{}'",
				name,
				expected.to_string(),
				found.to_string(),
			),
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
