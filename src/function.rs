use crate::ast::Node;
use crate::types::LiteralType;

#[derive(Clone, Debug, PartialEq)]
pub struct Func {
	pub name: String,
	pub params: Vec<Box<Node>>,
	pub r_type: LiteralType,
	pub body: Box<Node>
}

impl Func {
	pub fn new(name: String, params: Vec<Box<Node>>, r_type: LiteralType, body: Box<Node>) -> Func {
		Func{
			name: name,
			params: params,
			r_type: r_type,
			body: body
		}
	}
}