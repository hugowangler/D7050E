use std::collections::HashMap;

use crate::{
    ast::Node,
    context::Context,
    function::Func,
    operators::{BinOpcode, LogOpcode, RelOpcode},
    scope::Scope,
    types::LiteralType,
    value::Value,
};

type Funcs = HashMap<String, Func>; // Stores all the function names and the their structs

pub fn interp(mut funcs_ast: Vec<Box<Node>>) -> Option<Value> {
    let mut context = Context::new();
    let mut funcs: Funcs = HashMap::new();

    // Declare all of the functions of the parsed program
    for func in funcs_ast.drain(..) {
        visit(func, &mut context, &mut funcs);
    }

    match funcs.get("main") {
        Some(main) => main.clone().call(vec![], &mut context, &mut funcs),
        None => panic!("No main function is defined in the program"),
    }
}

pub fn visit(node: Box<Node>, context: &mut Context, funcs: &mut Funcs) -> Value {
    match *node {
        Node::Number(num) => Value::Number(num),
        Node::Bool(b) => Value::Bool(b),
        Node::_String(text) => Value::String(text),
        Node::Var(name) => eval_var(&name, context),
        Node::VarValue { var, expr, next } => {
            update_var(var, visit(expr, context, funcs), context, funcs, next)
        }
        Node::BinOp(left, op, right) => eval_bin_op(
            visit(left, context, funcs),
            op,
            visit(right, context, funcs),
        ),
        Node::RelOp(left, op, right) => eval_rel_op(
            visit(left, context, funcs),
            op,
            visit(right, context, funcs),
        ),
        Node::LogOp(left, op, right) => eval_log_op(
            visit(left, context, funcs),
            op,
            visit(right, context, funcs),
        ),
        Node::Let { var, expr, next } => {
            assign_var(var, visit(expr, context, funcs), context, funcs, next)
        }
        Node::If {
            cond,
            statement,
            next,
        } => eval_if_statement(visit(cond, context, funcs), statement, context, funcs, next),
        Node::IfElse {
            cond,
            if_statement,
            else_statement,
            next,
        } => eval_if_else_statement(
            visit(cond, context, funcs),
            if_statement,
            else_statement,
            context,
            funcs,
            next,
        ),
        Node::While {
            cond,
            statement,
            next,
        } => eval_while_statement(cond, statement, context, funcs, next),
        Node::Func {
            name,
            params,
            r_type,
            body,
        } => eval_func_dec(&name, &params, r_type, &body, funcs),
        Node::FuncCall { name, args, next } => eval_func_call(&name, args, context, funcs, next),
        Node::Return(expr) => visit(expr, context, funcs),
        // Print node used for easier debugging
        Node::Print { expr, next } => {
            let var_name = match *expr.clone() {
                Node::Var(name) => Some(name),
                _ => None,
            };

            match var_name {
                Some(name) => println!("{:#?} = {:#?}", name, visit(expr, context, funcs)),
                None => println!("{:#?}", visit(expr, context, funcs)),
            }

            match next {
                Some(statement) => visit(statement, context, funcs),
                None => Value::None,
            }
        }
        _ => panic!("Node not supported: {:?}", *node),
    }
}

fn eval_func_dec(
    name: &str,
    params: &Vec<Box<Node>>,
    r_type: Option<LiteralType>,
    body: &Box<Node>,
    funcs: &mut Funcs,
) -> Value {
    let func = Func::new(name.to_string(), params.clone(), r_type, body.clone());

    match funcs.insert(name.to_string(), func) {
        Some(_) => panic!("Function: {} is already defined", name.clone()),
        None => Value::None,
    }
}

fn eval_func_call(
    name: &str,
    args: Vec<Box<Node>>,
    context: &mut Context,
    funcs: &mut Funcs,
    next: Option<Box<Node>>,
) -> Value {
    let func_res = match funcs.get(name) {
        Some(func) => match func.clone().call(args, context, funcs) {
            Some(ret) => ret,
            None => Value::None,
        },
        None => panic!("eval_func_call: Function \"{}\" is not defined", name),
    };
    context.pop();
    match next {
        Some(statement) => visit(statement, context, funcs),
        None => func_res,
    }
}

fn assign_var(
    var: Box<Node>,
    expr: Value,
    context: &mut Context,
    funcs: &mut Funcs,
    next: Option<Box<Node>>,
) -> Value {
    match *var {
        Node::VarBinding(var, _var_type) => def_var(var, expr, context),
        _ => panic!("assign_var: No VarBinding node"),
    }

    match next {
        Some(statement) => visit(statement, context, funcs),
        None => Value::None,
    }
}

fn def_var(var: Box<Node>, expr: Value, context: &mut Context) {
    match *var {
        Node::Var(name) => context.insert_var(name, expr),
        _ => panic!("def_var: No var node"),
    }
}

fn update_var(
    var: Box<Node>,
    expr: Value,
    context: &mut Context,
    funcs: &mut Funcs,
    next: Option<Box<Node>>,
) -> Value {
    match *var {
        Node::Var(name) => {
            match context.update_var(name.clone(), expr) {
                None => panic!("update_var: Variable \"{}\" is not defined", name),
                Some(_) => (),
            };
        }
        _ => panic!("update_var: Node not Var type"),
    }

    match next {
        Some(statement) => visit(statement, context, funcs),
        None => Value::None,
    }
}

