use super::ParseError;
use crate::ast::Node;

pub fn parse(s: &str) -> Result<Box<Node>, ParseError> {
    let result = crate::parse::grammar::StatementParser::new().parse(s);
    return match result {
        Ok(s) => Ok(s),
        Err(e) => Err(ParseError{message: e.to_string()}),
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_state_1() {
        assert!(parse("let a: i32 = 12;").is_ok())
    }

    #[test]
    fn test_state_mult() {
        assert!(parse("let a: i32 = 12; let a: i32 = 12; let a: i32 = 12; let a: i32 = 12;").is_ok())
    }

       #[test]
    fn test_state_rel() {
        assert!(parse("let b: bool = a > 1 && b < 3;").is_ok());
    }

    #[test]
    fn test_state_log() {
        assert!(parse("let b: bool = a && b || c && a;").is_ok());
    }

    #[test]
    fn test_state_rellog() {
        assert!(parse("let xyz: bool = a || b && a < c;").is_ok());
        assert!(parse("let xyz: bool = a < b && a == c;").is_ok());

    }
    
    fn test_state_exprlog() {
        assert!(parse("let xyz: bool = a && a < c;").is_ok());
        assert!(parse("let xyz: bool = a < b || a + 6;").is_ok());
    }

    #[test]
    fn test_expr_number_paran() {
        assert!(parse("let xyz: bool = a || (1);").is_ok());
        assert!(parse("let xyz: bool = b && (1+2));").is_err());
        assert!(parse("let xyz: bool = c && ((1+2));").is_ok());
    }
}


