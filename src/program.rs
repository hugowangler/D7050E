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

    #[test]
    fn update_var_in_new_scope() {
        let res = run(Path::new("tests/scope/update_var_in_new_scope.txt"));
        assert_eq!(res, Some(Value::Number(1)))
    }

    #[test]
    fn new_var_in_scope() {
        let res = run(Path::new("tests/scope/new_var_in_scope.txt"));
        assert_eq!(res, Some(Value::Number(0)))
    }

    #[test]
    fn new_var_in_scope_return() {
        let res = run(Path::new("tests/scope/new_var_in_scope_return.txt"));
        assert_eq!(res, Some(Value::Number(1)))
    }

    #[test]
    #[should_panic]
    fn use_var_outside_scope() {
        run(Path::new("tests/scope/use_var_outside_scope.txt"));
    }

    #[test]
    #[should_panic]
    fn scope_not_same_diff_function() {
        run(Path::new("tests/scope/scope_not_same_diff_function.txt"));
    }

    #[test]
    fn fn_sum() {
        let res = run(Path::new("tests/function/sum.txt"));
        assert_eq!(res, Some(Value::Number(15)))
    }
}
