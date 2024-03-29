use std::collections::HashMap;

use crate::{
    ast::Node, context::Context, function::Func, operators::Opcode, scope::Scope,
    types::LiteralType, value::Value,
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
        Node::UnaryOp(op, value) => eval_unary(op, visit(value, context, funcs)),
        Node::Bool(b) => Value::Bool(b),
        Node::_String(text) => Value::String(text),
        Node::Var(name) => eval_var(&name, context),
        Node::VarValue { var, expr, next } => {
            update_var(var, visit(expr, context, funcs), context, funcs, next)
        }
        Node::Expr(left, op, right) => eval_expr(
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
        Node::Return { expr, .. } => visit(expr, context, funcs),
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

// TODO: Allow for only def. a variable and not having to assign value
fn assign_var(
    var: Box<Node>,
    expr: Value,
    context: &mut Context,
    funcs: &mut Funcs,
    next: Option<Box<Node>>,
) -> Value {
    match *var {
        Node::VarBinding(var, typ, mutable) => def_var(var, mutable, typ, expr, context),
        _ => panic!("assign_var: No VarBinding node"),
    }

    match next {
        Some(statement) => visit(statement, context, funcs),
        None => Value::None,
    }
}

fn def_var(var: Box<Node>, mutable: bool, typ: LiteralType, expr: Value, context: &mut Context) {
    match *var {
        Node::Var(name) => context.insert_var(name, mutable, typ, expr),
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
                None => panic!(
                    "Variable \"{}\" is not defined, try: Let {}: <type> = <expr>;",
                    name, name
                ),
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
    match context.get_var_value(name) {
        Some(value) => value,
        None => panic!("Variable \"{}\" is not defined", name),
    }
}

fn eval_unary(op: Opcode, val: Value) -> Value {
    let n = match val {
        Value::Number(n) => n,
        _ => panic!("UnaryOp on value not number!"),
    };

    match op {
        Opcode::Sub => Value::Number(-n),
        _ => unreachable!(),
    }
}

fn eval_expr(left: Value, op: Opcode, right: Value) -> Value {
    match op {
        Opcode::Add | Opcode::Sub | Opcode::Div | Opcode::Mul => eval_num_expr(left, op, right),
        Opcode::AND | Opcode::OR => eval_log_op(left, op, right),
        Opcode::EQ | Opcode::NEQ | Opcode::GT | Opcode::LT | Opcode::LEQ | Opcode::GEQ => {
            eval_rel_op(left, op, right)
        }
    }
}

/// Evaluates an expression which will result in a number (Value::Number enum)
fn eval_num_expr(left: Value, op: Opcode, right: Value) -> Value {
    let (l, r) = match (left.clone(), right.clone()) {
        (Value::Number(l_num), Value::Number(r_num)) => (l_num, r_num),
        _ => panic!(
            "Left or right part of number expression not a number, 
			found: left = {:#?} and right = {:#?}",
            left, right
        ),
    };

    match op {
        Opcode::Add => Value::Number(l + r),
        Opcode::Sub => Value::Number(l - r),
        Opcode::Div => Value::Number(l / r),
        Opcode::Mul => Value::Number(l * r),
        _ => panic!(
            "Wrong operation for evaluating a expression resulting in a number, found: {}",
            op.to_string()
        ),
    }
}

fn eval_rel_op(left: Value, op: Opcode, right: Value) -> Value {
    match (left, right) {
        (Value::Number(l_num), Value::Number(r_num)) => eval_num_rel_op(l_num, op, r_num),
        (Value::Bool(l_bool), Value::Bool(r_bool)) => eval_bool_rel_op(l_bool, op, r_bool),
        _ => panic!("eval_rel_op left and right not same type"),
    }
}

fn eval_num_rel_op(left: i32, op: Opcode, right: i32) -> Value {
    match op {
        Opcode::EQ => Value::Bool(left == right),
        Opcode::NEQ => Value::Bool(left != right),
        Opcode::GT => Value::Bool(left > right),
        Opcode::LT => Value::Bool(left < right),
        Opcode::GEQ => Value::Bool(left >= right),
        Opcode::LEQ => Value::Bool(left <= right),
        _ => panic!(
            "Wrong operation for evaluating a relational expression, found: {}",
            op.to_string()
        ),
    }
}

fn eval_bool_rel_op(left: bool, op: Opcode, right: bool) -> Value {
    match op {
        Opcode::EQ => Value::Bool(left == right),
        Opcode::NEQ => Value::Bool(left != right),
        _ => panic!(
            "Wrong operation for evaluating a relational expression, found: {}",
            op.to_string()
        ),
    }
}

fn eval_log_op(left: Value, op: Opcode, right: Value) -> Value {
    let (l, r) = match (left, right) {
        (Value::Bool(l_bool), Value::Bool(r_bool)) => (l_bool, r_bool),
        _ => panic!("eval_log_op LEFT and RIGHT not both booleans"),
    };

    match op {
        Opcode::AND => Value::Bool(l && r),
        Opcode::OR => Value::Bool(l || r),
        _ => panic!(
            "Wrong operation for evaluating a logical expression, found: {}",
            op.to_string()
        ),
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
                // Return the result of the while statement if it returns anything,
                // otherwise continue the loop
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

// --------------------------------- TESTS ---------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::program_parser::parse;
    use std::{error::Error, fs::File, io::prelude::*, path::Path};

    fn parse_interp(path: &Path) -> Option<Value> {
        let display = path.display();
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => panic!("Could not open {}: {}", display, e.description()),
        };

        let mut input = String::new();
        match file.read_to_string(&mut input) {
            Ok(_) => (),
            Err(e) => panic!("Could not read file: {:?}", e),
        }

        interp(parse(input).unwrap())
    }

    // Expression precedence
    #[test]
    fn add_sub_mul_prec() {
        assert_eq!(
            parse_interp(Path::new("tests/precedence/add_sub.txt")),
            Some(Value::Number(9))
        );
        assert_eq!(
            parse_interp(Path::new("tests/precedence/add_sub_mul.txt")),
            Some(Value::Number(27))
        );
    }

    #[test]
    fn paran_prec() {
        assert_eq!(
            parse_interp(Path::new("tests/precedence/paran_mul.txt")),
            Some(Value::Number(24))
        );
        assert_eq!(
            parse_interp(Path::new("tests/precedence/paran_mul_rev.txt")),
            Some(Value::Number(24))
        );
        assert_eq!(
            parse_interp(Path::new("tests/precedence/paran_add.txt")),
            Some(Value::Number(25))
        );
        assert_eq!(
            parse_interp(Path::new("tests/precedence/paran_add_rev.txt")),
            Some(Value::Number(25))
        );
    }

    // Scopes
    #[test]
    fn if_update_var_in_new_scope() {
        let res = parse_interp(Path::new("tests/scope/if_update_var_new_scope.txt"));
        assert_eq!(res, Some(Value::Number(1)))
    }

    #[test]
    fn if_var_in_scope() {
        let res = parse_interp(Path::new("tests/scope/if_var_in_scope.txt"));
        assert_eq!(res, Some(Value::Number(0)))
    }

    #[test]
    fn if_var_in_scope_return() {
        let res = parse_interp(Path::new("tests/scope/if_var_in_scope_return.txt"));
        assert_eq!(res, Some(Value::Number(1)))
    }

    #[test]
    #[should_panic]
    fn if_var_outside_scope() {
        parse_interp(Path::new("tests/scope/if_var_outside_scope.txt"));
    }

    #[test]
    #[should_panic]
    fn fn_scope_not_same() {
        parse_interp(Path::new("tests/scope/fn_scope_not_same.txt"));
    }

    #[test]
    fn while_update_var_new_scope() {
        assert_eq!(
            parse_interp(Path::new("tests/scope/while_update_var_new_scope.txt")),
            Some(Value::Number(5))
        )
    }

    #[test]
    fn while_var_in_scope_return() {
        assert_eq!(
            parse_interp(Path::new("tests/scope/while_var_in_scope_return.txt")),
            Some(Value::Number(1))
        )
    }

    #[test]
    fn while_var_in_scope() {
        assert_eq!(
            parse_interp(Path::new("tests/scope/while_var_in_scope.txt")),
            Some(Value::Number(0))
        )
    }

    #[test]
    #[should_panic]
    fn while_var_outside_scope() {
        parse_interp(Path::new("tests/scope/while_var_outside_scope.txt"));
    }

    #[test]
    fn if_else_update_var_in_scope() {
        assert_eq!(
            parse_interp(Path::new("tests/scope/if_else_update_var_new_scope.txt")),
            Some(Value::Number(10))
        )
    }

    #[test]
    fn if_else_var_in_scope_return() {
        assert_eq!(
            parse_interp(Path::new("tests/scope/if_else_var_in_scope_return.txt")),
            Some(Value::Number(20))
        )
    }

    #[test]
    fn if_else_var_in_scope() {
        assert_eq!(
            parse_interp(Path::new("tests/scope/if_else_var_in_scope.txt")),
            Some(Value::Number(1))
        )
    }

    #[test]
    #[should_panic]
    fn if_else_var_outside_scope() {
        parse_interp(Path::new("tests/scope/if_else_var_outside_scope.txt"));
    }

    // Functions
    #[test]
    fn fn_sum() {
        let res = parse_interp(Path::new("tests/function/sum.txt"));
        assert_eq!(res, Some(Value::Number(15)))
    }

    #[test]
    fn fn_fibonacci() {
        let res = parse_interp(Path::new("tests/function/fibonacci.txt"));
        assert_eq!(res, Some(Value::Number(6765)))
    }

    // Mutability
    #[test]
    #[should_panic]
    fn no_mut_var() {
        parse_interp(Path::new("tests/mutability/no_mut_var.txt"));
    }

    #[test]
    fn mut_var() {
        assert_eq!(
            parse_interp(Path::new("tests/mutability/mut_var.txt")),
            Some(Value::Number(100))
        );
    }

    #[test]
    #[should_panic]
    fn fn_no_mut_param() {
        parse_interp(Path::new("tests/mutability/fn_no_mut_param.txt"));
    }

    #[test]
    fn fn_mut_param() {
        assert_eq!(
            parse_interp(Path::new("tests/mutability/fn_mut_param.txt")),
            Some(Value::Number(51))
        );
    }
}
