lalrpop_mod!(pub grammar); // synthesized by LALRPOP

pub mod expr_parser;
pub mod keyword_parser;
pub mod logexpr_parser;
pub mod program_parser;
pub mod relexpr_parser;
pub mod statement_parser;

#[derive(Debug)]
pub struct ParseError {
    message: String,
}
