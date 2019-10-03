use std::collections::HashMap;

use crate::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct Scope {
	pub vars: HashMap<String, Value>
}

impl Scope {
	pub fn new() -> Scope {
		Scope{
			vars: HashMap::new()
		}
	}
}