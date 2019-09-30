use crate::types::{
    LiteralType
};

use crate::operators::{
    BinOpcode,
    LogOpcode,
    RelOpcode
};

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
	Number(i32),	// 
	Bool(bool),	//
    _String(String),	//

    Var(String),	//
    VarBinding(Box<Node>, LiteralType),	//
    VarValue(Box<Node>, Box<Node>),	//

    BinOp(Box<Node>, BinOpcode, Box<Node>),	//
    LogOp(Box<Node>, LogOpcode, Box<Node>),	//
	RelOp(Box<Node>, RelOpcode, Box<Node>),	//

    // Keywords
    Let(Box<Node>, Box<Node>),	//
    Print(Box<Node>),	//
    If{cond: Box<Node>, statement: Box<Node>},	//
    IfElse{cond: Box<Node>, if_statement: Box<Node>, else_statement: Box<Node>},	//
    While{cond: Box<Node>, statement: Box<Node>},	//

    Func{name: String, params: Vec<Box<Node>>, r_type: LiteralType, body: Box<Node>},
    FuncParam(Box<Node>, LiteralType),
    FuncCall{name: String, args: Vec<Box<Node>>},

    // Loop modifiers
    Break,
    Continue,

    Return(Box<Node>),

    Statement(Box<Node>, Box<Node>)	//
}