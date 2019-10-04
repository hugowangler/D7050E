use crate::{
	value::Value,
	scope::Scope
};

#[derive(Clone, Debug, PartialEq)]
pub struct Context {
	pub scopes: Vec<Scope>,
}

impl Context {
	pub fn new() -> Context {
		Context{
			scopes: vec![]
		}
	}

	pub fn insert_var(&mut self, name: String, value: Value) {
		match self.scopes.iter_mut().last() {
			Some(scope) => scope.vars.insert(name, value),
			None => panic!("insert_var: No scope in context")
		};
	}

	pub fn update_var(&mut self, name: String, value: Value) -> Option<Value> {
		for scope in self.scopes.iter_mut().rev() {
			match scope.vars.get(&name.clone()) {
				Some(_) => return scope.vars.insert(name.clone(), value.clone()),
				None => ()
			};
		}
		None
	}

	pub fn get_var(&mut self, name: &str) -> Option<Value> {
		for scope in self.scopes.iter().rev() {
			match scope.vars.get(name) {
				Some(value) => return Some(value.clone()),
				None => ()
			};
		}
		None
	}

	pub fn push(&mut self, scope: Scope) {
		self.scopes.push(scope);
	}

	pub fn pop(&mut self) -> Scope {
		match self.scopes.pop() {
			Some(scope) => scope,
			None => panic!("Trying to pop from empty context scope")
		}
	}
}

