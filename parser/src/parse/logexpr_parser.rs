use super::ParseError;
use crate::ast::Node;

pub fn parse(s: &str) -> Result<Box<Node>, ParseError> {
    let result = crate::parse::grammar::LogExprParser::new().parse(s);
    return match result {
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError{message: e.to_string()}),
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_logexpr_number() {
        assert!(parse("a && 123").is_ok());
    }

    #[test]
    fn test_expr_number_paran() {
        assert!(parse("a || (1)").is_ok());
        assert!(parse("b && (1+2))").is_err());
        assert!(parse("c && ((1+2))").is_ok());
    }

}