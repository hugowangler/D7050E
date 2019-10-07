use crate::types::LiteralType;
use crate::operators::{BinOpcode, LogOpcode, RelOpcode};

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
	Number(i32),	// 
	Bool(bool),	//
    _String(String),	//

    Var(String),	//
    VarBinding(Box<Node>, LiteralType),	//

    BinOp(Box<Node>, BinOpcode, Box<Node>),	//
    LogOp(Box<Node>, LogOpcode, Box<Node>),	//
	RelOp(Box<Node>, RelOpcode, Box<Node>),	//

    // Keywords
	// Keywords contain an optional field for the "next" node in the ast which contains other keywords
	VarValue{var: Box<Node>, expr: Box<Node>, next: Option<Box<Node>>},	//
    Let{var: Box<Node>, expr: Box<Node>, next: Option<Box<Node>>},	//
    Print{text: Box<Node>, next: Option<Box<Node>>},	//
    If{cond: Box<Node>, statement: Box<Node>, next: Option<Box<Node>>},	//
    IfElse{cond: Box<Node>, if_statement: Box<Node>, else_statement: Box<Node>, next: Option<Box<Node>>},	//
    While{cond: Box<Node>, statement: Box<Node>, next: Option<Box<Node>>},	//

    Func{name: String, params: Vec<Box<Node>>, r_type: LiteralType, body: Box<Node>}, //
    FuncParam(Box<Node>, LiteralType),	//
    FuncCall{name: String, args: Vec<Box<Node>>, next: Option<Box<Node>>}, //

    // Loop modifiers
    Break,
    Continue,

    Return(Box<Node>),
}

impl Node {
	pub fn insert_next(&mut self, node: Box<Node>) {
		match *self {
			Node::VarValue{var: _, expr: _, ref mut next} => *next = Some(node),
			Node::Let{var: _, expr: _, ref mut next} => *next = Some(node),
			Node::Print{text: _, ref mut next} => *next = Some(node),
			Node::If{cond: _, statement: _, ref mut next} => *next = Some(node),
			Node::IfElse{cond: _, if_statement: _, else_statement: _, ref mut next} => *next = Some(node),
			Node::While{cond: _, statement: _, ref mut next} => *next = Some(node),
			Node::FuncCall{name: _, args: _, ref mut next} => *next = Some(node),
			_ => panic!("Trying to insert next node a node which does not contain have a next node")
		}
	}
}