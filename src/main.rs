#[macro_use]
extern crate lalrpop_util;

mod ast;
mod context;
mod function;
mod interpreter;
mod llvm;
mod operators;
mod parse;
mod program;
mod scope;
mod type_checker;
mod type_errors;
mod types;
mod value;
mod variable;

use std::path::Path;

#[allow(unused)]
fn main() {
    // run with false = interp and true = compile (llvm)
    program::run(Path::new("input.rs"), true);
    // llvm::main();
}
