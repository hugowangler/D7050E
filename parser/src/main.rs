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
    statement_parser
};

fn main() {
    //println!("{:#?}", &content_parser::parse("let a : bool = (c == b)"));
    //println!("{:#?}", &relexpr_parser::parse("a || b"));
    //println!("{:#?}", &logexpr_parser::parse("a || c > b"));
    //println!("{:#?}", &keyword_parser::parse("let b : bool = a + 5 > b && c"));
    println!("{:#?}", &statement_parser::
        parse(
            "fn main(x: i32, y: i32) -> i32 {
            while (done) {
                    let x: i32 = x + 1;
                    
                    while (x > 5) {
                        let y: i32 = 12 - x;
                        x = x - 1;
                        continue;
                    }

                    if (x == 5) {
                        let done: bool = false;
                        break;
                    }
                }
            }"
        )
    );
}