fn eval_var(name: &str, context: &mut Context) -> Value {
    match context.get_var(name) {
        Some(value) => value,
        None => panic!("eval_var: Variable \"{}\" is not defined", name),
    }
}

// TODO: add floats to grammar and handle them here
fn eval_bin_op(left: Value, op: BinOpcode, right: Value) -> Value {
    let l = match left {
        Value::Number(num) => num,
        _ => panic!("eval_bin_op LEFT no number"),
    };

    let r = match right {
        Value::Number(num) => num,
        _ => panic!("eval_bin_op RIGHT no number"),
    };

    match op {
        BinOpcode::Add => Value::Number(l + r),
        BinOpcode::Sub => Value::Number(l - r),
        BinOpcode::Div => Value::Number(l / r),
        BinOpcode::Mul => Value::Number(l * r),
    }
}

fn eval_rel_op(left: Value, op: RelOpcode, right: Value) -> Value {
    match (left, right) {
        (Value::Number(l_num), Value::Number(r_num)) => eval_num_rel_op(l_num, op, r_num),
        (Value::Bool(l_bool), Value::Bool(r_bool)) => eval_bool_rel_op(l_bool, op, r_bool),
        _ => panic!("eval_rel_op left and right not same type"),
    }
}

fn eval_num_rel_op(left: i32, op: RelOpcode, right: i32) -> Value {
    match op {
        RelOpcode::EQ => Value::Bool(left == right),
        RelOpcode::NEQ => Value::Bool(left != right),
        RelOpcode::GT => Value::Bool(left > right),
        RelOpcode::LT => Value::Bool(left < right),
        RelOpcode::GEQ => Value::Bool(left >= right),
        RelOpcode::LEQ => Value::Bool(left <= right),
    }
}

fn eval_bool_rel_op(left: bool, op: RelOpcode, right: bool) -> Value {
    match op {
        RelOpcode::EQ => Value::Bool(left == right),
        RelOpcode::NEQ => Value::Bool(left != right),
        _ => panic!("eval_bool_rel_op OPERATION not valid for booleans"),
    }
}

fn eval_log_op(left: Value, op: LogOpcode, right: Value) -> Value {
    let (l, r) = match (left, right) {
        (Value::Bool(l_bool), Value::Bool(r_bool)) => (l_bool, r_bool),
        _ => panic!("eval_log_op LEFT and RIGHT not both booleans"),
    };

    match op {
        LogOpcode::AND => Value::Bool(l && r),
        LogOpcode::OR => Value::Bool(l || r),
    }
}

fn eval_if_statement(
    cond: Value,
    statement: Box<Node>,
    context: &mut Context,
    funcs: &mut Funcs,
    next: Option<Box<Node>>,
) -> Value {
    context.push(Scope::new());
    let stmnt_res = match cond.clone() {
        Value::Bool(b) => {
            if b {
                visit(statement, context, funcs)
            } else {
                Value::None
            }
        }
        _ => panic!("eval_if_statement CONDITION did not evaluate to a boolean"),
    };
    context.pop();

    // If the if-statement returns anything, return that. Otherwise do the next statement
    match stmnt_res {
        Value::None => match next {
            Some(stmnt) => visit(stmnt, context, funcs),
            None => Value::None,
        },
        _ => stmnt_res,
    }
}

fn eval_if_else_statement(
    cond: Value,
    if_s: Box<Node>,
    else_s: Box<Node>,
    context: &mut Context,
    funcs: &mut Funcs,
    next: Option<Box<Node>>,
) -> Value {
    context.push(Scope::new());
    let stmnt_res = match cond.clone() {
        Value::Bool(b) => {
            if b {
                visit(if_s, context, funcs)
            } else {
                visit(else_s, context, funcs)
            }
        }
        _ => panic!("eval_if_else_statement CONDITION does not evaluate to a boolean"),
    };
    context.pop();

    match stmnt_res {
        Value::None => match next {
            Some(stmnt) => visit(stmnt, context, funcs),
            None => Value::None,
        },
        _ => stmnt_res,
    }
}

fn eval_while_statement(
    cond: Box<Node>,
    statement: Box<Node>,
    context: &mut Context,
    funcs: &mut Funcs,
    next: Option<Box<Node>>,
) -> Value {
    context.push(Scope::new());
    match visit(cond.clone(), context, funcs) {
        Value::Bool(b) => {
            if b {
                let do_stmnt = visit(statement.clone(), context, funcs);
                // Return the result of the while statement if it returns anything, otherwise continue the loop
                match do_stmnt {
                    Value::None => {
                        context.pop();
                        eval_while_statement(cond, statement.clone(), context, funcs, next.clone())
                    }
                    _ => {
                        context.pop();
                        do_stmnt
                    }
                }
            } else {
                match next {
                    Some(stmnt) => {
                        context.pop();
                        visit(stmnt, context, funcs)
                    }
                    None => {
                        context.pop();
                        Value::None
                    }
                }
            }
        }
        _ => panic!("eval_while_statement CONDITION does not evalute to a boolean"),
    }
}
