use crate::{scope::Scope, types::LiteralType, value::Value, variable::Variable};

#[derive(Clone, Debug, PartialEq)]
pub struct Context {
    pub scopes: Vec<Scope>,
}

impl Context {
    pub fn new() -> Context {
        Context { scopes: vec![] }
    }

    pub fn insert_var(&mut self, name: String, mutable: bool, typ: LiteralType, value: Value) {
        let new_var = Variable::new(value, mutable, typ);
        match self.scopes.iter_mut().last() {
            Some(scope) => scope.vars.insert(name, new_var),
            None => panic!("insert_var: No scope in context"),
        };
    }

    pub fn update_var(&mut self, name: String, value: Value) -> Option<Value> {
        for scope in self.scopes.iter_mut().rev() {
            match scope.vars.get_mut(&name.clone()) {
                Some(var) => {
                    if var.is_mut() {
                        return Some(var.update_value(value.clone()));
                    } else {
                        panic!("Cannot assign twice to immutable variable \"{}\"", name)
                    }
                }
                None => (),
            };
        }
        None // Variable was not found in any scope
    }

    pub fn get_var(&mut self, name: &str) -> Option<&Variable> {
        for scope in self.scopes.iter().rev() {
            match scope.vars.get(name) {
                Some(var) => return Some(var),
                None => (),
            };
        }
        None
    }

    pub fn get_var_value(&mut self, name: &str) -> Option<Value> {
        match self.get_var(name) {
            Some(var) => Some(var.get_value()),
            None => None,
        }
    }

    pub fn push(&mut self, scope: Scope) {
        self.scopes.push(scope);
    }

    pub fn pop(&mut self) -> Scope {
        match self.scopes.pop() {
            Some(scope) => scope,
            None => panic!("Trying to pop from empty context scope"),
        }
    }
}
