use std::{collections::HashMap, error::Error};

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    execution_engine::JitFunction,
    module::Module,
    types::BasicTypeEnum,
    values::{BasicValueEnum, FunctionValue, IntValue, PointerValue, InstructionValue},
    IntPredicate, OptimizationLevel,
};

use crate::{ast::Node, operators::Opcode, parse::program_parser, types::LiteralType};

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

type MainFn = unsafe extern "C" fn() -> i32;

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = program_parser::parse(
        "
		fn main() -> bool {
			let a: bool = false;

			if (a && true) {
				return true;
			} else if (a == 10) {
				return true;
			} else {
				return a;
			}
			return true;
		}
		"
        .to_string(),
    )
    .unwrap();
    println!("ast = {:?}", &input);
    let mut compiler = Compiler::new();
    let execution_engine = compiler
        .module
        .create_jit_execution_engine(OptimizationLevel::None)?;

    compiler.compile_program(&input);
    compiler.module.print_to_stderr();

    let res: JitFunction<MainFn> = unsafe { execution_engine.get_function("main").ok().unwrap() };
    unsafe {
        println!("exection result = {}", res.call());
    }

    // function.verify(true);

    Ok(())
}

pub struct Compiler {
    context: Context,
    builder: Builder,
    module: Module,
    scopes: Vec<HashMap<String, PointerValue>>,
    curr_fn: Option<FunctionValue>,
}

/// The compiler assumes that it compiles programs which have been type checked and
/// should therefore not contain any errors
impl Compiler {
    pub fn new() -> Self {
        let context = Context::create();
        Compiler {
            builder: context.create_builder(),
            module: context.create_module("program"),
            context: context,
            scopes: vec![],
            curr_fn: None,
        }
    }

    /// Compiles a parsed program and returns the resulting JitFunction<MainFn>
    /// which can den be called to execute the program
    pub fn compile(&mut self, program: &Vec<Box<Node>>) -> Option<JitFunction<MainFn>> {
        let execution_engine = self
            .module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        self.compile_program(program);
        self.module.print_to_stderr(); // LLVM IR

        let res: Option<JitFunction<MainFn>> =
            unsafe { execution_engine.get_function("main").ok() };
        res
    }

    /// Gets the function value of the function which is currently being compiled
    fn fn_value(&self) -> FunctionValue {
        match self.curr_fn {
            Some(function) => function,
            None => panic!("No current function set"),
        }
	}
	
	/// Gets a variable from the vector of scopes by searching in reverse order
	/// (allows for shadowing)
	fn get_variable(&self, id: &str) -> PointerValue {
		for scope in self.scopes.iter().rev() {
			match scope.get(id) {
				Some(ptr) => return *ptr,
				None => ()
			};
		}
		unreachable!()
	}

    /// Creates a new stack allocation instruction in the entry block of the function
    fn create_entry_block_alloca(&mut self, name: &str, block: &BasicBlock) -> PointerValue {
        let builder = self.context.create_builder();

        match block.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(&block),
        }

