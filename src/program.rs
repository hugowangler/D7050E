use std::{error::Error, fs::File, io::prelude::*, path::Path};

use crate::{interpreter::interp, parse::program_parser::parse, value::Value};

pub fn run(path: &Path) -> Option<Value> {
    let display = path.display();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Could not open {}: {}", display, e.description()),
    };

    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Ok(_) => (),
        Err(e) => panic!("Could not read file: {:?}", e),
    }

    match parse(input) {
        Ok(parsed_prog) => {
            print!("parsed_prog = {:#?}", &parsed_prog);
            interp(parsed_prog)
        }
        Err(e) => panic!("Error while parsing program: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expression precedence
    #[test]
    fn add_sub_mul_prec() {
        assert_eq!(
            run(Path::new("tests/precedence/add_sub.txt")),
            Some(Value::Number(9))
        );
        assert_eq!(
            run(Path::new("tests/precedence/add_sub_mul.txt")),
            Some(Value::Number(27))
        );
    }

    #[test]
    fn paran_prec() {
        assert_eq!(
            run(Path::new("tests/precedence/paran_mul.txt")),
            Some(Value::Number(24))
        );
        assert_eq!(
            run(Path::new("tests/precedence/paran_mul_rev.txt")),
            Some(Value::Number(24))
        );
        assert_eq!(
            run(Path::new("tests/precedence/paran_add.txt")),
            Some(Value::Number(25))
        );
        assert_eq!(
            run(Path::new("tests/precedence/paran_add_rev.txt")),
            Some(Value::Number(25))
        );
    }

    // Scopes
    #[test]
    fn if_update_var_in_new_scope() {
        let res = run(Path::new("tests/scope/if_update_var_new_scope.txt"));
        assert_eq!(res, Some(Value::Number(1)))
    }

    #[test]
    fn if_var_in_scope() {
        let res = run(Path::new("tests/scope/if_var_in_scope.txt"));
        assert_eq!(res, Some(Value::Number(0)))
    }

    #[test]
    fn if_var_in_scope_return() {
        let res = run(Path::new("tests/scope/if_var_in_scope_return.txt"));
        assert_eq!(res, Some(Value::Number(1)))
    }

    #[test]
    #[should_panic]
    fn if_var_outside_scope() {
        run(Path::new("tests/scope/if_var_outside_scope.txt"));
    }

    #[test]
    #[should_panic]
    fn fn_scope_not_same() {
        run(Path::new("tests/scope/fn_scope_not_same.txt"));
    }

    #[test]
    fn while_update_var_new_scope() {
        assert_eq!(
            run(Path::new("tests/scope/while_update_var_new_scope.txt")),
            Some(Value::Number(5))
        )
    }

    #[test]
    fn while_var_in_scope_return() {
        assert_eq!(
            run(Path::new("tests/scope/while_var_in_scope_return.txt")),
            Some(Value::Number(1))
        )
    }

    #[test]
    fn while_var_in_scope() {
        assert_eq!(
            run(Path::new("tests/scope/while_var_in_scope.txt")),
            Some(Value::Number(0))
        )
    }

    #[test]
    #[should_panic]
    fn while_var_outside_scope() {
        run(Path::new("tests/scope/while_var_outside_scope.txt"));
    }

    #[test]
    fn if_else_update_var_in_scope() {
        assert_eq!(
            run(Path::new("tests/scope/if_else_update_var_new_scope.txt")),
            Some(Value::Number(10))
        )
    }

    #[test]
    fn if_else_var_in_scope_return() {
        assert_eq!(
            run(Path::new("tests/scope/if_else_var_in_scope_return.txt")),
            Some(Value::Number(20))
        )
    }

    #[test]
    fn if_else_var_in_scope() {
        assert_eq!(
            run(Path::new("tests/scope/if_else_var_in_scope.txt")),
            Some(Value::Number(1))
        )
    }

    #[test]
    #[should_panic]
    fn if_else_var_outside_scope() {
        run(Path::new("tests/scope/if_else_var_outside_scope.txt"));
    }

    // Functions
    #[test]
    fn fn_sum() {
        let res = run(Path::new("tests/function/sum.txt"));
        assert_eq!(res, Some(Value::Number(15)))
    }

    #[test]
    fn fn_fibonacci() {
        let res = run(Path::new("tests/function/fibonacci.txt"));
        assert_eq!(res, Some(Value::Number(6765)))
    }

    // Mutability
    #[test]
    #[should_panic]
    fn no_mut_var() {
        run(Path::new("tests/mutability/no_mut_var.txt"));
    }

    #[test]
    fn mut_var() {
        assert_eq!(
            run(Path::new("tests/mutability/mut_var.txt")),
            Some(Value::Number(100))
        );
    }

    #[test]
    #[should_panic]
    fn fn_no_mut_param() {
        run(Path::new("tests/mutability/fn_no_mut_param.txt"));
    }

    #[test]
    fn fn_mut_param() {
        assert_eq!(
            run(Path::new("tests/mutability/fn_mut_param.txt")),
            Some(Value::Number(51))
        );
    }
}
