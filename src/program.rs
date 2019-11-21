use std::{
    error::Error,
    fs::File,
    io::prelude::*,
    io::{self, Write},
    path::Path,
};

use crate::{interpreter::interp, llvm::Compiler, parse::program_parser::parse, type_checker::type_check};

/// Runs a program defined in the path, if compile is false the program is interpreted
/// otherwise it will be compiled with llvm
pub fn run(path: &Path, compile: bool) -> io::Result<()> {
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
            // println!("parsed_prog = {:#?}", &parsed_prog);
            match type_check(parsed_prog.clone()) {
                Ok(_) => {
                    if !compile {
                        match interp(parsed_prog) {
                            Some(res) => {
                                return io::stdout().write_fmt(format_args!("{:?}\n", res))
                            }
                            None => return Ok(()),
                        }
                    } else {
						let mut compiler = Compiler::new();
						let main_fn = compiler.compile(&parsed_prog);
						unsafe{return io::stdout().write_fmt(format_args!{"Execution result = {}\n", main_fn.call()})}
                    }
                }
                Err(e) => {
                    for error in e.errors.iter() {
                        io::stderr().write_fmt(format_args!("Error: {}\n", error))?
                    }
                    io::stderr().write_fmt(format_args!("Could not compile '{}'\n", display))
                }
            }
        }
        Err(e) => panic!("Error while parsing 'input.txt': {:?}", e),
    }
}
