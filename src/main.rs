#[macro_use]
extern crate lalrpop_util;

mod parse;
mod ast;
mod types;
mod operators;

use std::{
    fs::File,
    path::Path,
    io::prelude::*,
    error::Error
};

#[allow(unused_imports)]
use parse::{
    keyword_parser,
    expr_parser,
    relexpr_parser,
    logexpr_parser,
    statement_parser,
    program_parser
};

fn main() {
    let path = Path::new("input.rs");
    let display = path.display();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Could not open {}: {}", display, e.description())
    };

    let mut input = String::new();
    
    match file.read_to_string(&mut input) {
        Ok(_) => (),
        Err(e) => panic!("Could not read file: {:?}", e)
    }

    let program = program_parser::parse(input).unwrap();
    println!("program = {:#?}", program);
}