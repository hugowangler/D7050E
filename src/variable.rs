use crate::{types::LiteralType, value::Value};

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    value: Value,
    mutable: bool,
    typ: LiteralType,
}

impl Variable {
    pub fn new(val: Value, mutable: bool, typ: LiteralType) -> Variable {
        Variable {
            value: val,
            mutable: mutable,
            typ: typ,
        }
    }

    pub fn get_value(&self) -> Value {
        self.value.clone()
    }

    pub fn get_type(&self) -> LiteralType {
        self.typ
    }

    pub fn update_value(&mut self, val: Value) -> Value {
        self.value = val.clone();
        val
    }

    pub fn is_mut(&self) -> bool {
        self.mutable
    }
}
