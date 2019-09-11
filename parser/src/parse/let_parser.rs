use super::ParseError;
use crate::ast::Node;

pub fn parse(s: &str) -> Result<Box<Node>, ParseError> {
    let result = crate::parse::grammar::LetParser::new().parse(s);
    return match result {
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError{message: e.to_string()}),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_number() {
        assert!(parse("let a:u32 = 2;").is_ok());
    }

    #[test]
    fn test_let_expr() {
        assert!(parse("let b : i32 = 2 + a;").is_ok());
    }

    #[test]
    fn test_let_op() {
        assert!(parse("let b : i32 = 1 + 2;").is_ok());   // Add
        assert!(parse("let a : u32 = 2 - b;").is_ok());   // Sub
        assert!(parse("let b : i8 = 2 / a;").is_ok());   // Div
        assert!(parse("let b : i16 = 2 * a;").is_ok());   // Mul
    }
}