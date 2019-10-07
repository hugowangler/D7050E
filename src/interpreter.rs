use std::collections::HashMap;

use crate::{
	ast::Node,    
	operators::{BinOpcode, RelOpcode, LogOpcode},
	types::LiteralType,
	value::Value,
	scope::Scope,
	context::Context,
	function::Func,
};

type Funcs = HashMap<String, Func>;	// Stores all the function names and their bodies

pub fn interp(mut ast: Vec<Box<Node>>) -> Option<Value> {
	let mut context = Context::new();
	let mut funcs: Funcs = HashMap::new();
	
	for node in ast.drain(..) {
		visit(node, &mut context, &mut funcs);
	}

	match funcs.get("main") {
		Some(main) => main.clone().call(vec![], &mut context, &mut funcs),
		None => panic!("No main function is defined in the program")
	}
}

pub fn visit(node: Box<Node>, context: &mut Context, funcs: &mut Funcs) -> Value {
	match *node {
		Node::Number(num) => Value::Number(num),
		Node::Bool(b) => Value::Bool(b),
		Node::_String(text) => Value::String(text),
		Node::Var(name) => eval_var(&name, context),
		Node::VarValue(var, expr) => update_var(var, visit(expr, context, funcs), context),
		Node::BinOp(left, op, right) => eval_bin_op(visit(left, context, funcs), op, visit(right, context, funcs)),
		Node::RelOp(left, op, right) => eval_rel_op(visit(left, context, funcs), op, visit(right, context, funcs)),
		Node::LogOp(left, op, right) => eval_log_op(visit(left, context, funcs), op, visit(right, context, funcs)),
		Node::Let(var, expr) => assign_var(var, visit(expr, context, funcs), context),
		Node::Statement(left, right) => {    // Statement is parent node that has children containing statements
			context.push(Scope::new());
 			match *left {
				Node::Return(expr) => {
					// println!("left return = {:#?}", expr);
					return visit(expr, context, funcs)
				},
				_ => ()
			};
			visit(left, context, funcs);
 			match *right {
				Node::Return(expr) => {
					// println!("right return = {:#?}", expr);
					return visit(expr, context, funcs)
				},
				_ => ()
			};
			visit(right, context, funcs);
			context.pop();
			Value::None
		},
		Node::If{cond, statement} => eval_if_statement(visit(cond, context, funcs), statement, context, funcs),
		Node::IfElse{cond, if_statement, else_statement} => eval_if_else_statement(visit(cond, context, funcs), if_statement, else_statement, context, funcs),
		Node::While{cond, statement} => eval_while_statement(cond, statement, context, funcs),
		Node::Func{name, params, r_type, body} => eval_func_dec(&name, &params, r_type, &body, funcs),
		Node::FuncCall{name, args} => eval_func_call(&name, args, context, funcs),
		Node::Return(expr) => visit(expr, context, funcs),
		Node::Print(text) => {
			println!("{:#?}", visit(text, context, funcs));
			Value::None
		},
		_ => panic!("Node not supported: {:?}", *node)
	}
}

fn eval_func_dec(name: &str, params: &Vec<Box<Node>>, r_type: LiteralType, body: &Box<Node>, funcs: &mut Funcs) -> Value {
	let func = Func::new(name.to_string(), params.clone(), r_type, body.clone());

	match funcs.insert(name.to_string(), func) {
		Some(_) => panic!("Function: {} is already defined", name.clone()),
		None => Value::None
	}
}

fn eval_func_call(name: &str, args: Vec<Box<Node>>, context: &mut Context, funcs: &mut Funcs) -> Value {
	match funcs.get(name) {
		Some(func) => match func.clone().call(args, context, funcs) {
			Some(ret) => ret,
			None => Value::None
		},
		None => panic!("eval_func_call: Function \"{}\" is not defined", name)
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
		Node::Var(name) => context.insert_var(name, expr),
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

fn eval_var(name: &str, context: &mut Context) -> Value {
	match context.get_var(name) {
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
	match cond.clone() {
		Value::Bool(b) => match b {
			true => visit(statement, context, funcs),
			false => Value::None
		},
		_ => panic!("eval_if_statement CONDITION did not evaluate to a boolean")
	}
}

fn eval_if_else_statement(cond: Value, if_s: Box<Node>, else_s: Box<Node>, context: &mut Context, funcs: &mut Funcs) -> Value {
	match cond.clone() {
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
