use std::collections::HashMap;

use crate::{
    ast::Node,
    context::Context,
    function::Func,
    operators::Opcode,
    scope::Scope,
    type_errors::{ErrorKind, TypeErrors},
    types::LiteralType,
    value::Value,
};

type Funcs = HashMap<String, Func>; // Functions return types
#[allow(dead_code)]
pub fn type_check(mut funcs_ast: Vec<Box<Node>>) -> Result<(), TypeErrors> {
    let mut funcs: Funcs = HashMap::new();
    let mut context: Context = Context::new();
    let mut type_errors: TypeErrors = TypeErrors::new();

    // Declare all of the functions
    for func in funcs_ast.drain(..) {
        func_dec(func, &mut funcs);
    }

    // Type check the functions
    context.push(Scope::new());
    for (_, func) in funcs.clone().iter() {
        visit(
            func.body.clone(),
            &mut context,
            &mut funcs,
            &mut type_errors,
        );
    }

    if type_errors.len() > 0 {
        // println!("{:?}", type_errors);
        return Err(type_errors);
    }
    Ok(())
}

fn func_dec(func: Box<Node>, funcs: &mut Funcs) {
    match *func {
        Node::Func {
            name,
            params,
            r_type,
            body,
        } => {
            let func = Func::new(name.to_string(), params.clone(), r_type, body.clone());
            funcs.insert(name, func);
        }
        _ => unreachable!(),
    }
}

fn visit(
    node: Box<Node>,
    context: &mut Context,
    funcs: &mut Funcs,
    err: &mut TypeErrors,
) -> Result<LiteralType, ()> {
    match *node {
        Node::Number(_) => Ok(LiteralType::I32),
        Node::Bool(_) => Ok(LiteralType::Bool),
        Node::_String(_) => Ok(LiteralType::_String),
        Node::UnaryOp(_, _) => Ok(LiteralType::I32),
        Node::Expr(left, op, right) => expr(
            visit(left, context, funcs, err),
            op,
            visit(right, context, funcs, err),
            err,
        ),
        Node::Let { var, expr, next } => var_dec(
            var,
            visit(expr, context, funcs, err),
            context,
            funcs,
            err,
            next,
        ),
        _ => panic!("Node not supported: {:#?}", *node),
    }
}

fn expr(
    left: Result<LiteralType, ()>,
    op: Opcode,
    right: Result<LiteralType, ()>,
    err: &mut TypeErrors,
) -> Result<LiteralType, ()> {
    // Determine if the left and right sub expressions evaluated to a type
    let (l, r) = match (left, right) {
        (Ok(l_type), Ok(r_type)) => (Some(l_type), Some(r_type)),
        _ => (None, None),
    };

    // If left and right has a type, calculate new type
    if let (Some(left), Some(right)) = (l, r) {
        match op {
            Opcode::Add | Opcode::Sub | Opcode::Div | Opcode::Mul => num_expr(left, op, right, err),
            Opcode::AND | Opcode::OR => log_op(left, op, right, err),
            Opcode::EQ | Opcode::NEQ | Opcode::GT | Opcode::LT | Opcode::LEQ | Opcode::GEQ => {
                rel_op(left, op, right, err)
            }
        }
    } else {
        Err(())
    }
}

fn num_expr(
    left: LiteralType,
    op: Opcode,
    right: LiteralType,
    err: &mut TypeErrors,
) -> Result<LiteralType, ()> {
    if let (LiteralType::I32, LiteralType::I32) = (left, right) {
        return Ok(LiteralType::I32);
    } else {
        err.insert_err(ErrorKind::OpWrongType {
            op: op,
            typ: LiteralType::Bool,
        });
    }
    Err(())
}

fn log_op(
    left: LiteralType,
    op: Opcode,
    right: LiteralType,
    err: &mut TypeErrors,
) -> Result<LiteralType, ()> {
    if let (LiteralType::Bool, LiteralType::Bool) = (left, right) {
        return Ok(LiteralType::Bool);
    } else {
        err.insert_err(ErrorKind::OpWrongType {
            op: op,
            typ: LiteralType::I32,
        });
    }
    Err(())
}

fn rel_op(
    left: LiteralType,
    op: Opcode,
    right: LiteralType,
    err: &mut TypeErrors,
) -> Result<LiteralType, ()> {
    match op {
        Opcode::EQ | Opcode::NEQ => {
            // '==' and '!=' can compare bools or i32s, otherwise type error
            if let (LiteralType::Bool, LiteralType::Bool) | (LiteralType::I32, LiteralType::I32) =
                (left, right)
            {
                return Ok(LiteralType::Bool);
            } else {
				// Relational operations expects right to be same type as left
                err.insert_err(ErrorKind::MismatchedTypesOp {
                    op: op,
                    found: right,
					expected: left
                });
            }
            return Err(());
        }
        Opcode::GT | Opcode::LT | Opcode::LEQ | Opcode::GEQ => {
            // '>', '<', '<=' and '>=' can compare i32s, otherwise type error
            if let (LiteralType::I32, LiteralType::I32) = (left, right) {
                return Ok(LiteralType::Bool);
            } else {
                err.insert_err(ErrorKind::OpWrongType {
                    op: op,
                    typ: LiteralType::Bool,
                });
            }
            return Err(());
        }
        _ => unreachable!(),
    }
}

