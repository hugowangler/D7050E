#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP
pub mod ast;

fn main() {
    println!("Hello, world!");
    let expr = grammar::LetParser::new()
        .parse("let a = 2 + 3 - 5;")
        .unwrap();
    //assert_eq!(expr, "(2 + a)");
    println!("expr = {:#?}", expr);
}
