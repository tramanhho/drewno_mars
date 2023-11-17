use crate::parser::ast::*;
use super::{IRSymbolTable, Counter};

#[derive(Debug)]
pub enum FunctionType {
    Global,
    Local{ id: String }
}

pub trait ThreeAC {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable);
    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter) -> String;
}
pub trait Exp3AC {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable);
    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter, curr: Vec<String>) -> (Vec<String>, String);
}

pub trait ExpKind3AC {
    fn to_3ac(&self) -> String;
}

impl ThreeAC for Box<Program> {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        for gbl in self.globals.iter() {
            gbl.find_vars(curr_fn, vars);
        }
    }

    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter) -> String {
        let mut output : Vec<String> = Vec::new();
        output.push("[BEGIN GLOBALS]".to_string());
        output.push(vars.globals());
        output.push("[END GLOBALS]".to_string());

        for gbl in self.globals.iter() {
            let out = gbl.convert_3ac(vars, counts);
            if out.trim() != "" {
                output.push(format!("\t{}", out));
            }
        }

        output.join("\n")
    }
}

impl ThreeAC for Decl {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        use Decl::*;

        match *self {
            VarDecl(ref x) => x.find_vars(curr_fn, vars),
            FnDecl(ref x) => x.find_vars(curr_fn, vars),
            _ => ()
        }
    }

    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter) -> String {
        use Decl::*;

        match *self {
            VarDecl(ref x) => x.convert_3ac(vars, counts),
            FnDecl(ref x) => x.convert_3ac(vars, counts),
            _ => "".to_string()
        }
    }
}

impl ThreeAC for VarDecl {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        vars.add_var(curr_fn, self.id.to_string());
    }

    fn convert_3ac(&self, _vars: &mut IRSymbolTable, _counts: &mut Counter) -> String {
        "".to_string() // drew davidson forgor to add so we don't need to ^-^ 

        // match &self.init_val {
        //     Some(v) => "".to_string(),
        //     None => "".to_string(),
        // }
    }
}

impl ThreeAC for FnDecl {
    fn find_vars(&self, _curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        vars.add_fn(self.id.to_string());

        for stmt in self.body.iter() {
            stmt.find_vars(&FunctionType::Local { id: self.id.to_string() }, vars);
        }
    }
    
    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter) -> String {
        let mut output : Vec<String> = Vec::new();
        let id = self.id.to_string();
        output.push("".to_string());
        output.push(format!("[BEGIN {} LOCALS]", id));
        output.push(vars.fn_locals(&id));
        output.push(format!("[END {} LOCALS]", id));
        output.push("".to_string());

        output.push(format!("{}:\tenter {}", id, id));
        let old_lbl_counter = counts.lbl;
        counts.lbl += 1;
        counts.tmp = 0;
        for stmt in self.body.iter() {
            let out = stmt.convert_3ac(vars, counts);
            if out.trim() != "" {
                output.push(format!("\t{}", out));
            }
        }
        output.push(format!("\tgoto lbl_{}", old_lbl_counter));
        output.push(format!("lbl_{}:\tleave {}", old_lbl_counter, id));
        output.join("\n")
    }
}

impl ThreeAC for FormalDecl {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        use self::FormalDecl::*;
        match self {
            VarDecl(ref x) => x.find_vars(curr_fn, vars),
            FormalDecl{id, formal_type: _,} => {
                vars.add_var(curr_fn, id.to_string())
            }
        }
    }

    fn convert_3ac(&self, _vars: &mut IRSymbolTable, _counts: &mut Counter) -> String {
        "".to_string() // drew davidson forgor to add so we don't need to ^-^ 

        // match &self.init_val {
        //     Some(v) => "".to_string(),
        //     None => "".to_string(),
        // }
    }
}

