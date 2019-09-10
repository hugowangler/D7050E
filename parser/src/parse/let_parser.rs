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
        assert!(parse("let a = 2;").is_ok());
    }

    #[test]
    fn test_let_expr() {
        assert!(parse("let b = 2 + a;").is_ok());
    }
}