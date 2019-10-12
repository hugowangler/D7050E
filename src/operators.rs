#[derive(Clone, Debug, PartialEq)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,

    // Logical operations
    AND,
    OR,

    // Relational operations
    EQ,
    NEQ,
    GT,
    LT,
    LEQ,
    GEQ,
}

impl Opcode {
    pub fn to_string(&self) -> String {
        match &*self {
            Opcode::Mul => "*".to_string(),
            Opcode::Div => "/".to_string(),
            Opcode::Add => "+".to_string(),
            Opcode::Sub => "-".to_string(),
            Opcode::AND => "&&".to_string(),
            Opcode::OR => "||".to_string(),
            Opcode::EQ => "==".to_string(),
            Opcode::NEQ => "!=".to_string(),
            Opcode::GT => ">".to_string(),
            Opcode::LT => "<".to_string(),
            Opcode::LEQ => "<=".to_string(),
            Opcode::GEQ => ">=".to_string(),
        }
    }
}
