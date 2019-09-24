use super::ParseError;
use crate::ast::Node;

#[allow(dead_code)]
pub fn parse(s: &str) -> Result<Box<Node>, ParseError> {
    let result = crate::parse::grammar::RelExprParser::new().parse(s);
    return match result {
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError{message: e.to_string()}),
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_relexpr_number() {
        assert!(parse("123 == 123").is_ok());
    }

    #[test]
    fn test_relexpr_number_paran() {
        assert!(parse("a < (1+2-3)").is_ok());
        assert!(parse("b > (tjena)").is_ok());
        assert!(parse("c == ((1+2))").is_ok());
    }

    #[test]
    fn test_expr_var() {
        assert!(parse("a + b + 2 - 5 != c").is_ok());
    }

    #[test]
    fn test_expr_op() {
        assert!(parse("hej > a + b").is_ok());    // GT
        assert!(parse("c < a - b").is_ok());    // LT
        assert!(parse("a <= b / 3").is_ok());    // LEQ
        assert!(parse("a >= b * 3 / 5").is_ok());    // GEQ
    }

}