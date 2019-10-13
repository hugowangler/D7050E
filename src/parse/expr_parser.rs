use super::ParseError;
use crate::ast::Node;

#[allow(dead_code)]
pub fn parse(s: &str) -> Result<Box<Node>, ParseError> {
    let result = crate::parse::grammar::ExprParser::new().parse(s);
    return match result {
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError {
            message: e.to_string(),
        }),
    };
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
        assert!(parse("a + b").is_ok()); // Add
        assert!(parse("a - b").is_ok()); // Sub
        assert!(parse("a / b").is_ok()); // Div
        assert!(parse("a * b").is_ok()); // Mul
    }

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
    fn test_relexpr_var() {
        assert!(parse("a + b + 2 - 5 != c").is_ok());
    }

    #[test]
    fn test_rlexpr_op() {
        assert!(parse("hej > a + b").is_ok()); // GT
        assert!(parse("c < a - b").is_ok()); // LT
        assert!(parse("a <= b / 3").is_ok()); // LEQ
        assert!(parse("a >= b * 3 / 5").is_ok()); // GEQ
    }

    #[test]
    fn test_logexpr_expr() {
        assert!(parse("a && 123").is_ok()); // Var Log Num
        assert!(parse("true && 123").is_ok()); // bool Log Num
        assert!(parse("a && true").is_ok()); // Var Log Bool
    }

    #[test]
    fn test_logexpr_rel() {
        assert!(parse("a > 1 && b < 3").is_ok());
    }

    #[test]
    fn test_logexpr_log() {
        assert!(parse("a && b || c && a").is_ok());
    }

    #[test]
    fn test_logexpr_rellog() {
        assert!(parse("a || b && a < c").is_ok());
        assert!(parse("a < b && a == c").is_ok());
    }

    #[test]
    fn test_logexpr_exprlog() {
        assert!(parse("a && a < c").is_ok());
        assert!(parse("a < b || a + 6").is_ok());
    }

    #[test]
    fn test_logexpr_number_paran() {
        assert!(parse("a || (1)").is_ok());
        assert!(parse("b && (1+2))").is_err());
        assert!(parse("c && ((1+2))").is_ok());
    }
}
