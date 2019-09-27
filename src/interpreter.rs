pub mod interpreter {
    use std::collections::HashMap;

    use crate::ast::Node;    
    use crate::operators::{
        BinOpcode,
        RelOpcode,
        LogOpcode
    };

    #[derive(Clone, Debug, PartialEq)]
    enum Value {
        Number(i32),
        Bool(bool),
		String(String),
        None,
    }

    pub fn interp(mut ast: Vec<Box<Node>>) {
        let mut vars: HashMap<String, Value> = HashMap::new();
        for node in ast.drain(..) {
            visit(node, &mut vars);
        }
        // println!("vars = {:#?}", vars);
    }

    fn visit(node: Box<Node>, map: &mut HashMap<String, Value>) -> Value {
        match *node {
            Node::Number(num) => Value::Number(num),
            Node::Bool(b) => Value::Bool(b),
			Node::_String(text) => Value::String(text),
            Node::Var(name) => eval_var(&name, map),
			Node::VarValue(var, expr) => update_var(var, visit(expr, map), map),
            Node::BinOp(left, op, right) => eval_bin_op(visit(left, map), op, visit(right, map)),
            Node::RelOp(left, op, right) => eval_rel_op(visit(left, map), op, visit(right, map)),
            Node::LogOp(left, op, right) => eval_log_op(visit(left, map), op, visit(right, map)),
            Node::Let(var, expr) => assign_var(var, visit(expr, map), map),
            Node::Statement(left, right) => {    // Statement is parent node that has children containing statements
                visit(left, map);
                visit(right, map);
                Value::None     // Garbage enum just to not get rust error of no return value
            },
            Node::If{cond, statement} => eval_if_statement(visit(cond, map), statement, map),
            Node::IfElse{cond, if_statement, else_statement} => eval_if_else_statement(visit(cond, map), if_statement, else_statement, map),
            Node::While{cond, statement} => eval_while_statement(cond, statement, map),
			Node::Func{name, params, r_type, body} => Value::None,
            Node::Print(text) => {
                println!("{:#?}", visit(text, map));
                Value::None
            }
            _ => panic!("Node not supported: {:?}", *node)
        }
    }

	fn eval_func() {

	}

    fn assign_var(var: Box<Node>, expr: Value, map: &mut HashMap<String, Value>) -> Value {
        match *var {
            Node::VarBinding(var, _var_type) => def_var(var, expr, map),
            _ => panic!("assign_var: No VarBinding node")
        };
        Value::None
    }

	fn def_var(var: Box<Node>, expr: Value, map: &mut HashMap<String, Value>) {
		match *var {
			Node::Var(name) => {
				map.insert(name, expr);
			},
			_ => panic!("def_var: No var node")
		}
	}

	fn update_var(var: Box<Node>, expr: Value, map: &mut HashMap<String, Value>) -> Value {
		match *var {
			Node::Var(name) => {
				match map.insert(name.clone(), expr) {
					Some(_res) => Value::None,
					None => panic!("Error: Variable {:?} is not defined", name)
				}
			},
			_=> panic!("update_var: Node not Var type")
		}
	}

    fn eval_var(name: &str, map: &mut HashMap<String, Value>) -> Value {
        match map.get(name) {
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

    fn eval_if_statement(cond: Value, statement: Box<Node>, map: &mut HashMap<String, Value>) -> Value {
        match cond {
            Value::Bool(b) => match b {
                true => visit(statement, map),
                false => Value::None
            },
            _ => panic!("eval_if_statement CONDITION did not evaluate to a boolean")
        }
    }

    fn eval_if_else_statement(cond: Value, if_s: Box<Node>, else_s: Box<Node>, map: &mut HashMap<String, Value>) -> Value {
        match cond {
            Value::Bool(b) => match b {
                true => visit(if_s, map),
                false => visit(else_s, map)
            },
            _ => panic!("eval_if_else_statement CONDITION does not evaluate to a boolean")
        }
    }

    fn eval_while_statement(cond: Box<Node>, statement: Box<Node>, map: &mut HashMap<String, Value>) -> Value {
        match visit(cond.clone(), map) {
            Value::Bool(b) => match b {
                true => {
                    visit(statement.clone(), map);
                    eval_while_statement(cond, statement.clone(), map)
                },
                false => Value::None
            },
            _ => panic!("eval_while_statement CONDITION does not evalute to a boolean")
        }
    }
}