impl ThreeAC for Stmt {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        use Stmt::*;
        match self {
            Block(ref x) => x.find_vars(curr_fn, vars),
            Line(ref x) => x.find_vars(curr_fn, vars),
            VarDecl(ref x) => x.find_vars(curr_fn, vars),
        };
    }

    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter) -> String {
        use Stmt::*;
        match self {
            Block(ref x) => x.convert_3ac(vars, counts),
            Line(ref x) => x.convert_3ac(vars, counts),
            VarDecl(ref x) => "".to_string(),
        }
    }
}

impl ThreeAC for BlockStmt {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        use BlockStmt::*;
        match self {
            While{cond, body} | If{cond, body} => {
                cond.find_vars(curr_fn, vars);

                for stmt in body.iter() {
                    stmt.find_vars(curr_fn, vars);
                }
            },
            IfElse{cond, true_branch, false_branch} => {
                cond.find_vars(curr_fn, vars);
                
                for stmt in true_branch.iter() {
                    stmt.find_vars(curr_fn, vars);
                }
                for stmt in false_branch.iter() {
                    stmt.find_vars(curr_fn, vars);
                }
            },
        }
    }

    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter) -> String {
        use BlockStmt::*;
        let output: Vec<String> = Vec::new();
        match self {
            While{cond, body} | If{cond, body} => {
                
                "".to_string()
            },
            IfElse{cond, true_branch, false_branch} => "".to_string(),
        }
    }
}

impl ThreeAC for LineStmt {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        self.kind.find_vars(curr_fn, vars);
    }

    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter) -> String {
        self.kind.convert_3ac(vars, counts)
    }
}

impl ThreeAC for LineStmtKind {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        use LineStmtKind::*;

        match self {
            Assign{dest: _, src} => src.find_vars(curr_fn, vars),
            Give{output} => output.find_vars(curr_fn, vars),
            Return{result} => {match result {
                Some(exp) => exp.find_vars(curr_fn, vars),
                None => ()}},
            Call(ref exp) => exp.find_vars(curr_fn, vars),
            _ => ()
        }
    }

    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter) -> String {
        use LineStmtKind::*;
        match self {

            Assign{dest, src} => {
                let (pre_src, new_src) = 
                    src.convert_3ac(vars, counts, Vec::new());
                if pre_src.len() > 0 {
                    format!("{}\n\t[{}] := {}", pre_src.join("\n\t"), dest, new_src)
                } else {
                    format!("[{}] := {}", dest, new_src)
                }
                
            },

            PostDec{loc} => format!("[{}] := {} ADD64 1", loc, loc),

            PostInc{loc} => format!("[{}] := {} SUB64 1", loc, loc),

            Give{output} => {
                let (pre_out, new_output) = 
                    output.convert_3ac(vars, counts, Vec::new());
                if pre_out.len() > 0 {
                    format!("{}\n\tWRITE {}", pre_out.join("\n\t"), new_output)
                } else {
                    format!("WRITE {}", new_output)
                }
            },

            Take{recipient} => format!("READ [{}]", recipient),

            Return{result} => {match result {
                Some(x) => {
                    let (pre_out, new_ret) = 
                        x.convert_3ac(vars, counts, Vec::new());
                    if pre_out.len() > 0 {
                        format!("{}\n\tsetret {}", pre_out.join("\n\t"), new_ret)
                    } else {
                        format!("setret {}", new_ret)
                    }
                    
                },
                None => "".to_string()}},

            Exit => "exit".to_string(),

            Call(ref exp) => {
                let (pre_call, new_call) = 
                    exp.convert_3ac(vars, counts, Vec::new());
                if pre_call.len() > 0 {
                    format!("{}\n\tcall {}", pre_call.join("\n\t"), new_call)
                } else {
                    format!("call {}", new_call)
                }
            },
        }
    }
}

impl Exp3AC for Exp {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        self.kind.find_vars(curr_fn, vars);
    }

    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter, curr: Vec<String>) -> (Vec<String>, String) {
        self.kind.convert_3ac(vars, counts, curr)
    }
}

