use std::collections::HashMap;

use crate::parser::ast::*;
use crate::parser::ast::position::Position;

mod type_node;
use type_node::*;

pub fn type_error_check(mut prog: Box<Program>) -> Result<(), ()> {
    let mut analyzer: TypeAnalyzer = TypeAnalyzer {
        functions: HashMap::new(),
        classes: HashMap::new(),
		error: false
    };

    prog.analyze_type(&mut analyzer);

    if analyzer.error {
        eprintln!("Type Analysis Failed");
    } 
    
	Ok(())
}

pub struct TypeAnalyzer {
	functions: HashMap<String, FunctionKind>,
	classes: HashMap<String, Vec<Type>>,
	error: bool
}

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

	fn report_error(&mut self, err: ErrorType, position: Position) {
		self.error = true;
		use self::ErrorType::*;

		match err {
			GiveFn 			=> eprintln!("FATAL {position}: Attempt to output a function"),
			GiveClass 		=> eprintln!("FATAL {position}: Attempt to output a class"), 
			GiveVoid 		=> eprintln!("FATAL {position}: Attempt to output void"), 
			ReadFn 			=> eprintln!("FATAL {position}: Attempt to assign user input to function"), 
			ReadClass		=> eprintln!("FATAL {position}: Attempt to assign user input to class"), 
			CallNonFn 		=> eprintln!("FATAL {position}: Attempt to call a non-function"), 
			FnWrongArgNum 	=> eprintln!("FATAL {position}: Function call with wrong number of args"), 
			FnWrongArgType 	=> eprintln!("FATAL {position}: Type of actual does not match type of formal"), 
			ReturnMissing	=> eprintln!("FATAL {position}: Missing return value "), 
			ReturnBad 		=> eprintln!("FATAL {position}: Return with a value in void function"), 
			ReturnVoid 	  	=> eprintln!("FATAL {position}: Bad return value"), 
			WrongOpMath  	=> eprintln!("FATAL {position}: Arithmetic operator applied to invalid operand"), 
			WrongOpCmp   	=> eprintln!("FATAL {position}: Relational operator applied to non-numeric operand"), 
			WrongOpLogic	=> eprintln!("FATAL {position}: Logical operator applied to non-bool operand"), 
			CondNonBool		=> eprintln!("FATAL {position}: Non-bool expression used as a condition"), 
			BadEquality 	=> eprintln!("FATAL {position}: Invalid equality operand"), 
			BadAssign 		=> eprintln!("FATAL {position}: Invalid assignment operand"), 
			NonLval 		=> eprintln!("FATAL {position}: Non-Lval assignment"), 
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