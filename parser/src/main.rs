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
    statement_parser,
    program_parser
};

fn main() {
    //println!("{:#?}", &content_parser::parse("let a : bool = (c == b)"));
    //println!("{:#?}", &relexpr_parser::parse("a || b"));
    //println!("{:#?}", &logexpr_parser::parse("a || c > b"));
    //println!("{:#?}", &keyword_parser::parse("let b : bool = a + 5 > b && c"));
    println!("{:#?}", &program_parser::
        parse("
            fn main() {
                sum(1, 2);
            }
            
            fn sum(x: i32, y: i32) -> i32 {
                let sum: i32 = x + y;
                return sum;
            }
  
        ")
    );
}