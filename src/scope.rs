use std::collections::HashMap;

use crate::{ast::Node, value::Value, variable::Variable};

#[derive(Clone, Debug, PartialEq)]
pub struct Scope {
    pub vars: HashMap<String, Variable>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            vars: HashMap::new(),
        }
    }

    pub fn init(vars: Vec<(&Node, Value)>) -> Scope {
        let mut scope = Scope::new();
        for pair in vars.iter() {
            let (param, arg) = pair;
            match param {
                Node::FuncParam(var, _typ) => {
                    match &**var {
                        Node::Var(name) => scope.vars.insert(name.to_string(), arg.clone()),
                        _ => unreachable!(),
                    };
                }
                _ => unreachable!(),
            }
        }
        scope
    }
}