impl Exp3AC for ExpKind {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        use ExpKind::*;

        match self {
            UnaryExp(ref exp) => exp.find_vars(curr_fn, vars),
            BinaryExp(ref exp) => exp.find_vars(curr_fn, vars),
            CallExp(ref exp) => exp.find_vars(curr_fn, vars),
            // IntLit(ref lit) => "".to_string(),
            StrLit(ref lit) => vars.add_string(lit.to_string()),
            _ => ()
        }
    }

    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter, curr: Vec<String>) -> (Vec<String>, String) {
        use ExpKind::*;

        match self {
            True => (curr, "1".to_string()),
            False => (curr, "0".to_string()),
            // Magic => {
            //     curr.push()
            // },
            UnaryExp(exp) => exp.convert_3ac(vars, counts, curr),
            BinaryExp(exp) => exp.convert_3ac(vars, counts, curr),
            CallExp(exp) => exp.convert_3ac(vars, counts, curr),
            IntLit(i32) => (curr, i32.to_string()),
            StrLit(str) => (curr, vars.id_from_string(str)),
            Loc(loc) => (curr, loc.to_string()),
            _ => (curr, "".to_string())
        }
    }
}

impl Exp3AC for UnaryExp {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        vars.inc_fn_tmps(curr_fn);
        self.exp.find_vars(curr_fn, vars);
    }

    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter, mut curr: Vec<String>) -> (Vec<String>, String) {
        let exp;
        (curr, exp) = self.exp.convert_3ac(vars, counts, curr);

        let ret = format!("[tmp{}]", counts.tmp);
        curr.push(format!("{} := {} {}", ret, self.kind.to_3ac(), exp));
        counts.tmp += 1;

        (curr, ret)
    }
}

impl ExpKind3AC for UnaryExpKind {
    fn to_3ac(&self) -> String {
        match self {
            UnaryExpKind::Not => "NOT64".to_string(),
            UnaryExpKind::Neg => "NEG64".to_string(),
        }
    }
}

impl Exp3AC for BinaryExp {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        vars.inc_fn_tmps(curr_fn);

        self.lhs.find_vars(curr_fn, vars);
        self.rhs.find_vars(curr_fn, vars);
    }

    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter, mut curr: Vec<String>) -> (Vec<String>, String) {
        let lhs;
        (curr, lhs) = self.lhs.convert_3ac(vars, counts, curr);

        let rhs;
        (curr, rhs) = self.rhs.convert_3ac(vars, counts, curr);

        let ret = format!("[tmp{}]", counts.tmp);
        curr.push(format!("{} := {} {} {}", ret, lhs, self.kind.to_3ac(), rhs));
        counts.tmp += 1;
        
        (curr, ret)
    }
}

impl ExpKind3AC for BinaryExpKind {
    fn to_3ac(&self) -> String {
        use BinaryExpKind::*;
        match self {
            And => "AND64".to_string(),
            Or => "OR64".to_string(),
            Equals => "EQ64".to_string(),
            NotEquals => "NEQ64".to_string(),
            Greater => "GT64".to_string(),
            Less => "LT64".to_string(),
            GreaterEq => "GTE64".to_string(),
            LessEq => "LTE64".to_string(),
            Plus => "ADD64".to_string(),
            Minus => "SUB64".to_string(),
            Times => "MULT64".to_string(),
            Divide => "DIV64".to_string(),
        }
    }
}

impl Exp3AC for CallExp {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        vars.inc_fn_tmps(curr_fn);
        for arg in self.args.iter() {
            arg.find_vars(curr_fn, vars);
        }
    }

    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter, mut curr: Vec<String>) -> (Vec<String>, String) {
        for arg in self.args.iter() {
            let _ : String;
            (curr, _) = arg.convert_3ac(vars, counts, curr);
        }
        (curr, self.name.to_string())
    }
}