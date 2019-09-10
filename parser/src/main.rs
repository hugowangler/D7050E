#[macro_use] extern crate lalrpop_util;
mod parse;
mod ast;

#[allow(unused_imports)]
use parse::{
    expr_parser,
    let_parser
};

fn main() {
    println!("Hello, world!");
}
