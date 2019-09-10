use super::ParseError;
use crate::ast::Node;

pub fn parse(s: &str) -> Result<Box<Node>, ParseError> {
    let result = crate::parse::grammar::ExprParser::new().parse(s);
    return match result {
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError{message: e.to_string()}),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_number() {
        assert!(parse("123").is_ok());
    }

    #[test]
    fn test_expr_number_paran() {
        assert!(parse("(1)").is_ok());
    }
}