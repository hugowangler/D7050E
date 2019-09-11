#[macro_use] extern crate lalrpop_util;
mod parse;
mod ast;
mod types;

#[allow(unused_imports)]
use parse::{
    expr_parser,
    let_parser
};

fn main() {
    println!("{:#?}", &let_parser::parse("let a : i32 = b + 3;"));
}
