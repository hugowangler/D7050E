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
            "if (b && c) {
                    let xyz: bool = a < b || a + 6;
                } else if (b && d) {
                    if (c == 5) {
                        let a: i32 = 123123 == b;
                    } else {
                        let a: i32 = b && c;
                    }
                    let b: i32 = 15;
                } else if (b) {
                    let c: i32 = 12;
                } else if (c) {
                    let d: i32 = a && b;
                } else {
                    let asd: bool = true;
                }"
        )
    );
}