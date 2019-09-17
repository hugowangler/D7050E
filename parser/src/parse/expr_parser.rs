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
    use super::parse;

    #[test]
    fn test_expr_number() {
        assert!(parse("123").is_ok());
    }

    #[test]
    fn test_expr_number_paran() {
        assert!(parse("(1)").is_ok());
        assert!(parse("(1+2))").is_err());
        assert!(parse("((1+2))").is_ok());
    }

    #[test]
    fn test_expr_var() {
        assert!(parse("a + b + 2 - 5").is_ok());
    }

    #[test]
    fn test_expr_op() {
        assert!(parse("a + b").is_ok());    // Add
        assert!(parse("a - b").is_ok());    // Sub
        assert!(parse("a / b").is_ok());    // Div
        assert!(parse("a * b").is_ok());    // Mul
    }

}