        let alloca = builder.build_alloca(self.context.i32_type(), name);
        self.scopes.last_mut().unwrap().insert(name.to_string(), alloca);
        alloca
    }

    /// Compiles a program by declaring its functions and compiling them
    fn compile_program(&mut self, program: &Vec<Box<Node>>) {
        let mut funcs: HashMap<&str, (&Option<LiteralType>, &Box<Node>)> = HashMap::new();

        // Create all of the functions in program
        for func in program.iter() {
            let (name, params, r_type, body) = match &**func {
                Node::Func {
                    name,
                    params,
                    r_type,
                    body,
                } => (name, params, r_type, body),
                _ => unreachable!(),
            };

            // Get the param types and names
            let mut param_types: Vec<BasicTypeEnum> = vec![];
            let mut param_names: Vec<&str> = vec![];
            for param in params.iter() {
                match **param {
                    Node::FuncParam(ref param, param_type, _) => {
                        match **param {
                            Node::Var(ref name) => param_names.push(name),
                            _ => unreachable!(),
                        }

                        match param_type {
                            LiteralType::I32 => param_types.push(self.context.i32_type().into()),
                            LiteralType::Bool => param_types.push(self.context.i32_type().into()),
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }
            }

            // Create the function type
            let fn_type = if let Some(typ) = r_type {
                match typ {
                    LiteralType::I32 => {
                        let i32_type = self.context.i32_type();
                        i32_type.fn_type(&param_types, false)
                    }
                    LiteralType::Bool => {
                        let bool_type = self.context.bool_type();
                        bool_type.fn_type(&param_types, false)
                    }
                    _ => unreachable!(),
                }
            } else {
                let void_type = self.context.void_type();
                void_type.fn_type(&param_types, false)
            };

            // Store function bodies and return type for compiling specific functions
            funcs.insert(name, (&r_type, body));

            let new_func = self.module.add_function(name, fn_type, None);

            // Set param names
            for (param, name) in new_func.get_param_iter().zip(param_names.iter()) {
                param.into_int_value().set_name(name);
            }

            self.context.append_basic_block(&new_func, "entry");
        }

        // Compile the functions
        for (name, (r_type, body)) in funcs.iter() {
            let func = self.module.get_function(name).unwrap();
            self.compile_fn(func, r_type, body);
        }
    }

    fn compile_fn(&mut self, func: FunctionValue, r_type: &Option<LiteralType>, body: &Box<Node>) {
		self.curr_fn = Some(func);

		// New scope for function
		self.scopes.push(HashMap::new());

        let block = &func.get_first_basic_block().unwrap();
        // allocate parameters
        for param in func.get_param_iter() {
            let name = param
                .into_int_value()
                .get_name()
                .to_string_lossy()
                .into_owned();
            let alloca = self.create_entry_block_alloca(&name, &block);
            self.builder.position_at_end(&block);
            self.builder.build_store(alloca, param);
            self.scopes.last_mut().unwrap().insert(name, alloca);
        }

        self.builder.position_at_end(&block);

        // compile body
        self.compile_block(body, &block);

        // void functions still needs to return
        if let None = r_type {
            self.builder
                .position_at_end(&func.get_first_basic_block().unwrap());
            self.builder.build_return(None);
		}
		
		// Done with func so pop scope
		self.scopes.pop();
    }

    /// Compiles all of the statements in a block
    fn compile_block(&mut self, statement: &Box<Node>, block: &BasicBlock) {
		self.scopes.push(HashMap::new());
        let mut next_statement = Some(statement.clone());

        // While the current statement contains a next statement compile it
        while let Some(_) = next_statement {
            self.compile_stmnt(&next_statement.clone().unwrap(), block);
            next_statement = extract_next!(next_statement);
		}
		self.scopes.pop();
    }

    /// Compiles a statement and returns the instruction value along with a bool which indactes
    /// if the statement was a return statement
    fn compile_stmnt(&mut self, statement: &Box<Node>, block: &BasicBlock) {
        match *statement.clone() {
            Node::Let { var, expr, .. } => {
                // Get variable identifier
                let id = match *var {
                    Node::VarBinding(var, _, _) => match *var {
                        Node::Var(id) => id,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };
                let expr_val = self.compile_expr(&expr);

                // Allocate local variable on stack
                let alloca = self.create_entry_block_alloca(&id, block);
                self.builder.build_store(alloca, expr_val);
            }

            Node::VarValue { var, expr, .. } => {
                // update var
                let id = match *var {
                    Node::Var(id) => id,
                    _ => unreachable!(),
                };
                let expr_val = self.compile_expr(&expr);

                // Get the variables pointer value and store new value
				let var = self.get_variable(&id);
                self.builder.build_store(var, expr_val);
            }

            Node::Return { expr, .. } => {
                let ret_val = self.compile_expr(&expr);
                self.builder.build_return(Some(&ret_val));
            }

            Node::If {
                cond, statement, ..
            } => self.compile_if(&cond, &statement),

            Node::IfElse {
                cond,
                if_statement,
                else_statement,
                ..
            } => self.compile_if_else(&cond, &if_statement, &else_statement),

            Node::While {
                cond, statement, ..
			} => self.compile_while(&cond, &statement),
			
			Node::FuncCall { name, args, .. } => {self.compile_call(&name, &args);},

            _ => unimplemented!("compile_stmnt: Node {:?}", statement),
        }
    }

    fn compile_call(&mut self, name: &str, args: &Vec<Box<Node>>) -> IntValue {
        let mut compiled_args: Vec<IntValue> = vec![];

        // compile each argument
        for arg in args.iter() {
            compiled_args.push(self.compile_expr(arg));
        }

        let args_val: Vec<BasicValueEnum> = compiled_args
            .iter()
            .by_ref()
            .map(|&val| val.into())
            .collect();

        self.builder
            .build_call(
                self.module.get_function(name).unwrap(),
                args_val.as_slice(),
                "tmp",
            )
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_int_value()
    }

    fn compile_while(&mut self, cond: &Box<Node>, statement: &Box<Node>) {
		let func = self.fn_value();

        // build branches
        let cond_bb = self.context.append_basic_block(&func, "cond");
        let do_bb = self.context.append_basic_block(&func, "do");
        let cont_bb = self.context.append_basic_block(&func, "whilecont");

        self.builder.build_unconditional_branch(&cond_bb);

        // build cond block
        self.builder.position_at_end(&cond_bb);
		let cond_res = self.compile_expr(cond);
        self.builder
            .build_conditional_branch(cond_res, &do_bb, &cont_bb);

        // build do block
        self.builder.position_at_end(&do_bb);
        self.compile_block(statement, &do_bb);

        // continue while loop
        self.builder.build_unconditional_branch(&cond_bb);

        // merge
        self.builder.position_at_end(&cont_bb);
        let phi = self.builder.build_phi(self.context.i32_type(), "whiletmp");

        let some_num = self.context.i32_type().const_int(2, false);
        phi.add_incoming(&[(&some_num, &do_bb), (&some_num, &cont_bb)])
    }

    /// Compiles if statements with else and/or elseif
    fn compile_if_else(&mut self, cond: &Box<Node>, if_stmnt: &Box<Node>, else_stmnt: &Box<Node>) {
		let func = self.fn_value();
		
		// create compare that will be used as cond
		let cond = self.compile_expr(cond);
		// let zero_const = self.context.i32_type().const_int(0, false);
		// let cond = self.builder.build_int_compare(IntPredicate::NE, cond, zero_const, "ifcond");

        // build branches
        let then_bb = self.context.append_basic_block(&func, "then");
        let else_bb = self.context.append_basic_block(&func, "else");
        let cont_bb = self.context.append_basic_block(&func, "ifcont");

        self.builder
            .build_conditional_branch(cond, &then_bb, &else_bb);

        // build then block
        self.builder.position_at_end(&then_bb);
        self.compile_block(if_stmnt, &then_bb);
        self.builder.build_unconditional_branch(&cont_bb);

        // build else block
        self.builder.position_at_end(&else_bb);
        self.compile_block(else_stmnt, &else_bb);
        self.builder.build_unconditional_branch(&cont_bb);

        // merge
        self.builder.position_at_end(&cont_bb);
        let phi = self.builder.build_phi(self.context.i32_type(), "iftmp");

        let some_num = self.context.i32_type().const_int(2, false);
        phi.add_incoming(&[(&some_num, &then_bb), (&some_num, &else_bb)]);
    }

    /// Compiles plain if statements
    fn compile_if(&mut self, cond: &Box<Node>, statement: &Box<Node>) {
		let func = self.fn_value();
        let cond = self.compile_expr(cond);

        // build then and continue branch
        let then_bb = self.context.append_basic_block(&func, "then");
        let cont_bb = self.context.append_basic_block(&func, "ifcont");

        self.builder
            .build_conditional_branch(cond, &then_bb, &cont_bb);

        // build then block
        self.builder.position_at_end(&then_bb);
        self.compile_block(statement, &then_bb);
        self.builder.build_unconditional_branch(&cont_bb);

        // merge
        self.builder.position_at_end(&cont_bb);
        let phi = self.builder.build_phi(self.context.i32_type(), "iftmp");

        let some_num = self.context.i32_type().const_int(2, false);
        phi.add_incoming(&[(&some_num, &then_bb), (&some_num, &cont_bb)]);
    }

    fn compile_expr(&mut self, expr: &Box<Node>) -> IntValue {
        match &**expr {
            Node::Number(num) => self.context.i32_type().const_int(*num as u64, false),

            Node::Bool(b) => match b {
                true => self.context.i32_type().const_int(1, false),
                false => self.context.i32_type().const_int(0, false),
            },

            Node::UnaryOp(op, expr) => {
                let value = self.compile_expr(&expr);
                match op {
                    Opcode::Sub => self.builder.build_int_neg(value, "neg"),
                    _ => unreachable!(),
                }
            }

            Node::Var(id) => {
				let var = self.get_variable(&id);
                self.builder.build_load(var, &id).into_int_value()
            }

            Node::FuncCall { name, args, .. } => self.compile_call(&name, &args),

            Node::Expr(left, op, right) => {
                let l_val = self.compile_expr(&left);
                let r_val = self.compile_expr(&right);
                match op {
                    Opcode::Add => self.builder.build_int_add(l_val, r_val, "add"),
                    Opcode::Sub => self.builder.build_int_sub(l_val, r_val, "sub"),
                    Opcode::Mul => self.builder.build_int_mul(l_val, r_val, "mul"),
                    Opcode::Div => self.builder.build_int_signed_div(l_val, r_val, "div"),
                    Opcode::AND => self.builder.build_and(l_val, r_val, "and"),
                    Opcode::OR => self.builder.build_or(l_val, r_val, "or"),
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

// TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::program_parser::parse;

    #[test]
    fn variable_add() {
        let input = parse(
            "fn main() -> i32 {
				let a: i32 = 10;
				let b: i32 = 10 + a;
				return b;
			}"
            .to_string(),
        )
        .unwrap();

        let mut compiler = Compiler::new();
        let res = compiler.compile(&input).unwrap();
        assert_eq!(unsafe { res.call() }, 20);
	}
	
	#[test]
    fn bool_and() {
        let input = parse(
            "fn main() -> i32 {
				return true && false;
			}"
            .to_string(),
        )
        .unwrap();

        let mut compiler = Compiler::new();
        let res = compiler.compile(&input).unwrap();
        assert_eq!(unsafe { res.call() }, 0);
    }

    #[test]
    fn variable_bool() {
        let input = parse(
            "fn main() -> bool {
				let a: bool = false && true;
				let b: bool = a || false;
				return b;
			}"
            .to_string(),
        )
        .unwrap();

        let mut compiler = Compiler::new();
        let res = compiler.compile(&input).unwrap();
        assert!(unsafe { res.call() } == 0);
    }

    #[test]
    fn variable_update() {
        let input = parse(
            "fn main() -> i32 {
				let a: i32 = 10;
				let b: i32 = 10 + a;
				a = a + b;
				return a;
			}"
            .to_string(),
        )
        .unwrap();

        let mut compiler = Compiler::new();
        let res = compiler.compile(&input).unwrap();
        assert_eq!(unsafe { res.call() }, 30);
    }

    #[test]
    fn precedence_num() {
        let input = parse(
            "fn main() -> i32 {
				return 2 * 10 - 3 + 2 * 5;
			}"
            .to_string(),
        )
        .unwrap();

        let mut compiler = Compiler::new();
        let res = compiler.compile(&input).unwrap();
        assert_eq!(unsafe { res.call() }, 27);
    }

    #[test]
    fn test_if() {
        let input = parse(
            "fn main() -> i32 {
				let a: bool = true;

				if (a) {
					return 1;
				}

				return 0;
			}"
            .to_string(),
        )
        .unwrap();

        let mut compiler = Compiler::new();
        let res = compiler.compile(&input).unwrap();
        assert_eq!(unsafe { res.call() }, 1);
    }

    #[test]
    fn test_if_else() {
        let input = parse(
            "fn main() -> i32 {
				let a: bool = true;

				if (a == false) {
					return 1;
				} else {
					return 2;
				}
				return 3;
			}"
            .to_string(),
        )
        .unwrap();

        let mut compiler = Compiler::new();
        let res = compiler.compile(&input).unwrap();
        assert_eq!(unsafe { res.call() }, 2);
    }

    #[test]
    fn test_elseif() {
        let input = parse(
            "fn main() -> i32 {
				let a: bool = false;

				if (a == true) {
					return 1;
				} else if (a || true) {
					return 2;
				}
				return 3;
			}"
            .to_string(),
        )
        .unwrap();

        let mut compiler = Compiler::new();
        let res = compiler.compile(&input).unwrap();
        assert_eq!(unsafe { res.call() }, 2);
    }

    #[test]
    fn test_elseif_else() {
        let input = parse(
            "fn main() -> i32 {
				let a: bool = false;

				if (a && true) {
					return 1;
				} else if (a == true) {
					return 2;
				} else {
					return 3;
				}
				return 4;
			}"
            .to_string(),
        )
        .unwrap();

        let mut compiler = Compiler::new();
        let res = compiler.compile(&input).unwrap();
        assert_eq!(unsafe { res.call() }, 3);
    }

    #[test]
    fn test_while() {
        let input = parse(
            "fn main() -> i32 {
				let num: i32 = 0;

				while (num < 10) {
					num = num + 1;
				}
				return num;
			}"
            .to_string(),
        )
        .unwrap();

        let mut compiler = Compiler::new();
        let res = compiler.compile(&input).unwrap();
        assert_eq!(unsafe { res.call() }, 10);
    }

    #[test]
    fn test_fib() {
    	let input = parse(
    		"fn main() -> i32 {
    			return fib(20);
    		}

    		fn fib(n: i32) -> i32 {
    			if (n <= 1) {
    				return n;
				}
				
    			return fib(n - 2) + fib(n - 1);
    		}

    		".to_string()
    	).unwrap();

    	let mut compiler = Compiler::new();
    	let res = compiler.compile(&input).unwrap();
    	assert_eq!(unsafe{res.call()}, 6765);
	}

	#[test]
    fn test_shadowing() {
    	let input = parse(
    		"fn main() -> i32 {
				let a: i32 = 20;
				
				if (a == 20) {
					let a: i32 = 1000;
				}
				return a;
    		}
    		".to_string()
    	).unwrap();

    	let mut compiler = Compiler::new();
    	let res = compiler.compile(&input).unwrap();
    	assert_eq!(unsafe{res.call()}, 20);
	}

	#[test]
    fn test_shadowing_use_shadow() {
    	let input = parse(
    		"fn main() -> i32 {
				let a: i32 = 20;
				
				if (a == 20) {
					let a: i32 = 1000;
					if (a == 1000) {
						return a;
					}
				}
				return a;
    		}
    		".to_string()
    	).unwrap();

    	let mut compiler = Compiler::new();
    	let res = compiler.compile(&input).unwrap();
    	assert_eq!(unsafe{res.call()}, 1000);
	}

	#[test]
    fn test_func_param() {
    	let input = parse(
    		"fn main() -> i32 {
				return test(5, false);
			}
			
			fn test(a: i32, b: bool) -> i32 {
				if ((a == 5) && (b == false)) {
					return 1;
				} else {
					return 2;
				}
				return 3;
			}
    		".to_string()
    	).unwrap();

    	let mut compiler = Compiler::new();
    	let res = compiler.compile(&input).unwrap();
    	assert_eq!(unsafe{res.call()}, 1);
	}
	

}
