use std::collections::HashMap;

use crate::{
    ast::Node, context::Context, interpreter::visit, scope::Scope, types::LiteralType, value::Value,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Func {
    pub name: String,
    pub params: Vec<Box<Node>>,
    pub r_type: Option<LiteralType>,
    pub body: Box<Node>,
}

type Funcs = HashMap<String, Func>; // Stores all the function names and their bodies

impl Func {
    pub fn new(
        name: String,
        params: Vec<Box<Node>>,
        r_type: Option<LiteralType>,
        body: Box<Node>,
    ) -> Func {
        Func {
            name: name,
            params: params,
            r_type: r_type,
            body: body,
        }
    }

    pub fn get_r_type(&self) -> LiteralType {
        match self.r_type {
            Some(typ) => typ,
            None => LiteralType::Void,
        }
    }

    pub fn get_param_types(&self) -> Vec<(String, LiteralType)> {
        let mut param_types = vec![];
        for param in self.params.iter() {
            match *param.clone() {
                Node::FuncParam(var, typ, _) => match *var {
                    Node::Var(name) => param_types.push((name, typ)),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
        param_types
    }

    pub fn call(
        &mut self,
        args: Vec<Box<Node>>,
        context: &mut Context,
        funcs: &mut Funcs,
    ) -> Option<Value> {
        self.check_args(&args);
        let mut param_arg = vec![];

        // Create a scope containing the parameters with value of the arguments
        for pair in self.params.iter().zip(args.iter()) {
            let (param, arg) = pair;
            param_arg.push((&**param, visit(arg.clone(), context, funcs)));
        }
        context.push(Scope::init(param_arg));

        // Execute the function body and return if the function returns
        match visit(self.body.clone(), context, funcs) {
            Value::Bool(b) => Some(Value::Bool(b)),
            Value::Number(num) => Some(Value::Number(num)),
            Value::None => None,
            _ => panic!("Unkown return type in function \"{:?}\"", self.name),
        }

        // visit(self.body.clone(), context, funcs)
    }

    fn check_args(&self, args: &Vec<Box<Node>>) {
        if args.len() != self.params.len() {
            panic!("Invlaid arguments")
        }
    }
}
