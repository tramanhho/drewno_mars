use std::collections::HashMap;

use crate::parser::ast::*;
use crate::parser::ast::span::Span;

mod type_node;
use type_node::*;

use std::fmt::{Display, Formatter, Error};

pub fn type_error_check(mut prog: Box<Program>)  {
    let mut analyzer: TypeAnalyzer = TypeAnalyzer {
        functions: HashMap::new(),
        classes: HashMap::new(),
        vars: HashMap::new(),
        scope: 0,
		error: false,
    };

    prog.analyze_type(&mut analyzer);
    if analyzer.error {
        eprintln!("Type Analysis Failed");
    } 
    
}

#[derive(Clone)]
pub struct TypeAnalyzer {
	functions: HashMap<String, FunctionKind>,
	classes: HashMap<String, Vec<Field>>,
    vars: HashMap<VarKey, Type>,
    scope: usize,
	error: bool
}

impl Display for TypeAnalyzer {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let mut output : Vec<String> = Vec::new();
		output.push("\n======================".to_string());
        output.push(format!("Scope: {}", self.scope));
        output.push("\nFunctions:".to_string());
        for (fn_name, kind) in self.functions.iter() {
            output.push(format!("{}: {}", fn_name, kind));
        }
        output.push("\nClasses:".to_string());
        for (class_name, fields) in self.classes.iter() {
            output.push(format!("{}: ", class_name));
            for field in fields.iter() {
                output.push(format!("\t{}", field));
            }
        }
		output.push("\nVariables:".to_string());
		for (key, var_type) in self.vars.iter() {
            output.push(format!("{}: {}", key, var_type));
        }
		output.push("======================\n".to_string());
        write!(fmt, "{}", output.join("\n"))
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct VarKey {
	id: String,
	scope: usize,
}

impl Display for VarKey {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "[{}] {}", self.scope, self.id)
    }
}

#[derive(Clone)]
pub struct FunctionKind {
	arg_types: Vec<Type>,
	return_type: Type,
}

impl Display for FunctionKind {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "({}) -> {}", 
            self.arg_types.iter().map(|arg| arg.to_string()).collect::<Vec::<String>>().join(", "), 
            self.return_type)
    }
}

#[derive(Clone)]
pub struct Field {
	id: String,
    field_type: Type
}

impl Display for Field {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{} {{{}}}", self.id, self.field_type)
    }
}

impl TypeAnalyzer {
    pub fn add_var(&mut self, id: String, var_type: Type) {
		let key = VarKey {
			id,
			scope: self.scope
		};

		self.vars.insert(key, var_type);
	}

    pub fn add_class_inst(&mut self, id: String, class_name: String) {
        let empty_vec = Vec::<Field>::new();

        let field_refs = match self.classes.get(&class_name) {
            Some(field_vec) => field_vec,
            None => &empty_vec
        };

        for field in field_refs.iter() {
            let key = VarKey {
                id: format!("{}--{}", id, field.id),
                scope: self.scope
            };

            self.vars.insert(key, field.field_type.clone());
        }
	}

    pub fn remove_scope(&mut self) {
		self.vars.retain(|k, _| k.scope != self.scope);
	}

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
		let mut fields: Vec<Field> = Vec::new();
		for field in class.member_f.iter() {
			use crate::parser::ast::Decl::*;
			let field = *field.clone();
			match field {
				VarDecl(x) => {
                    let var_field = Field {
                        id: format!("{}--{}", class.id.to_string(), x.id.to_string()),
                        field_type: *x.var_type.clone()
                    };
                    fields.push(var_field)
                },
				FnDecl(mut x) => self.add_class_fn(class.id.to_string(), &mut x),
				_ => ()
			}
		}
		self.classes.insert(class.id.to_string(), fields);
	}

	pub fn get_fn(&self, fn_name: &String) -> Result<&FunctionKind, ()> {
		match self.functions.get(fn_name) {
			Some(x) => Ok(&x),
			None => Err(())
		}
	}

    pub fn get_var_type(&self, id: String) -> Result<Type, ()> {
        let mut key_check: Vec<VarKey> = Vec::new();
        
        for s in (0..=self.scope).rev() {
            key_check.push(VarKey {
                id: id.clone(),
                scope: s
            });
        }

        // println!("{}", &self);
        // println!("{:?}", &key_check);
        for key in key_check.into_iter() {
            // println!("{}", key);
            match self.vars.clone().get(&key) {
                Some(kind) => {
                    return Ok(kind.clone()); 
                },
                None => (),
            };
        }

        Err(())
	}

	pub fn has_fn(&self, name: &String) -> bool {
        // println!();
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
			ReturnVoid 		=> eprintln!("FATAL {span}: Return with a value in void function"), 
			ReturnBad 	  	=> eprintln!("FATAL {span}: Bad return value"), 
			WrongOpMath  	=> eprintln!("FATAL {span}: Arithmetic operator applied to invalid operand"), 
			WrongOpCmp   	=> eprintln!("FATAL {span}: Relational operator applied to non-numeric operand"), 
			WrongOpLogic	=> eprintln!("FATAL {span}: Logical operator applied to non-bool operand"), 
			CondNonBool		=> eprintln!("FATAL {span}: Non-bool expression used as a condition"), 
			BadEqualityOne 	=> eprintln!("FATAL {span}: Invalid equality operand"), 
            BadEqualityTwo 	=> eprintln!("FATAL {span}: Invalid equality operation"), 
			BadAssignOne 	=> eprintln!("FATAL {span}: Invalid assignment operand"), 
            BadAssignTwo 	=> eprintln!("FATAL {span}: Invalid assignment operation"), 
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
	BadEqualityOne,
    BadEqualityTwo,
	BadAssignOne,
    BadAssignTwo,
	NonLval
}