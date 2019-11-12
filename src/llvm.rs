use std::collections::HashMap;

use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
    module::Module,
    passes::PassManager,
    types::BasicTypeEnum,
    values::{BasicValueEnum, FloatValue, FunctionValue, InstructionValue, IntValue, PointerValue},
    FloatPredicate, OptimizationLevel,
};

use crate::{
	ast::Node,
	parse::expr_parser,
};

pub fn main() {
	let input = expr_parser::parse(
		"
		5
		"
	).unwrap();

	let compiler = Compiler::new();
	compiler.compile_expr(input);
}

pub struct Compiler {
	pub context: Context,
	pub builder: Builder,
	pub module: Module,
}

impl Compiler {
	pub fn new() -> Self {
		let context = Context::create();
		Compiler {
			builder: context.create_builder(),
			module: context.create_module("min"),
			context: context,
		}
	}

	pub fn compile_expr(&self, expr: Box<Node>) -> IntValue {
		match *expr {
			Node::Number(num) => self.context.i32_type().const_int(num as u64, false),
			_ => unimplemented!("Node '{:?}' not supported", *expr)
		}
	}
}