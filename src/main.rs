#[macro_use]
extern crate lalrpop_util;

mod parse; 
mod ast; 
mod types; 
mod operators; 
mod interpreter; 
mod value;
mod context;
mod scope;
mod function;
mod program;

use std::path::Path;

fn main() {
	program::run(Path::new("input.rs"))
}