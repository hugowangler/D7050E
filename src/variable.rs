use crate::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    value: Value,
    mutable: bool,
}

impl Variable {
    pub fn new(val: Value, mutable: bool) -> Variable {
        Variable {
            value: val,
            mutable: mutable,
        }
    }

    pub fn get_value(&self) -> Value {
        self.value.clone()
    }

    pub fn update_value(&mut self, val: Value) -> Value {
        self.value = val.clone();
        val
    }

    pub fn is_mut(&self) -> bool {
        self.mutable
    }
}
