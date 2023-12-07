use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error};

mod three_ac_node;
use three_ac_node::ThreeAC;
use three_ac_node::FunctionType;

use crate::parser::ast::{PrimType, Program, TypeKind};

pub fn convert_3ac(prog: Box<Program>) -> String {
    let mut vars: IRSymbolTable = IRSymbolTable {
        globals: Vec::new(),
        strings: Vec::new(),
        functions: HashMap::new()
    };

    prog.find_vars(&FunctionType::Global, &mut vars);

    let mut counts: Counter = Counter {
        lbl: 0,
        tmp: 0
    };

    // println!("{}", vars);
    prog.convert_3ac(&mut vars, &mut counts)
}

#[derive(Copy, Clone)]
pub enum Variable3ACType {
    Int,
    Bool,
    String
}

impl Display for Variable3ACType {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            Variable3ACType::Int => write!(fmt, "int"),
            Variable3ACType::Bool => write!(fmt, "bool"),
            Variable3ACType::String => write!(fmt, "string"),
        }
    }
}
pub struct Variable3AC {
    id: String,
    var_type: Variable3ACType
}

impl Display for Variable3AC {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}-{}", self.id, self.var_type)
    }
}

pub struct IRSymbolTable {
    globals: Vec<Variable3AC>,
    strings: Vec<String>,
    functions: HashMap<String, FunctionValue>
}


pub struct Counter {
    lbl: usize,
    tmp: usize,
}

struct FunctionValue {
    locals: Vec<Variable3AC>,
    tmps: Vec<Variable3ACType>
}

impl Display for FunctionValue {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "\t[{}]\n\tTemps: [{}]", 
        self.locals.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","), 
        self.tmps.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","))
    }
}

impl FunctionValue {
    fn new() -> FunctionValue {
        FunctionValue { 
            locals: Vec::new(), 
            tmps: Vec::new()
        }
    }
}

impl Display for IRSymbolTable {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let mut output: Vec::<String> = Vec::new();
        output.push("\n======================".to_string());
        
        output.push("\nGlobals:".to_string());
        output.push(self.globals());
        output.push("\nFunctions:".to_string());
        for (id, val) in self.functions.iter() {
            output.push(format!("# {}\n{}", id, val));
        }
        output.push("======================\n".to_string());
        write!(fmt, "{}", output.join("\n"))
    }
}

impl IRSymbolTable {
    fn add_var(&mut self, scope: &FunctionType, var_id: String, var_type: TypeKind) {
        use FunctionType::*;
        use TypeKind::*;
        use PrimType::*;
        let var_type = match var_type {
            Prim(x) => match x {
                Bool =>  Variable3ACType::Bool,
                _ => Variable3ACType::Int,
            }
            _ => Variable3ACType::Int
        };
        let var = Variable3AC {
            id: var_id,
            var_type: var_type
        };
        match scope {
            Global => {self.globals.push(var);},
            Local { id } => {
                self.functions.entry(id.to_owned())
                    .and_modify(|val| val.locals.push(var))
                    .or_insert(FunctionValue::new());
            }
        }
    }

    fn add_fn(&mut self, id: String) {
        self.functions.insert(id.clone(), FunctionValue::new());
        // self.globals.push(id);                  // TODO: took this out to help x86, remove later
    }

    fn add_string(&mut self, str: String) {
        self.strings.push(str);
    }

    fn inc_fn_tmps(&mut self, curr_fn: &FunctionType, tmp_type: Variable3ACType) {
        let fn_id = match curr_fn {
            FunctionType::Global => return,
            FunctionType::Local { id } => id.to_owned()
        };

        self.functions.entry(fn_id)
            .and_modify(|val| val.tmps.push(tmp_type))
            .or_insert_with(|| {
                let mut val = FunctionValue::new();
                val.tmps.push(tmp_type);
                val
            });
    }

    fn globals(&self) -> String {
        let mut output : Vec::<String> = Vec::new();

        for gbl in self.globals.iter() {
            output.push(format!("{}", gbl));
        }

        for (i, str) in self.strings.iter().enumerate() {
            output.push(format!("str{} {}", i, str));
        }

        output.join("\n")
    }

    fn fn_locals(&self, id: &String) -> String {
        let mut output : Vec::<String> = Vec::new();
        
        let val = match self.functions.get(id) {
            Some(val) => val,
            None => return "".to_string()
        };

        for local in val.locals.iter() {
            output.push(format!("{}", local));
        }

        
        for (i, tmp_type) in val.tmps.iter().enumerate() {
            output.push(format!("tmp{}-{}", i, tmp_type));
        }

        output.join("\n")
    }

    fn id_from_string(&self, str: &String) -> String {
        for (i, s) in self.strings.iter().enumerate() {
            if s == str {
                return format!("str{}", i);
            }
        }
        "".to_string()
    }
}