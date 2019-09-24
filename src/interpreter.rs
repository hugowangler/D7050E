pub mod interpreter {
    use crate::ast::Node;
    use crate::operators::BinOpcode;

    enum Value {
        Number(i32),
        Bool(bool),
        Var(String),
    }

    pub fn interp(mut ast: Vec<Box<Node>>) {
        for node in ast.drain(..) {
            visit(node);
        }
    }


    fn visit(node: Box<Node>) -> Value {
        match *node {
            Node::Number(num) => Value::Number(num),
            Node::Bool(b) => Value::Bool(b),

            Node::BinOp(left, op, right) => eval_bin_op(visit(left), op, visit(right)),

            _ => panic!("Node not supported")
        }
    }

    // TODO: add floats to grammar and handle them here
    fn eval_bin_op(left: Value, op: BinOpcode, right: Value) -> Value {
        let left_value = match left {
            Value::Number(num) => num,
            _ => panic!("eval_bin_op left no number")
        };

        let right_value = match right {
            Value::Number(num) => num,
            _ => panic!("eval_bin_op right no number")
        };

        match op {
            BinOpcode::Add => Value::Number(left_value + right_value),
            BinOpcode::Sub => Value::Number(left_value - right_value),
            BinOpcode::Div => Value::Number(left_value / right_value),
            BinOpcode::Mul => Value::Number(left_value * right_value),
        }
    }

}