use std::collections::HashMap;

use crate::parser::ast::*;
use crate::parser::ast::span::Span;

mod type_node;
use type_node::*;

pub fn type_error_check(mut prog: Box<Program>)  {
    let mut analyzer: TypeAnalyzer = TypeAnalyzer {
        functions: HashMap::new(),
        classes: HashMap::new(),
		error: false
    };

    prog.analyze_type(&mut analyzer);
    if analyzer.error {
        eprintln!("Type Analysis Failed");
    } 
    
}

#[derive(Clone)]
pub struct TypeAnalyzer {
	functions: HashMap<String, FunctionKind>,
	classes: HashMap<String, Vec<Type>>,
	error: bool
}

#[derive(Clone)]
pub struct FunctionKind {
	arg_types: Vec<Type>,
	return_type: Type,
}

impl TypeAnalyzer {
	pub fn add_fn(&mut self, func: &mut FnDecl) {
		self.add_fn_helper(func, func.id.to_string());
	}

	fn add_class_fn(&mut self, class_name: String, func: &mut FnDecl) {
		self.add_fn_helper(func, format!("{}--{}", class_name, func.id.to_string()));
	}

	fn add_fn_helper(&mut self, func: &mut FnDecl, id: String) {
		let mut arg_types: Vec<Type> = Vec::new();
		for arg in func.args.iter() {
			use crate::parser::ast::FormalDecl::*;
			let arg = &**arg;
			match arg {
				VarDecl(x) => arg_types.push(*x.var_type.clone()),
				FormalDecl{id: _, formal_type} => arg_types.push(*formal_type.clone())
			}
		}

		let value = FunctionKind {
			arg_types,
			return_type: *func.ret.clone()
		};

		self.functions.insert(id, value);
	}

	pub fn add_class(&mut self, class: &mut ClassDecl) {
		let mut field_types: Vec<Type> = Vec::new();
		for field in class.member_f.iter() {
			use crate::parser::ast::Decl::*;
			let field = *field.clone();
			match field {
				VarDecl(x) => field_types.push(*x.var_type.clone()),
				FnDecl(mut x) => self.add_class_fn(class.id.to_string(), &mut x),
				_ => ()
			}
		}
		self.classes.insert(class.id.to_string(), field_types);
	}

	pub fn get_fn(&self, fn_name: &String) -> Result<&FunctionKind, ()> {
		match self.functions.get(fn_name) {
			Some(x) => Ok(&x),
			None => Err(())
		}
	}

	pub fn get_fn_return_type(&self, fn_name: &String) -> Result<&Type, ()> {
		match self.functions.get(fn_name) {
			Some(x) => Ok(&x.return_type),
			None => Err(())
		}
	}

	pub fn get_fn_arg_types(&self, fn_name: &String) -> Result<&Vec<Type>, ()> {
		match self.functions.get(fn_name) {
			Some(x) => Ok(&x.arg_types),
			None => Err(())
		}
	}

	pub fn has_fn(&self, name: &String) -> bool {
		if self.functions.get(name).is_some()  {
			true
		} else {
			false
		}
	}

	pub fn has_class(&self, name: &String) -> bool {
		if self.classes.get(name).is_some() {
			true
		} else {
			false
		}
	}

	fn report_error(&mut self, err: &ErrorType, span: &Span) {
		self.error = true;
		use self::ErrorType::*;

		match err {
			GiveFn 			=> eprintln!("FATAL {span}: Attempt to output a function"),
			GiveClass 		=> eprintln!("FATAL {span}: Attempt to output a class"), 
			GiveVoid 		=> eprintln!("FATAL {span}: Attempt to output void"), 
			ReadFn 			=> eprintln!("FATAL {span}: Attempt to assign user input to function"), 
			ReadClass		=> eprintln!("FATAL {span}: Attempt to assign user input to class"), 
			CallNonFn 		=> eprintln!("FATAL {span}: Attempt to call a non-function"), 
			FnWrongArgNum 	=> eprintln!("FATAL {span}: Function call with wrong number of args"), 
			FnWrongArgType 	=> eprintln!("FATAL {span}: Type of actual does not match type of formal"), 
			ReturnMissing	=> eprintln!("FATAL {span}: Missing return value "), 
			ReturnBad 		=> eprintln!("FATAL {span}: Return with a value in void function"), 
			ReturnVoid 	  	=> eprintln!("FATAL {span}: Bad return value"), 
			WrongOpMath  	=> eprintln!("FATAL {span}: Arithmetic operator applied to invalid operand"), 
			WrongOpCmp   	=> eprintln!("FATAL {span}: Relational operator applied to non-numeric operand"), 
			WrongOpLogic	=> eprintln!("FATAL {span}: Logical operator applied to non-bool operand"), 
			CondNonBool		=> eprintln!("FATAL {span}: Non-bool expression used as a condition"), 
			BadEquality 	=> eprintln!("FATAL {span}: Invalid equality operand"), 
			BadAssign 		=> eprintln!("FATAL {span}: Invalid assignment operand"), 
			NonLval 		=> eprintln!("FATAL {span}: Non-Lval assignment"), 
		}
	}
}

enum ErrorType {
	GiveFn,
	GiveClass,
	GiveVoid,
	ReadFn,
	ReadClass,
	CallNonFn,
	FnWrongArgNum,
	FnWrongArgType,
	ReturnMissing,
	ReturnBad,
	ReturnVoid,
	WrongOpMath,
	WrongOpCmp,
	WrongOpLogic,
	CondNonBool,
	BadEquality,
	BadAssign,
	NonLval
}