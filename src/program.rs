use std::{
    error::Error,
    fs::File,
    io::prelude::*,
    io::{self, Write},
    path::Path,
};

use crate::{interpreter::interp, parse::program_parser::parse, type_checker::type_check};

pub fn run(path: &Path) -> io::Result<()> {
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
            println!("parsed_prog = {:#?}", &parsed_prog);
            match type_check(parsed_prog.clone()) {
                Ok(_) => match interp(parsed_prog) {
                    Some(res) => io::stdout().write_fmt(format_args!("{:?}\n", res)),
                    None => Ok(()),
                },
                Err(e) => {
                    for error in e.errors.iter() {
                        io::stderr().write_fmt(format_args!("Error: {}\n", error))?
                    }
                    io::stderr().write_fmt(format_args!("Could not compile '{}'\n", display))
                }
            }
        }
        Err(e) => panic!("Error while parsing program: {:?}", e),
    }
}
