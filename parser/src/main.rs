#[macro_use] extern crate lalrpop_util;
mod parse;
mod ast;
mod types;
mod operators;

#[allow(unused_imports)]
use parse::{
    keyword_parser,
    expr_parser,
    relexpr_parser,
    logexpr_parser,
    content_parser
};

fn main() {
    //println!("{:#?}", &content_parser::parse("let a : bool = (c == b)"));
    //println!("{:#?}", &relexpr_parser::parse("a == b != c == false"));
    //println!("{:#?}", &logexpr_parser::parse("a || c && b"));
    println!("{:#?}", &content_parser::parse("let a: i32 = 12; let b: i32 = 24;"));
    //println!("{:#?}", &keyword_parser::parse("let b : bool = a + 5 > b && c"));
}