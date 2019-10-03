pub mod interpreter {
    use std::collections::HashMap;
	
    use crate::ast::Node;    
    use crate::operators::{BinOpcode, RelOpcode, LogOpcode};
	use crate::types::LiteralType;
	use crate::value::Value;
	use crate::scope::Scope;
	use crate::context::Context;
	use crate::function::Func;

	type Funcs = HashMap<String, Func>;	// Stores all the function names and their bodies

    pub fn interp(mut ast: Vec<Box<Node>>) {
        let mut context = Context::new();
		let mut funcs: Funcs = HashMap::new();
        
		for node in ast.drain(..) {
            visit(node, &mut context, &mut funcs);
        }
		println!("context = {:#?}", context);
    }

    fn visit(node: Box<Node>, context: &mut Context, funcs: &mut Funcs) -> Value {
        match *node {
            Node::Number(num) => Value::Number(num),
            Node::Bool(b) => Value::Bool(b),
			Node::_String(text) => Value::String(text),
            Node::Var(name) => eval_var(name, context),
			Node::VarValue(var, expr) => update_var(var, visit(expr, context, funcs), context),
            Node::BinOp(left, op, right) => eval_bin_op(visit(left, context, funcs), op, visit(right, context, funcs)),
            Node::RelOp(left, op, right) => eval_rel_op(visit(left, context, funcs), op, visit(right, context, funcs)),
            Node::LogOp(left, op, right) => eval_log_op(visit(left, context, funcs), op, visit(right, context, funcs)),
            Node::Let(var, expr) => assign_var(var, visit(expr, context, funcs), context),
            Node::Statement(left, right) => {    // Statement is parent node that has children containing statements
				context.push(Scope::new());
                visit(left, context, funcs);
                visit(right, context, funcs);
				context.pop();
                Value::None     // Garbage enum just to not get rust error of no return value
            },
            Node::If{cond, statement} => eval_if_statement(visit(cond, context, funcs), statement, context, funcs),
            Node::IfElse{cond, if_statement, else_statement} => eval_if_else_statement(visit(cond, context, funcs), if_statement, else_statement, context, funcs),
            Node::While{cond, statement} => eval_while_statement(cond, statement, context, funcs),
			Node::Func{name, params, r_type, body} => eval_func_dec(name, params, r_type, body, context, funcs),
			Node::FuncCall{name, args} => eval_func_call(name, args, context, funcs),
            Node::Print(text) => {
                println!("{:#?}", visit(text, context, funcs));
                Value::None
            }
            _ => panic!("Node not supported: {:?}", *node)
        }
    }

	fn eval_func_dec(name: String, params: Vec<Box<Node>>, r_type: LiteralType, body: Box<Node>, context: &mut Context, funcs: &mut Funcs) -> Value {
		let func = Func::new(name.clone(), params, r_type, body);

		match funcs.insert(name.clone(), func) {
			Some(_) => panic!("Function: {} is already defined", name.clone()),
			None => Value::None
		}
	}

	fn eval_func_call(name: String, args: Vec<Box<Node>>, context: &mut Context, funcs: &mut Funcs) -> Value {
		let callee = match funcs.get(&name) {
			Some(func) => func,
			None => panic!("Function: {} is not defined", name)
		};

		if callee.params.len() == args.len() {
			let mut evaled_args: Vec<Box<Node>> = Vec::new();
			// for arg in args.iter() {
			// 	evaled_args.push(visit(arg.clone(), context, funcs));
			// }

			visit(callee.body.clone(), context, funcs)
		} else {
			panic!("Invalid number of arguments when calling {}", name)
		}
	}

    fn assign_var(var: Box<Node>, expr: Value, context: &mut Context) -> Value {
        match *var {
            Node::VarBinding(var, _var_type) => def_var(var, expr, context),
            _ => panic!("assign_var: No VarBinding node")
        };
        Value::None
    }

	fn def_var(var: Box<Node>, expr: Value, context: &mut Context) {
		match *var {
			Node::Var(name) => {
				context.insert_var(name, expr)
			},
			_ => panic!("def_var: No var node")
		}
	}

	fn update_var(var: Box<Node>, expr: Value, context: &mut Context) -> Value {
		match *var {
			Node::Var(name) => {
				match context.update_var(name.clone(), expr) {
					None => panic!("update_var: Variable \"{}\" is not defined", name),
					Some(_) => Value::None
				}
			},
			_=> panic!("update_var: Node not Var type")
		}
	}

    fn eval_var(name: String, context: &mut Context) -> Value {
        match context.get_var(name.clone()) {
			Some(value) => value,
			None => panic!("eval_var: Variable \"{}\" is not defined", name)
		}
    }

    // TODO: add floats to grammar and handle them here
    fn eval_bin_op(left: Value, op: BinOpcode, right: Value) -> Value {
        let l = match left {
            Value::Number(num) => num,
            _ => panic!("eval_bin_op LEFT no number")
        };

        let r = match right {
            Value::Number(num) => num,
            _ => panic!("eval_bin_op RIGHT no number")
        };

        match op {
            BinOpcode::Add => Value::Number(l + r),
            BinOpcode::Sub => Value::Number(l - r),
            BinOpcode::Div => Value::Number(l / r),
            BinOpcode::Mul => Value::Number(l * r)
        }
    }

    fn eval_rel_op(left: Value, op: RelOpcode, right: Value) -> Value {
        match (left, right) {
            (Value::Number(l_num), Value::Number(r_num)) => eval_num_rel_op(l_num, op, r_num),
            (Value::Bool(l_bool), Value::Bool(r_bool)) => eval_bool_rel_op(l_bool, op, r_bool),
            _ => panic!("eval_rel_op left and right not same type")
        }
    }

    fn eval_num_rel_op(left: i32, op: RelOpcode, right: i32) -> Value {
        match op {
            RelOpcode::EQ => Value::Bool(left == right),
            RelOpcode::NEQ => Value::Bool(left != right),
            RelOpcode::GT => Value::Bool(left > right),
            RelOpcode::LT => Value::Bool(left < right),
            RelOpcode::GEQ => Value::Bool(left >= right),
            RelOpcode::LEQ => Value::Bool(left <= right)
        }
    }

    fn eval_bool_rel_op(left: bool, op: RelOpcode, right: bool) -> Value {
        match op {
            RelOpcode::EQ => Value::Bool(left == right),
            RelOpcode::NEQ => Value::Bool(left != right),
            _ => panic!("eval_bool_rel_op OPERATION not valid for booleans")
        }
    }

    fn eval_log_op(left: Value, op: LogOpcode, right: Value) -> Value {
        let (l, r) = match (left, right) {
            (Value::Bool(l_bool), Value::Bool(r_bool)) => (l_bool, r_bool),
            _ => panic!("eval_log_op LEFT and RIGHT not both booleans")
        };

        match op {
            LogOpcode::AND => Value::Bool(l && r),
            LogOpcode::OR => Value::Bool(l || r)
        }
    }

    fn eval_if_statement(cond: Value, statement: Box<Node>, context: &mut Context, funcs: &mut Funcs) -> Value {
        match cond {
            Value::Bool(b) => match b {
                true => visit(statement, context, funcs),
                false => Value::None
            },
            _ => panic!("eval_if_statement CONDITION did not evaluate to a boolean")
        }
    }

    fn eval_if_else_statement(cond: Value, if_s: Box<Node>, else_s: Box<Node>, context: &mut Context, funcs: &mut Funcs) -> Value {
        match cond {
            Value::Bool(b) => match b {
                true => visit(if_s, context, funcs),
                false => visit(else_s, context, funcs)
            },
            _ => panic!("eval_if_else_statement CONDITION does not evaluate to a boolean")
        }
    }

    fn eval_while_statement(cond: Box<Node>, statement: Box<Node>, context: &mut Context, funcs: &mut Funcs) -> Value {
        match visit(cond.clone(), context, funcs) {
            Value::Bool(b) => match b {
                true => {
                    visit(statement.clone(), context, funcs);
                    eval_while_statement(cond, statement.clone(), context, funcs)
                },
                false => {
					Value::None
				}
            },
            _ => panic!("eval_while_statement CONDITION does not evalute to a boolean")
        }
    }
}