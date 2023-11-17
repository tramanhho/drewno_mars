mod three_ac_node;
use std::collections::HashMap;
use three_ac_node::ThreeAC;
use three_ac_node::FunctionType;
use crate::parser::ast::Program;
use std::fmt::{Display, Formatter, Error};

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

pub struct IRSymbolTable {
    globals: Vec<String>,
    strings: Vec<String>,
    functions: HashMap<String, FunctionValue>
}

pub struct Counter {
    lbl: usize,
    tmp: usize,
}

struct FunctionValue {
    locals: Vec<String>,
    tmps: usize
}

impl Display for FunctionValue {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "\t[{}]\n\tTemps: {}", self.locals.join(","), self.tmps)
    }
}

impl FunctionValue {
    fn new() -> FunctionValue {
        FunctionValue { 
            locals: Vec::new(), 
            tmps: 0 
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
    fn add_var(&mut self, scope: &FunctionType, var_id: String) {
        use FunctionType::*;

        match scope {
            Global => {self.globals.push(var_id);},
            Local { id } => {
                self.functions.entry(id.to_owned())
                    .and_modify(|val| val.locals.push(var_id))
                    .or_insert(FunctionValue::new());
            }
        }
    }

    fn add_fn(&mut self, id: String) {
        self.functions.insert(id.clone(), FunctionValue::new());
        self.globals.push(id);
    }

    fn add_string(&mut self, str: String) {
        self.strings.push(str);
    }

    fn inc_fn_tmps(&mut self, curr_fn: &FunctionType) {
        let fn_id = match curr_fn {
            FunctionType::Global => return,
            FunctionType::Local { id } => id.to_owned()
        };

        self.functions.entry(fn_id)
            .and_modify(|val| val.tmps += 1)
            .or_insert_with(|| {
                let mut val = FunctionValue::new();
                val.tmps += 1;
                val
            });
    }

    fn globals(&self) -> String {
        let mut output : Vec::<String> = Vec::new();

        for id in self.globals.iter() {
            output.push(format!("{}", id));
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
            output.push(format!("{} (local var of 8 bytes)", local));
        }

        
        for i in 0..val.tmps {
            output.push(format!("tmp{} (tmp var of 8 bytes)", i));
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