pub mod interpreter {
    use std::collections::HashMap;

    use crate::ast::Node;    
    use crate::operators::{
        BinOpcode,
        RelOpcode,
        LogOpcode
    };
	use crate::types::LiteralType;

    #[derive(Clone, Debug, PartialEq)]
    enum Value {
        Number(i32),
        Bool(bool),
		String(String),
        None,
    }

	type Scope = HashMap<String, Value>;

    pub fn interp(mut ast: Vec<Box<Node>>) {
        let mut context: Vec<Scope> = Vec::new();
        for node in ast.drain(..) {
            visit(node, &mut context);
        }
        // println!("context = {:#?}", context);
    }

    fn visit(node: Box<Node>, context: &mut Vec<Scope>) -> Value {
        match *node {
            Node::Number(num) => Value::Number(num),
            Node::Bool(b) => Value::Bool(b),
			Node::_String(text) => Value::String(text),
            Node::Var(name) => eval_var(&name, context),
			Node::VarValue(var, expr) => update_var(var, visit(expr, context), context),
            Node::BinOp(left, op, right) => eval_bin_op(visit(left, context), op, visit(right, context)),
            Node::RelOp(left, op, right) => eval_rel_op(visit(left, context), op, visit(right, context)),
            Node::LogOp(left, op, right) => eval_log_op(visit(left, context), op, visit(right, context)),
            Node::Let(var, expr) => assign_var(var, visit(expr, context), context),
            Node::Statement(left, right) => {    // Statement is parent node that has children containing statements
                visit(left, context);
                visit(right, context);
                Value::None     // Garbage enum just to not get rust error of no return value
            },
            Node::If{cond, statement} => eval_if_statement(visit(cond, context), statement, context),
            Node::IfElse{cond, if_statement, else_statement} => eval_if_else_statement(visit(cond, context), if_statement, else_statement, context),
            Node::While{cond, statement} => eval_while_statement(cond, statement, context),
			Node::Func{name, params, r_type, body} => eval_func(name, params, r_type, body, context),
            Node::Print(text) => {
                println!("{:#?}", visit(text, context));
                Value::None
            }
            _ => panic!("Node not supported: {:?}", *node)
        }
    }

	fn eval_func(name: String, params: Vec<Box<Node>>, r_type: LiteralType, body: Box<Node>, context: &mut Vec<Scope>) -> Value {
		let scope = new_scope(context, name);
		Value::None
	}

    fn assign_var(var: Box<Node>, expr: Value, context: &mut Vec<Scope>) -> Value {
        match *var {
            Node::VarBinding(var, _var_type) => def_var(var, expr, context),
            _ => panic!("assign_var: No VarBinding node")
        };
        Value::None
    }

	fn def_var(var: Box<Node>, expr: Value, context: &mut Vec<Scope>) {
		match *var {
			Node::Var(name) => {
				let mut scope = get_scope(context);
				scope.insert(name, expr);
				context.push(scope);
			},
			_ => panic!("def_var: No var node")
		}
	}

	fn update_var(var: Box<Node>, expr: Value, context: &mut Vec<Scope>) -> Value {
		match *var {
			Node::Var(name) => {
				let mut scope = get_scope(context);

				match scope.insert(name.clone(), expr) {
					Some(_) => {
						context.push(scope);
						Value::None
					},
					None => panic!("Error: Variable {:?} is not defined", name)
				}
			},
			_=> panic!("update_var: Node not Var type")
		}
	}

	fn get_scope(context: &mut Vec<Scope>) -> Scope {
		match context.pop() {
			Some(scope) => scope,
			None => panic!("get_scope: Context contains no scope")
		}
	}

	fn new_scope(context: &mut Vec<Scope>, name: String) {
		let scope: Scope = HashMap::new();
		context.push(scope);
	}

    fn eval_var(name: &str, context: &mut Vec<Scope>) -> Value {
        match get_scope(context).get(name) {
            Some(res) => res.clone(),
            None => panic!("Error: Variable {:?} is not defined", name)
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

    fn eval_if_statement(cond: Value, statement: Box<Node>, context: &mut Vec<Scope>) -> Value {
        match cond {
            Value::Bool(b) => match b {
                true => visit(statement, context),
                false => Value::None
            },
            _ => panic!("eval_if_statement CONDITION did not evaluate to a boolean")
        }
    }

    fn eval_if_else_statement(cond: Value, if_s: Box<Node>, else_s: Box<Node>, context: &mut Vec<Scope>) -> Value {
        match cond {
            Value::Bool(b) => match b {
                true => visit(if_s, context),
                false => visit(else_s, context)
            },
            _ => panic!("eval_if_else_statement CONDITION does not evaluate to a boolean")
        }
    }

    fn eval_while_statement(cond: Box<Node>, statement: Box<Node>, context: &mut Vec<Scope>) -> Value {
        match visit(cond.clone(), context) {
            Value::Bool(b) => match b {
                true => {
                    visit(statement.clone(), context);
                    eval_while_statement(cond, statement.clone(), context)
                },
                false => Value::None
            },
            _ => panic!("eval_while_statement CONDITION does not evalute to a boolean")
        }
    }
}