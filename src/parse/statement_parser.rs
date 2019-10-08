use super::ParseError;
use crate::ast::Node;

#[allow(dead_code)]
pub fn parse(s: &str) -> Result<Box<Node>, ParseError> {
    let result = crate::parse::grammar::StatementParser::new().parse(s);
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
    fn test_state_let() {
        assert!(parse("let a: i32 = 12;").is_ok());
        assert!(
            parse("let a: i32 = 12; let a: i32 = 12; let a: i32 = 12; let a: i32 = 12;").is_ok()
        );
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
        assert!(parse(
            "if (b && c) {
                    let xyz: bool = a < b || a + 6;

                    if (xyz == true) {
                        let xyz: bool = c;
                    }
                }"
        )
        .is_ok());
    }

    #[test]
    fn test_state_if_else() {
        assert!(parse(
            "if (b && c) {
                    let xyz: bool = a < b || a + 6;
                } else {
                    let b: i32 = 15;
                }"
        )
        .is_ok());
    }

    #[test]
    fn test_state_rec_if_else() {
        assert!(parse(
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
        )
        .is_ok());
    }

    #[test]
    fn test_state_else_if() {
        assert!(parse(
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
        )
        .is_ok());
    }

    #[test]
    fn test_state_else_if_else() {
        assert!(parse(
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
        )
        .is_ok());
    }

    #[test]
    fn test_state_mul_else_if() {
        assert!(parse(
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
        )
        .is_ok());
    }

    #[test]
    fn test_state_while() {
        assert!(parse(
            "while (done) {
                    let x: i32 = x + 1;

                    if (x == 5) {
                        done = true;
                    }
                }"
        )
        .is_ok());
    }

    #[test]
    fn test_state_rec_while() {
        assert!(parse(
            "while (done) {
                    let x: i32 = x + 1;
                    
                    while (x > 5) {
                        let y: i32 = 12 - x;
                        x = x - 1;
                    }

                    if (x == 5) {
                        let done: bool = false;
                    }
                }"
        )
        .is_ok());
    }

    #[test]
    fn test_state_loop_mod() {
        assert!(parse(
            "while (done) {
                    let x: i32 = x + 1;
                    
                    while (x > 5) {
                        let y: i32 = 12 - x;
                        x = x - 1;
                        continue;
                    }

                    if (x == 5) {
                        let done: bool = false;
                        break;
                    }
                }"
        )
        .is_ok());
    }

    #[test]
    fn test_state_return() {
        assert!(parse(
            "if (true) {
                    return 5;
                }"
        )
        .is_ok());
    }

    #[test]
    fn test_state_func_call() {
        assert!(parse("main();").is_ok());
    }

    #[test]
    fn test_state_func_call_var() {
        assert!(parse("main(x);").is_ok());
    }

    #[test]
    fn test_state_func_call_mul_vars() {
        assert!(parse("main(x, y);").is_ok());
    }

    #[test]
    fn test_state_func_call_num() {
        assert!(parse("main(x, y, 123);").is_ok());
    }

    #[test]
    fn test_state_func_call_bool() {
        assert!(parse("main(x, y, 123, true);").is_ok());
    }

    #[test]
    fn test_state_func_call_paran() {
        assert!(parse("main x, y, 123, true);").is_err());
        assert!(parse("main x, y, 123, true;").is_err());
    }

    #[test]
    fn test_state_assign_func() {
        assert!(parse("res = sum();").is_ok());
    }

    #[test]
    fn test_state_def_var_func() {
        assert!(parse("let res: i32 = sum();").is_ok());
    }
}
