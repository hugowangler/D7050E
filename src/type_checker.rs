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

#[allow(unused_must_use)]
pub fn type_check(mut funcs_ast: Vec<Box<Node>>) -> Result<(), TypeErrors> {
    let mut funcs: Funcs = HashMap::new();
    let mut context: Context = Context::new();
    let mut type_errors: TypeErrors = TypeErrors::new();

    // Declare all of the functions
    for func in funcs_ast.drain(..) {
        func_dec(func, &mut funcs);
    }

    // Type check the function bodies
    for (_, func) in funcs.clone().iter() {
        context.push(Scope::init_param_types(&func.params)); // Push scope containing the params type
        visit(
            func.body.clone(),
            &mut context,
            &funcs,
            &func.name,
            &mut type_errors,
        );
        context.pop();
    }

    if type_errors.len() > 0 {
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

/// Type checks the AST
///
/// Returns a result of either
/// Ok(LiteralType): The type determined
/// Err(Option<LiteralType>): If possible, the type that would have resulted if the
/// 	sub expression was correctly typed, otherwise None
fn visit(
    node: Box<Node>,
    context: &mut Context,
    funcs: &Funcs,
    curr_func: &str,
    err: &mut TypeErrors,
) -> Result<LiteralType, Option<LiteralType>> {
    match *node {
        Node::Number(_) => Ok(LiteralType::I32),
        Node::Bool(_) => Ok(LiteralType::Bool),
        Node::_String(_) => Ok(LiteralType::_String),
        Node::UnaryOp(_, _) => Ok(LiteralType::I32),
        Node::Var(name) => var(&name, context, err),
        Node::VarValue { var, expr, next } => var_update(
            var,
            visit(expr, context, funcs, curr_func, err),
            context,
            funcs,
            curr_func,
            next,
            err,
        ),
        Node::Expr(left, op, right) => expr(
            visit(left, context, funcs, curr_func, err),
            op,
            visit(right, context, funcs, curr_func, err),
            err,
        ),
        Node::Let { var, expr, next } => var_dec(
            var,
            visit(expr, context, funcs, curr_func, err),
            context,
            funcs,
            curr_func,
            err,
            next,
        ),
        Node::FuncCall { name, args, next } => {
            func_call(&name, args, context, funcs, curr_func, err, next)
        }
        Node::Return(expr) => ret(
            visit(expr, context, funcs, curr_func, err),
            funcs,
            curr_func,
            err,
        ),
        _ => unimplemented!(),
    }
}

fn ret(
    val: Result<LiteralType, Option<LiteralType>>,
    funcs: &Funcs,
    curr_func: &str,
    err: &mut TypeErrors,
) -> Result<LiteralType, Option<LiteralType>> {
	let fn_r_type = match funcs.get(curr_func) {
		Some(func) => func.get_r_type(),
		_ => unreachable!()
	};

	let val_res = match val {
		Ok(typ) => Some(typ),
		Err(typ) => match typ {
			Some(typ) => Some(typ),
			None => None
		}
	};

	if let Some(val_type) = val_res {
		if fn_r_type != val_type {
			err.insert_err(ErrorKind::FnReturnMismatch{name: curr_func.to_string(), expected: fn_r_type, found: val_type});
			return Err(Some(fn_r_type))
		}
	}
	Ok(fn_r_type)
}

fn func_call(
    name: &str,
    args: Vec<Box<Node>>,
    context: &mut Context,
    funcs: &Funcs,
    curr_func: &str,
    err: &mut TypeErrors,
    next: Option<Box<Node>>,
) -> Result<LiteralType, Option<LiteralType>> {
    let func = match funcs.get(name) {
        Some(func) => func,
        None => {
            err.insert_err(ErrorKind::FnNotInScope {
                name: name.to_string(),
            });
            match next {
                Some(next) => return visit(next, context, funcs, curr_func, err),
                None => return Err(None),
            }
        }
    };

    let params = func.get_param_types();
    let mut arg_types = vec![];

    for arg in args.iter() {
        arg_types.push(visit(arg.clone(), context, funcs, curr_func, err));
    }

    if params.len() != arg_types.len() {
        err.insert_err(ErrorKind::FnNumParamMismatch {
            name: name.to_string(),
            takes: params.len(),
            supplied: args.len(),
        });
    } else {
        for pair in params.iter().zip(arg_types.iter()) {
            if let ((param_name, param_type), Ok(arg_type)) = pair {
                if param_type != arg_type {
                    err.insert_err(ErrorKind::FnParamTypeMismatch {
                        name: name.to_string(),
                        param: param_name.to_string(),
                        found: *arg_type,
                        expected: *param_type,
                    });
                }
            }
        }
    }

    match next {
        Some(next) => visit(next, context, funcs, curr_func, err),
        None => Ok(func.get_r_type()),
    }
}

fn var_dec(
    var: Box<Node>,
    val: Result<LiteralType, Option<LiteralType>>,
    context: &mut Context,
    funcs: &Funcs,
    curr_func: &str,
    err: &mut TypeErrors,
    next: Option<Box<Node>>,
) -> Result<LiteralType, Option<LiteralType>> {
    let val_res = match val {
        Ok(typ) => Some(typ),
        Err(typ) => match typ {
            Some(typ) => Some(typ),
            None => None,
        },
    };

    let (name, var_type, mutable) = match *var {
        Node::VarBinding(var, var_type, mutable) => match *var {
            Node::Var(name) => (name, var_type, mutable),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    // Push to context so it can be used to type check if used in other expr
    context.insert_var(name.clone(), mutable, var_type, Value::None);

    // Handle mismatched types
    let mut ret = Ok(var_type);
    if let Some(val_type) = val_res {
        if val_type != var_type {
            err.insert_err(ErrorKind::MismatchedTypesVar {
                var: name,
                expected: var_type,
                found: val_type,
            });
            ret = Err(Some(var_type));
        }
    } else {
        ret = Err(None);
    }

    match next {
        Some(next) => return visit(next, context, funcs, curr_func, err),
        None => ret,
    }
}

fn var(
    name: &str,
    context: &mut Context,
    err: &mut TypeErrors,
) -> Result<LiteralType, Option<LiteralType>> {
    match context.get_var(name) {
        Some(var) => Ok(var.get_type()),
        None => {
            err.insert_err(ErrorKind::VarNotInScope {
                var: name.to_string(),
            });
            Err(None)
        }
    }
}

fn var_update(
    var: Box<Node>,
    val: Result<LiteralType, Option<LiteralType>>,
    context: &mut Context,
    funcs: &Funcs,
    curr_func: &str,
    next: Option<Box<Node>>,
    err: &mut TypeErrors,
) -> Result<LiteralType, Option<LiteralType>> {
    let val_res = match val {
        Ok(typ) => Some(typ),
        Err(typ) => match typ {
            Some(typ) => Some(typ),
            None => None,
        },
    };

    // Get variable name and type, if not defined generate error and go to next
    let (var_name, var_type, var_mut) = match *var {
        Node::Var(name) => match context.get_var(name.as_str()) {
            Some(var) => (name, var.get_type(), var.is_mut()),
            None => {
                err.insert_err(ErrorKind::VarNotInScope { var: name });
                match next {
                    Some(next) => return visit(next, context, funcs, curr_func, err),
                    None => return Err(None),
                }
            }
        },
        _ => unreachable!(),
    };

    let mut ret = Ok(var_type);

    // If the new value of the variable has a type (passed type check),
    // check if the variable has the same type as the new value
    if let Some(val_type) = val_res {
        if var_type != val_type {
            err.insert_err(ErrorKind::MismatchedTypesVar {
                var: var_name,
                expected: var_type,
                found: val_type,
            });
            ret = Err(Some(var_type));
        } else {
            if !var_mut {
                err.insert_err(ErrorKind::VarImmut { var: var_name });
            }
        }
    } else {
        // Otherwise return the variables assigned type
        ret = Err(Some(var_type));
    }

    match next {
        Some(next) => visit(next, context, funcs, curr_func, err),
        None => ret,
    }
}

fn expr(
    left: Result<LiteralType, Option<LiteralType>>,
    op: Opcode,
    right: Result<LiteralType, Option<LiteralType>>,
    err: &mut TypeErrors,
) -> Result<LiteralType, Option<LiteralType>> {
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
        match op {
            Opcode::Add | Opcode::Sub | Opcode::Div | Opcode::Mul => Err(Some(LiteralType::I32)),
            _ => Err(Some(LiteralType::Bool)),
        }
    }
}

fn num_expr(
    left: LiteralType,
    op: Opcode,
    right: LiteralType,
    err: &mut TypeErrors,
) -> Result<LiteralType, Option<LiteralType>> {
    if let (LiteralType::I32, LiteralType::I32) = (left, right) {
        return Ok(LiteralType::I32);
    } else {
        err.insert_err(ErrorKind::OpWrongType {
            op: op,
            typ: LiteralType::Bool,
        });
    }
    Err(Some(LiteralType::I32))
}

fn log_op(
    left: LiteralType,
    op: Opcode,
    right: LiteralType,
    err: &mut TypeErrors,
) -> Result<LiteralType, Option<LiteralType>> {
    if let (LiteralType::Bool, LiteralType::Bool) = (left, right) {
        return Ok(LiteralType::Bool);
    } else {
        err.insert_err(ErrorKind::OpWrongType {
            op: op,
            typ: LiteralType::I32,
        });
    }
    Err(Some(LiteralType::Bool))
}

fn rel_op(
    left: LiteralType,
    op: Opcode,
    right: LiteralType,
    err: &mut TypeErrors,
) -> Result<LiteralType, Option<LiteralType>> {
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
                    expected: left,
                });
            }
            return Err(Some(LiteralType::Bool));
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
            return Err(Some(LiteralType::Bool));
        }
        _ => unreachable!(),
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
            found: LiteralType::Bool,
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
            found: LiteralType::I32,
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
            found: LiteralType::Bool,
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
            found: LiteralType::I32,
        });
        assert_eq!(type_check(input).unwrap_err(), errors);
    }

    #[test]
    fn var_not_in_scope() {
        let input = parse(
            "fn main() {
				let mut a: bool = true;
				let b: bool = a && c;
			}"
            .to_string(),
        )
        .unwrap();

        let mut errors = TypeErrors::new();
        errors.insert_err(ErrorKind::VarNotInScope {
            var: String::from("c"),
        });
        assert_eq!(type_check(input).unwrap_err(), errors);
    }

    #[test]
    fn var_in_scope() {
        let input = parse(
            "fn main() {
				let mut a: bool = true;
				let mut c: bool = false;
				let b: bool = a && c;
			}"
            .to_string(),
        )
        .unwrap();

        assert!(type_check(input).is_ok());
    }

    #[test]
    fn var_let_mm() {
        let input = parse(
            "fn main() {
				let mut a: bool = true;
				let mut b: bool = false;
				let c: i32 = a && b;
			}"
            .to_string(),
        )
        .unwrap();

        let mut errors = TypeErrors::new();
        errors.insert_err(ErrorKind::MismatchedTypesVar {
            var: String::from("c"),
            expected: LiteralType::I32,
            found: LiteralType::Bool,
        });
        assert_eq!(type_check(input).unwrap_err(), errors);
    }

    #[test]
    fn var_update() {
        let input = parse(
            "fn main() {
				let mut a: bool = true;
				let b: i32 = 0;
				let mut c: i32 = false;
				b = a && c;
			}"
            .to_string(),
        )
        .unwrap();

        let mut errors = TypeErrors::new();
        errors.insert_err(ErrorKind::MismatchedTypesVar {
            var: String::from("c"),
            expected: LiteralType::I32,
            found: LiteralType::Bool,
        });
        errors.insert_err(ErrorKind::OpWrongType {
            op: Opcode::AND,
            typ: LiteralType::I32,
        });
        errors.insert_err(ErrorKind::MismatchedTypesVar {
            var: String::from("b"),
            expected: LiteralType::I32,
            found: LiteralType::Bool,
        });
        assert_eq!(type_check(input).unwrap_err(), errors);

        let input = parse(
            "fn main() {
				let mut a: bool = true;
				let b: i32 = 0;
				let mut c: i32 = false;
				let b: i32 = a && c;
			}"
            .to_string(),
        )
        .unwrap();

        let mut errors = TypeErrors::new();
        errors.insert_err(ErrorKind::MismatchedTypesVar {
            var: String::from("c"),
            expected: LiteralType::I32,
            found: LiteralType::Bool,
        });
        errors.insert_err(ErrorKind::OpWrongType {
            op: Opcode::AND,
            typ: LiteralType::I32,
        });
        errors.insert_err(ErrorKind::MismatchedTypesVar {
            var: String::from("b"),
            expected: LiteralType::I32,
            found: LiteralType::Bool,
        });
        assert_eq!(type_check(input).unwrap_err(), errors);

        let input = parse(
            "fn main() {
				let mut a: bool = true;
				let b: bool = false;
				let mut c: bool = false;
				b = a && c;
			}"
            .to_string(),
        )
        .unwrap();

        let mut errors = TypeErrors::new();
        errors.insert_err(ErrorKind::VarImmut {
            var: String::from("b"),
        });
        assert_eq!(type_check(input).unwrap_err(), errors);
    }

    #[test]
    fn fn_params() {
        let input = parse(
            "fn main() {
				let a: i32 = 5;
				let b: bool = false;
				test(a);
				let c: i32 = 4;
			}
			
			fn test(a: i32, b: i32) {
				let c: i32 = 5;	
			}"
            .to_string(),
        )
        .unwrap();

        let mut errors = TypeErrors::new();
        errors.insert_err(ErrorKind::FnNumParamMismatch {
            name: String::from("test"),
            takes: 2,
            supplied: 1,
        });
        assert_eq!(type_check(input).unwrap_err(), errors);

        let input = parse(
            "fn main() {
				let a: i32 = 5;
				let b: bool = false;
				test(a, b);
				let c: i32 = 4;
			}
			
			fn test(a: i32, b: i32) {
				let c: i32 = 5;	
			}"
            .to_string(),
        )
        .unwrap();

        let mut errors = TypeErrors::new();
        errors.insert_err(ErrorKind::FnParamTypeMismatch {
            name: String::from("test"),
            param: String::from("b"),
            expected: LiteralType::I32,
            found: LiteralType::Bool,
        });
        assert_eq!(type_check(input).unwrap_err(), errors);
    }
}
