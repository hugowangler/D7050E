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
) -> Option<LiteralType> {
    match *node {
        Node::Number(_) => Some(LiteralType::I32),
        Node::Bool(_) => Some(LiteralType::Bool),
        Node::_String(_) => Some(LiteralType::_String),
        Node::UnaryOp(_, _) => Some(LiteralType::I32),
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
    left: Option<LiteralType>,
    op: Opcode,
    right: Option<LiteralType>,
    err: &mut TypeErrors,
) -> Option<LiteralType> {
    match op {
        Opcode::Add | Opcode::Sub | Opcode::Div | Opcode::Mul => num_expr(left, op, right, err),
        Opcode::AND | Opcode::OR => log_op(left, right, err),
        Opcode::EQ | Opcode::NEQ | Opcode::GT | Opcode::LT | Opcode::LEQ | Opcode::GEQ => {
            rel_op(left, right, err)
        }
    }
}

fn num_expr(
    left: Option<LiteralType>,
    op: Opcode,
    right: Option<LiteralType>,
    err: &mut TypeErrors,
) -> Option<LiteralType> {
    if let (Some(LiteralType::I32), Some(LiteralType::I32)) = (left, right) {
        return Some(LiteralType::I32);
    } else {
        err.insert_err(ErrorKind::OpWrongType {
            op: op,
            typ: LiteralType::Bool,
        });
    }
    None
}

fn log_op(
    left: Option<LiteralType>,
    right: Option<LiteralType>,
    err: &mut TypeErrors,
) -> Option<LiteralType> {
    match (left, right) {
        (Some(LiteralType::Bool), Some(LiteralType::Bool)) => Some(LiteralType::Bool),
        _ => panic!("log op"),
    }
}

fn rel_op(
    left: Option<LiteralType>,
    right: Option<LiteralType>,
    err: &mut TypeErrors,
) -> Option<LiteralType> {
    match (left, right) {
        (Some(LiteralType::Bool), Some(LiteralType::Bool)) => Some(LiteralType::Bool),
        _ => panic!("log op"),
    }
}

fn var_dec(
    var: Box<Node>,
    expr_type: Option<LiteralType>,
    context: &mut Context,
    funcs: &mut Funcs,
    err: &mut TypeErrors,
    next: Option<Box<Node>>,
) -> Option<LiteralType> {
    match *var {
        Node::VarBinding(var, typ, mutable) => {
            // Add the variable to the context so it can be used to evaluate typ if used in other expr
            match *var {
                Node::Var(name) => context.insert_var(name, mutable, typ, Value::None),
                _ => unreachable!(),
            }
            Some(typ)
        }
        _ => unreachable!(),
    }
}
