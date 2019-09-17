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
    logexpr_parser
};

fn main() {
    //println!("{:#?}", &keyword_parser::parse("let a : bool = b && c;"));
    //println!("{:#?}", &relexpr_parser::parse("abs"));
    //println!("{:#?}", &logexpr_parser::parse("true"));
    println!("{:#?}", &expr_parser::parse("1-2+3"));
    //println!("{:#?}", &keyword_parser::parse("let a: i32 = -1"));
}