#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralType {
    Bool,
    I32,
    _String,
    Void,
}

impl LiteralType {
    pub fn to_string(&self) -> String {
        match self {
            LiteralType::Bool => "bool".to_string(),
            LiteralType::I32 => "i32".to_string(),
            LiteralType::_String => "string".to_string(),
            LiteralType::Void => "()".to_string(),
        }
    }
}