fn var_dec(
    var: Box<Node>,
    expr_type: Result<LiteralType, ()>,
    context: &mut Context,
    funcs: &mut Funcs,
    err: &mut TypeErrors,
    next: Option<Box<Node>>,
) -> Result<LiteralType, ()> {
    let expr_type = match expr_type {
        Ok(typ) => Some(typ),
        Err(_) => None,
    };

    match expr_type {
        Some(expr_type) => match *var {
            Node::VarBinding(var, var_type, mutable) => {
                let name = match *var {
                    Node::Var(name) => name,
                    _ => unreachable!(),
                };
                // Add the variable to the context so it can be used to type check if used in other expr
                context.insert_var(name.clone(), mutable, var_type, Value::None);

                // Handle mismatched types
                let mut ret = Ok(var_type);
                if expr_type != var_type {
                    err.insert_err(ErrorKind::MismatchedTypesVar {
                        var: name,
                        expected: var_type,
                        found: expr_type,
                    });
                    ret = Err(());
                }

                match next {
                    Some(next) => return visit(next, context, funcs, err),
                    None => ret,
                }
            }
            _ => unreachable!(),
        },
        None => match next {
            Some(next) => return visit(next, context, funcs, err),
            None => Err(()),
        },
    }
}

// --------------------------- TESTS ---------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::program_parser::parse;

    #[test]
    fn op_type_num_expr_bool() {
        let input = parse(
            "fn main() {
				let a: i32 = 1 + true;	
			}"
            .to_string(),
        )
        .unwrap();

        let mut errors = TypeErrors::new();
        errors.insert_err(ErrorKind::OpWrongType {
            op: Opcode::Add,
            typ: LiteralType::Bool,
        });
        assert_eq!(type_check(input).unwrap_err(), errors);
    }

    #[test]
    fn op_type_num_expr_num() {
        let input = parse(
            "fn main() {
				let a: i32 = 1 + 2;	
			}"
            .to_string(),
        )
        .unwrap();

        assert!(type_check(input).is_ok());
    }

    #[test]
    fn op_type_rel_expr_num() {
        let input = parse(
            "fn main() {
				let a: bool = 1 > 2;	
			}"
            .to_string(),
        )
        .unwrap();

        assert!(type_check(input).is_ok());
    }

    #[test]
    fn op_type_rel_expr_bool() {
        let input = parse(
            "fn main() {
				let a: bool = 1 > false;	
			}"
            .to_string(),
        )
        .unwrap();

        let mut errors = TypeErrors::new();
        errors.insert_err(ErrorKind::OpWrongType {
            op: Opcode::GT,
            typ: LiteralType::Bool,
        });
        assert_eq!(type_check(input).unwrap_err(), errors);
    }

    #[test]
    fn op_type_log_expr_mix() {
        let input = parse(
            "fn main() {
				let a: bool = 1 == false;	
			}"
            .to_string(),
        )
        .unwrap();

        let mut errors = TypeErrors::new();
        errors.insert_err(ErrorKind::MismatchedTypesOp {
            op: Opcode::EQ,
            expected: LiteralType::I32,
			found: LiteralType::Bool
        });
        assert_eq!(type_check(input).unwrap_err(), errors);

		let input = parse(
            "fn main() {
				let a: bool = false != 1;	
			}"
            .to_string(),
        )
        .unwrap();

        let mut errors = TypeErrors::new();
        errors.insert_err(ErrorKind::MismatchedTypesOp {
            op: Opcode::NEQ,
            expected: LiteralType::Bool,
			found: LiteralType::I32
        });
        assert_eq!(type_check(input).unwrap_err(), errors);
    }

    #[test]
    fn let_i32() {
        let input = parse(
            "fn main() {
				let a: i32 = 1 + 2;	
			}"
            .to_string(),
        )
        .unwrap();

        assert!(type_check(input).is_ok());
    }

	#[test]
    fn let_bool() {
        let input = parse(
            "fn main() {
				let a: bool = true || false;	
			}"
            .to_string(),
        )
        .unwrap();

        assert!(type_check(input).is_ok());
    }

    #[test]
    fn mm_let_i32() {
        let input = parse(
            "fn main() {
				let a: i32 = 1 > 5;	
			}"
            .to_string(),
        )
        .unwrap();

        let mut errors = TypeErrors::new();
        errors.insert_err(ErrorKind::MismatchedTypesVar {
            var: String::from("a"),
            expected: LiteralType::I32,
			found: LiteralType::Bool
        });
        assert_eq!(type_check(input).unwrap_err(), errors);
    }

    #[test]
    fn mm_let_bool() {
        let input = parse(
            "fn main() {
				let a: bool = 1 + 5;	
			}"
            .to_string(),
        )
        .unwrap();

        let mut errors = TypeErrors::new();
        errors.insert_err(ErrorKind::MismatchedTypesVar {
            var: String::from("a"),
            expected: LiteralType::Bool,
			found: LiteralType::I32
        });
        assert_eq!(type_check(input).unwrap_err(), errors);
    }
}
