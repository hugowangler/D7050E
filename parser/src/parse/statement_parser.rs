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
    fn test_state_let() {
        assert!(parse("let a: i32 = 12;").is_ok());
        assert!(parse("let a: i32 = 12; let a: i32 = 12; let a: i32 = 12; let a: i32 = 12;").is_ok());
        assert!(parse("let b: bool = a > 1 && b < 3;").is_ok());
        assert!(parse("let b: bool = a && b || c && a;").is_ok());
        assert!(parse("let xyz: bool = a || b && a < c;").is_ok());
        assert!(parse("let xyz: bool = a < b && a == c;").is_ok());
        assert!(parse("let xyz: bool = a && a < c;").is_ok());
        assert!(parse("let xyz: bool = a < b || a + 6;").is_ok());
    }

    #[test]
    fn test_state_if() {
        assert!(parse("if (b) {let xyz: bool = a < b || a + 6;}").is_ok());
        assert!(parse("if (b > 5) {let xyz: i32 = a < b || a + 6;}").is_ok());
        assert!(parse("if (b && c) {let xyz: bool = a < b || a + 6;}").is_ok());

    }

    #[test]
    fn test_state_rec_if() {
        assert!(
            parse(
                "if (b && c) {
                    let xyz: bool = a < b || a + 6;

                    if (xyz == true) {
                        let xyz: bool = c;
                    }
                }"
            ).is_ok());
    }

    #[test]
    fn test_state_if_else() {
        assert!(
            parse(
                "if (b && c) {
                    let xyz: bool = a < b || a + 6;
                } else {
                    let b: i32 = 15;
                }"
            ).is_ok());
    }

    #[test]
    fn test_state_rec_if_else() {
        assert!(
            parse(
                "if (b && c) {
                    let xyz: bool = a < b || a + 6;
                } else {
                    if (c == 5) {
                        let a: i32 = 123123 == b;
                    } else {
                        let a: i32 = b && c;
                    }

                    let b: i32 = 15;
                }"
            ).is_ok()
        );
    }

    #[test]
    fn test_state_else_if() {
        assert!(
            parse(
                "if (b && c) {
                    let xyz: bool = a < b || a + 6;
                } else if (b && d) {
                    if (c == 5) {
                        let a: i32 = 123123 == b;
                    } else {
                        let a: i32 = b && c;
                    }

                let b: i32 = 15;
                }"
            ).is_ok()
        );
    }

    #[test]
    fn test_state_else_if_else() {
        assert!(
            parse(
                "if (b && c) {
                    let xyz: bool = a < b || a + 6;
                } else if (b && d) {
                    if (c == 5) {
                        let a: i32 = 123123 == b;
                    } else {
                        let a: i32 = b && c;
                    }

                    let b: i32 = 15;
                } else {
                    let asd: bool = true;
                }"
            ).is_ok()
        );
    }

    #[test]
    fn test_state_mul_else_if() {
        assert!(
            parse(
                "if (b && c) {
                    let xyz: bool = a < b || a + 6;
                } else if (b && d) {
                    if (c == 5) {
                        let a: i32 = 123123 == b;
                    } else {
                        let a: i32 = b && c;
                    }
                    let b: i32 = 15;
                } else if (b) {
                    let c: i32 = 12;
                } else if (c) {
                    let d: i32 = a && b;
                } else {
                    let asd: bool = true;
                }"
            ).is_ok()
        );
    }

}


