mod named_node;
use named_node::NamedNode;

use crate::parser::ast::*;
use crate::parser::ast::position::{line_bytes, PositionAPI, Position};
use crate::format::add_tabs;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::fmt::{Display, Formatter, Error};

pub fn named_unparse(mut prog: Box<Program>, raw_input: String) -> String {
    let mut unparser: NamedUnparser = NamedUnparser {
        scope: 0,
        table: HashMap::new(),
        classes: HashMap::new(),
        error: false
    };

    prog.correct_position_rec(&line_bytes(raw_input));
    let named_unparse = add_tabs(prog.named_unparse(&mut unparser));

    // println!("{}", unparser);
    if !unparser.error {
        named_unparse
    } else {
        eprintln!("Name Analysis Failed");
        "".to_string()
    }
}
pub enum NameError {
	BadType,
	MultipleDecl,
	UndefinedDecl
}

pub struct NamedUnparser {
    pub scope: usize,
    pub table: HashMap<SymbolKey, SymbolKind>,
    pub classes: HashMap<String, HashMap<String, SymbolKind>>,
    pub error: bool
}

impl NamedUnparser {
    fn add_entry(&mut self, id: String, kind: SymbolKind, position: &Position) {
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
						self.report_error(NameError::BadType, position);
						error = true;
					},
					_ => ()
				}
			},
			_ => ()
		};

        match self.table.entry(key.clone()) {
            Occupied(_) => { self.report_error(NameError::MultipleDecl, position); error = true; },
            Vacant(_) => { }
        }

		if !error {
			self.table.insert(key, kind);
		}

    }

	fn report_error(&mut self, error: NameError, position: &Position) {
		match error {
			NameError::BadType => eprintln!("FATAL {position}: Invalid type in declaration"),
			NameError::MultipleDecl => eprintln!("FATAL {position}: Multiply declared identifier"),
			NameError::UndefinedDecl => eprintln!("FATAL {position}: Undeclared identifier")
		}
		self.error = true;
	}

    fn add_class_entry(&mut self, class_id: String, field_id: String, kind: SymbolKind, position: &Position) {
        match self.classes.entry(class_id.clone()) {
            Occupied(mut x) => {
				let class = x.get_mut();
                match class.entry(field_id.clone()) {
                    Occupied(_) => self.report_error(NameError::MultipleDecl, position),
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

    fn add_class_instance(&mut self, class_id: String, var_id: String, position: &Position) {
        match self.classes.entry(class_id.clone()) {
            Occupied(x) => {
                for field in x.get().clone().values() {
                    self.add_entry( format!("{}--{}", var_id, class_id), field.clone(), position);
                }
            },
            Vacant(_) => {
				self.report_error(NameError::UndefinedDecl, position);
            }
        }
    }

	fn remove_scope(&mut self, scope: usize) {
        self.table.retain(|k, _| k.scope != scope);
    }

    fn find_entry(&mut self, id: &Id) -> Result<SymbolKind, ()> {
        let mut key_check: Vec<SymbolKey> = Vec::new();

        for s in (0..self.scope).rev() {
            key_check.push(SymbolKey {
                id: id.to_string(),
                scope: s
            });
        }

        for key in key_check.into_iter() {
            match self.table.clone().get(&key) {
                Some(kind) => {
                    return Ok(kind.clone()); 
                },
                None => (),
            }
        }
        
        self.report_error(NameError::UndefinedDecl, &id.position);
        Err(())
    }
}

impl Display for NamedUnparser {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let mut output : Vec<String> = Vec::new();
		output.push("\n======================".to_string());
        output.push(format!("Scope: {}", self.scope));
        output.push("Classes:".to_string());
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

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct SymbolKey {
    id: String,
    scope: usize // int incremented corresponding to nesting level, 0 = global
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
    fn add(&mut self, arg: Box<FormalDecl>);
    fn get_types(&self) -> String;
}

impl ArgTable for HashMap<String, Type>{
    fn add(&mut self, arg: Box<FormalDecl>) {
        let (arg_id, arg_type);
        use self::FormalDecl::*;
        match *arg {
            VarDecl(x) => {
                arg_id = x.id.to_string();
                arg_type = x.var_type;
            },
            FormalDecl{ref id, ref formal_type, } => {
                arg_id = id.to_string();
                arg_type = formal_type.clone();
            }
        }

        if !self.contains_key(&arg_id) {
            self.insert(arg_id, *arg_type);
        }
    }

    fn get_types(&self) -> String {
        self.values().map(|arg_type| format!("{}", arg_type)).collect::<Vec<String>>().join(", ")
    }
}
