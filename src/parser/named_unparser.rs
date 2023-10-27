
pub mod named_node;
use super::ast::*;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::fmt::{Display, Formatter, Error};
pub enum NameError {
	BadType,
	MultipleDecl,
	UndefinedDecl
}

impl NamedUnparser {
    fn add_entry(&mut self, id: String, kind: SymbolKind) {
        let key : SymbolKey = SymbolKey {
            id: id,
            scope: self.scope,
        };
		let mut error = false;
		use crate::parser::ast::Type::*;
		match kind {
			SymbolKind::Variable { ref var_type } => {
				match var_type {
					Prim(PrimType::Void) | PerfectPrim(PrimType::Void) => {
						self.report_error(NameError::BadType);
						error = true;
					},
					_ => ()
				}
			},
			_ => ()
		};

        match self.table.entry(key.clone()) {
            Occupied(_) => { self.report_error(NameError::MultipleDecl); error = true; },
            Vacant(_) => { }
        }

		if !error {
			self.table.insert(key, kind);
		}

    }

	fn report_error(&mut self, error: NameError) {
		match error {
			NameError::BadType => eprintln!("FATAL [range]: Invalid type in declaration"),
			NameError::MultipleDecl => eprintln!("FATAL [range]: Multiply declared identifier"),
			NameError::UndefinedDecl => eprintln!("FATAL [range]: Undeclared identifier")
		}
		self.error = true;
	}

    fn add_class_entry(&mut self, class_id: String, field_id: String, kind: SymbolKind) {
        match self.classes.entry(class_id.clone()) {
            Occupied(mut x) => {
				let class = x.get_mut();
                match class.entry(field_id.clone()) {
                    Occupied(_) => self.report_error(NameError::MultipleDecl),
                    Vacant(_) => { class.insert(field_id, kind); }
                }
            },
            Vacant(_) => {
                let mut new_class: HashMap<String, SymbolKind> = HashMap::new();
                new_class.insert(field_id, kind);
                self.classes.insert(class_id, new_class);
            }
        }
    }

    fn add_class_instance(&mut self, class_id: String, var_id: String) {
        match self.classes.entry(class_id.clone()) {
            Occupied(x) => {
                for field in x.get().clone().values() {
                    self.add_entry( format!("{}--{}", var_id, class_id), field.clone());
                }
            },
            Vacant(_) => {
				self.report_error(NameError::UndefinedDecl);
            }
        }
    }

	fn remove_scope(&mut self, scope: u8) {
        self.table.retain(|k, _| k.scope != scope);
    }
}

impl Display for NamedUnparser {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let mut output : Vec<String> = Vec::new();
		output.push("\n======================".to_string());
        output.push("Classes:".to_string());
        for (class_name, fields) in self.classes.iter() {
            output.push(format!("  {}: ", class_name));
            for (field_name, field_type) in fields.iter() {
                output.push(format!("    {}: {}", field_name, field_type));
            }
        }
		output.push("".to_string());
		output.push("Variables and Functions:".to_string());
		for (key, kind) in self.table.iter() {
            output.push(format!("  {}: {}", key, kind));
        }
		output.push("======================\n".to_string());
        write!(fmt, "{}", output.join("\n"))
    }
}

pub struct NamedUnparser {
    pub scope: u8,
    pub table: HashMap<SymbolKey, SymbolKind>,
    pub classes: HashMap<String, HashMap<String, SymbolKind>>,
    pub error: bool
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct SymbolKey {
    id: String,
    scope: u8 // int incremented corresponding to nesting level, 0 = global
}

// prepend class name to id, no need for class variant 
#[derive(Clone, Debug, PartialEq)]
pub enum SymbolKind {
    Variable { var_type: Type },
    Function { args: HashMap<String, Type>, ret: Type}
    // Function
}

impl Display for SymbolKey {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "[{}] {}", self.scope, self.id)
    }
}

impl Display for SymbolKind {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::SymbolKind::*;
        match self {
            Variable{ref var_type} => write!(fmt, "{}", var_type),
            Function{ref args, ref ret} => {
                let return_type = ret.to_string();
                write!(fmt, "{{({})->{}}}", args.get_types(), return_type)
            },
        }   
    }
}

trait ArgTable {
    fn add(&mut self, arg: Box<FormalDecl>) -> Result<(), String>;
    fn get_types(&self) -> String;
}

impl ArgTable for HashMap<String, Type>{
    fn add(&mut self, arg: Box<FormalDecl>) -> Result<(), String> {
        let (arg_id, arg_type);
        use self::FormalDecl::*;
        match *arg {
            VarDecl(x) => {
                arg_id = x.id.to_string();
                arg_type = x.var_type;
            },
            FormalDecl{ref id, ref formal_type} => {
                arg_id = id.to_string();
                arg_type = formal_type.clone();
            }
        }

        match self.entry(arg_id.clone()) {
            Occupied(x) => Err(format!("{} is already in the arg table", x.key())),
            Vacant(_) => {
                self.insert(arg_id, *arg_type);
                Ok(())
            }
        }
    }

    fn get_types(&self) -> String {
        self.values().map(|arg_type| format!("{}", arg_type)).collect::<Vec<String>>().join(", ")
    }
}
