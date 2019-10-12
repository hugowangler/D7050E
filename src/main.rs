#[macro_use]
extern crate lalrpop_util;

mod ast;
mod context;
mod function;
mod interpreter;
mod operators;
mod parse;
mod program;
mod scope;
mod types;
mod value;
mod variable;

use std::path::Path;

fn main() {
    println!("{:?}", program::run(Path::new("input.rs")));
}
