#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP
pub mod ast;

#[test]
fn test_expr() {
    let expr = grammar::ExprParser::new()
        .parse("22 * 44 + 66 - 420")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "(((22 * 44) + 66) - 420)");
    println!("test ={:#?}", expr);
}

#[test]
fn test2_expr() {
    let expr = grammar::ExprParser::new()
        .parse("(22 * 44) + 66")
        .unwrap();
    
    println!("test2 = {:#?}", expr);
}

/*
#[test]
fn test_var() {
    let expr = grammar::VariableParser::new()
        .parse("1337")
        .unwrap();

    println!("expr = {:?}", expr);
    assert_eq!(expr, "a + 2");
} */

#[test]
fn var_expr() {
    let expr = grammar::ExprParser::new()
        .parse("a+2")
        .unwrap();
    assert_eq!(&format!("{:?}",expr), "(a+2)");
    println!("expr = {:#?}", expr);

}

fn main() {
    println!("Hello, world!");
}
