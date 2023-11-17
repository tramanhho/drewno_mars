mod three_ac_node;
use std::collections::HashMap;
use three_ac_node::ThreeAC;
use three_ac_node::FunctionType;
use crate::parser::ast::Program;

pub fn convert_3ac(mut prog: Box<Program>)  {
    let mut vars: IntermediateRepresentation = IntermediateRepresentation {
        globals: HashMap::new(),
        num_str: 0,
        functions: HashMap::new(),
    };

    prog.find_vars(&FunctionType::Global, &mut vars);
    prog.convert_3ac(&mut vars);
}

pub struct IntermediateRepresentation {
    globals: HashMap<String, Option<String>>,
    num_str: usize,
    functions: HashMap<String, FunctionValue>
}

struct FunctionValue {
    locals: Vec<String>,
    tmps: usize
}

impl FunctionValue {
    fn new() -> FunctionValue {
        FunctionValue { 
            locals: Vec::new(), 
            tmps: 0 
        }
    }
}

impl IntermediateRepresentation {
    fn add_var(&mut self, scope: &FunctionType, var_id: String, init: Option<String>) {
        use FunctionType::*;

        match scope {
            Global => {self.globals.insert(var_id, init);},
            Local { id } => {
                self.functions.entry(id.to_owned())
                    .and_modify(|val| val.locals.push(var_id))
                    .or_insert(FunctionValue::new());
            }
        }
    }

    fn add_fn(&mut self, id: String) {
        self.functions.insert(id, FunctionValue::new());
    }

    fn add_string(&mut self, str: String) {
        self.globals.insert(format!("str{}", self.num_str), Some(str));
        self.num_str += 1;
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
}