use crate::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
	value: Value,
	mutable: bool
}

impl Variable {
	pub fn new(val: Value, mutable: bool) -> Variable {
		Variable {
			value: val,
			mutable: mutable
		}
	}

	pub fn get_value(&self) -> Value {
		self.value
	}

	pub fn update_value(&self, val: Value) {
		self.value = val;
	}

	pub fn is_mut(&self) -> bool {
		self.mutable
	}
}
