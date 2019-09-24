use super::ParseError;
use crate::ast::Node;

#[allow(dead_code)]
pub fn parse(s: &str) -> Result<Box<Node>, ParseError> {
    let result = crate::parse::grammar::KeywordParser::new().parse(s);
    return match result {
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError{message: e.to_string()}),
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_let_number() {
        assert!(parse("let a:u32 = 2").is_ok());
    }

    #[test]
    fn test_let_expr() {
        assert!(parse("let b : i32 = 2 + a").is_ok());
    }

    #[test]
    fn test_let_rel() {
        assert!(parse("let b : bool = a < b").is_ok());
        assert!(parse("let b : bool = a == b").is_ok());
        assert!(parse("let b : bool = a != b").is_ok());
        assert!(parse("let b : bool = a > b").is_ok());
        assert!(parse("let b : bool = a >= b").is_ok());
        assert!(parse("let b : bool = a <= b").is_ok());
    }

    #[test]
    fn test_let_log() {
        assert!(parse("let b : bool = a && b").is_ok());
        assert!(parse("let b : bool = a || b").is_ok());
    }
}