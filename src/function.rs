use std::collections::HashMap;

use crate::{
	ast::Node,
	types::LiteralType,
	interpreter::visit,
	context::Context,
	scope::Scope,
	value::Value,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Func {
	pub name: String,
	pub params: Vec<Box<Node>>,
	pub r_type: LiteralType,
	pub body: Box<Node>
}

type Funcs = HashMap<String, Func>;	// Stores all the function names and their bodies

impl Func {
	pub fn new(name: String, params: Vec<Box<Node>>, r_type: LiteralType, body: Box<Node>) -> Func {
		Func{
			name: name,
			params: params,
			r_type: r_type,
			body: body
		}
	}

	pub fn call(&mut self, args: Vec<Box<Node>>, context: &mut Context, funcs: &mut Funcs) -> Value {
		self.check_args(&args);
		let mut param_arg = vec![];
		
		for pair in self.params.iter().zip(args.iter()) {
			let (param, arg) = pair;
			param_arg.push(
				(&**param, visit(arg.clone(), context, funcs))
			);
		}
		context.push(Scope::init(param_arg));
		println!("context in call = {:#?}", context);
		visit(self.body.clone(), context, funcs)
	}

	fn check_args(&self, args: &Vec<Box<Node>>) {
		if args.len() != self.params.len() {
			panic!("Invlaid arguments")
		}
	}
}