use std::{collections::HashMap, error::Error};

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
    module::Module,
    passes::PassManager,
    types::BasicTypeEnum,
    values::{BasicValueEnum, FloatValue, FunctionValue, InstructionValue, IntValue, PointerValue},
    IntPredicate, OptimizationLevel,
};

use crate::{ast::Node, operators::Opcode, parse::statement_parser};

macro_rules! extract_next {
    ($statement:tt) => {
        match *$statement.unwrap() {
            Node::VarValue {
                var: _,
                expr: _,
                next,
            } => next,
            Node::Let {
                var: _,
                expr: _,
                next,
            } => next,
            Node::If {
                cond: _,
                statement: _,
                next,
            } => next,
            Node::IfElse {
                cond: _,
                if_statement: _,
                else_statement: _,
                next,
            } => next,
            Node::While {
                cond: _,
                statement: _,
                next,
            } => next,
            Node::FuncCall {
                name: _,
                args: _,
                next,
            } => next,
            _ => None,
        }
    };
}

type ExprFunc = unsafe extern "C" fn() -> i32;

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = statement_parser::parse(
		"
		let x: bool = true;
		return true;
		",
    )
    .unwrap();
    println!("ast = {:?}", &input);
    let mut compiler = Compiler::new();
    let execution_engine = compiler
        .module
        .create_jit_execution_engine(OptimizationLevel::None)?;

    let u32_type = compiler.context.i32_type();
    let fn_type = u32_type.fn_type(&[], false);
    let function = compiler.module.add_function("main", fn_type, None);
    let basic_block = compiler.context.append_basic_block(&function, "entry");
    compiler.builder.position_at_end(&basic_block);

    compiler.compile_block(input, &basic_block);
    compiler.module.print_to_stderr();

    let fun_expr: JitFunction<ExprFunc> =
        unsafe { execution_engine.get_function("main").ok().unwrap() };
    unsafe {
        println!("\nexection result = {}", fun_expr.call());
	}
	
	function.verify(true);

    Ok(())
}

pub struct Compiler {
    pub context: Context,
    pub builder: Builder,
    pub module: Module,
    variables: HashMap<String, PointerValue>,
}

/// The compiler assumes that it compiles programs which have been type checked and
/// should therefore not contain any errors
impl Compiler {
    fn new() -> Self {
        let context = Context::create();
        Compiler {
            builder: context.create_builder(),
            module: context.create_module("main"),
            context: context,
            variables: HashMap::new(),
        }
    }

    /// Creates a new stack allocation instruction in the entry block of the function
    fn create_entry_block_alloca(&mut self, name: &str, block: &BasicBlock) -> PointerValue {
        let builder = self.context.create_builder();

        match block.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(&block),
        }

        let alloca = builder.build_alloca(self.context.i32_type(), name);
        self.variables.insert(name.to_string(), alloca);
        alloca
    }

	/// Compiles all of the statements in a block
    fn compile_block(&mut self, statement: Box<Node>, block: &BasicBlock) {
		let mut next_statement = Some(statement);
		
		// While the current statement contains a next statement compile it
        while match next_statement {
            Some(_) => true,
            None => false,
        } {
            self.compile_stmnt(next_statement.clone().unwrap(), block);
            next_statement = extract_next!(next_statement);
        }
    }

    fn compile_stmnt(&mut self, statement: Box<Node>, block: &BasicBlock) {
        match *statement {
            Node::Let { var, expr, .. } => {
                // Get variable identifier
                let id = match *var {
                    Node::VarBinding(var, _, _) => match *var {
                        Node::Var(id) => id,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };
				let expr_val = self.compile_expr(expr);
				
                // Allocate local variable on stack
                let alloca = self.create_entry_block_alloca(&id, block);
                self.builder.build_store(alloca, expr_val);
            }
            Node::VarValue { var, expr, .. } => {	// update var
				let id = match *var {
					Node::Var(id) => id,
					_ => unreachable!()
				};
				let expr_val = self.compile_expr(expr);

				// Get the variables pointer value and store new value
				let var = self.variables.get(&id).unwrap();
				self.builder.build_store(*var, expr_val);
			}
            Node::Return(expr) => {
                let ret_val = self.compile_expr(expr);
                self.builder.build_return(Some(&ret_val));
            }
            _ => unimplemented!("compile_stmnt: Node {:?}", statement),
        }
    }

    fn compile_expr(&self, expr: Box<Node>) -> IntValue {
        match *expr {
            Node::Number(num) => self.context.i32_type().const_int(num as u64, false),
            Node::Bool(b) => match b {
                true => self.context.bool_type().const_int(1, false),
                false => self.context.bool_type().const_int(0, false),
            },
            Node::UnaryOp(op, expr) => {
                let value = self.compile_expr(expr);
                match op {
                    Opcode::Sub => self.builder.build_int_neg(value, "neg"),
                    _ => unreachable!(),
                }
            }
            Node::Var(id) => {
				let var = self.variables.get(&id).unwrap();
				self.builder.build_load(*var, &id).into_int_value()
			},
            Node::Expr(left, op, right) => {
                let l_val = self.compile_expr(left);
                let r_val = self.compile_expr(right);

                match op {
                    Opcode::Add => self.builder.build_int_add(l_val, r_val, "add"),
                    Opcode::Sub => self.builder.build_int_sub(l_val, r_val, "sub"),
                    Opcode::Mul => self.builder.build_int_mul(l_val, r_val, "mul"),
                    Opcode::Div => self.builder.build_int_signed_div(l_val, r_val, "div"),
                    Opcode::AND => self.builder.build_and(l_val, r_val, "and"),
                    Opcode::OR => self.builder.build_or(l_val, r_val, "and"),
                    Opcode::EQ => {
                        self.builder
                            .build_int_compare(IntPredicate::EQ, l_val, r_val, "eq")
                    }
                    Opcode::NEQ => {
                        self.builder
                            .build_int_compare(IntPredicate::NE, l_val, r_val, "neq")
                    }
                    Opcode::GT => {
                        self.builder
                            .build_int_compare(IntPredicate::SGT, l_val, r_val, "gt")
                    }
                    Opcode::LT => {
                        self.builder
                            .build_int_compare(IntPredicate::SLT, l_val, r_val, "lt")
                    }
                    Opcode::LEQ => {
                        self.builder
                            .build_int_compare(IntPredicate::SLE, l_val, r_val, "leq")
                    }
                    Opcode::GEQ => {
                        self.builder
                            .build_int_compare(IntPredicate::SGE, l_val, r_val, "geq")
                    }
                }
            }
            _ => unimplemented!("Node '{:?}' not supported", *expr),
        }
    }
}
