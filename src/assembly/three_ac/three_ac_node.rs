use crate::parser::ast::*;
use super::{IRSymbolTable, Counter, Variable3ACType};

#[derive(Debug)]
pub enum FunctionType {
    Global,
    Local{ id: String }
}

fn quad_vec_to_string(quads: Vec<String>) -> String {
    if quads.len() > 0 {
        format!("{}{}", quads.join("\n\t"), "\n\t")
    } else {
        "".to_string()
    }
}

pub trait ThreeAC {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable);
    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter) -> String;
}
pub trait Exp3AC {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable);
    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter, curr: Vec<String>) -> (Vec<String>, String);
}

pub trait Stmt3AC {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable);
    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter, leave_lbl: usize) -> String;
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
        vars.add_var(curr_fn, self.id.to_string(), *self.var_type.clone().kind);
    }

    fn convert_3ac(&self, _vars: &mut IRSymbolTable, _counts: &mut Counter) -> String {
        // "".to_string() // drew davidson forgor to add so we don't need to ^-^ 
        // TODO: fix this maybe
        match &self.init_val {
            Some(_) => "".to_string(),
            None => "".to_string(),
        }
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
        let leave_lbl = counts.lbl;
        counts.lbl += 1;
        counts.tmp = 0;
        for stmt in self.body.iter() {
            let out = stmt.convert_3ac(vars, counts, leave_lbl);
            if out.trim() != "" {
                output.push(format!("\t{}", out));
            }
        }
        // output.push(format!("\tgoto lbl_{}", old_lbl_counter));
        output.push(format!("lbl_{}:\tleave {}", leave_lbl, id));
        output.join("\n")
    }
}

impl ThreeAC for FormalDecl {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        use self::FormalDecl::*;
        match self {
            VarDecl(ref x) => x.find_vars(curr_fn, vars),
            FormalDecl{id, formal_type,} => {
                vars.add_var(curr_fn, id.to_string(), *formal_type.clone().kind)
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

impl Stmt3AC for Stmt {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        use Stmt::*;
        match self {
            Block(ref x) => x.find_vars(curr_fn, vars),
            Line(ref x) => x.find_vars(curr_fn, vars),
            VarDecl(ref x) => x.find_vars(curr_fn, vars),
        };
    }

    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter, leave_lbl: usize) -> String {
        use Stmt::*;
        match self {
            Block(ref x) => x.convert_3ac(vars, counts, leave_lbl),
            Line(ref x) => x.convert_3ac(vars, counts, leave_lbl),
            VarDecl(_) => "".to_string(),
        }
    }
}

impl Stmt3AC for BlockStmt {
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

    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter, leave_lbl: usize) -> String {
        use BlockStmt::*;
        let mut output: Vec<String> = Vec::new();
        let is_while = match self {
            While{cond: _, body: _} => true,
            _ => false
        };
        let mut loop_head = 0;
        if is_while {
            loop_head = counts.lbl;
            counts.lbl += 1;
            output.push("".to_string());
            output.push(format!("lbl_{}:\tnop", loop_head));
        }
        match self {
            While{cond, body} | If{cond, body} => {
                let (pre_cond, new_cond) = 
                    cond.convert_3ac(vars, counts, Vec::new());
                let after_lbl = counts.lbl;
                counts.lbl += 1;
                let pre_cond = quad_vec_to_string(pre_cond);
                if pre_cond.trim()  != "".to_string() {
                    output.push(format!("\t{}IFZ {} goto lbl_{}", pre_cond, new_cond, after_lbl));
                } else {
                    output.push(format!("IFZ {} goto lbl_{}", new_cond, after_lbl));
                }
                
                for stmt in body {
                    output.push(format!("\t{}", stmt.convert_3ac(vars, counts, leave_lbl)));
                }

                if is_while {
                    output.push(format!("\tgoto lbl_{}", loop_head));
                }
                output.push("".to_string());
                output.push(format!("lbl_{}:\tnop", after_lbl));
            },
            IfElse{cond, true_branch, false_branch} => {
                let (pre_cond, new_cond) = 
                    cond.convert_3ac(vars, counts, Vec::new());
                let after_lbl = counts.lbl;
                counts.lbl += 1;
                let false_lbl = counts.lbl;
                counts.lbl += 1;
                let pre_cond = quad_vec_to_string(pre_cond);

                if pre_cond.trim()  != "".to_string() {
                    output.push(format!("\t{}IFZ {} goto lbl_{}", pre_cond, new_cond, false_lbl));
                } else {
                    output.push(format!("IFZ {} goto lbl_{}", new_cond, false_lbl));
                }
                
                for stmt in true_branch {
                    output.push(format!("\t{}", stmt.convert_3ac(vars, counts, leave_lbl)));
                }
                output.push(format!("\tgoto lbl_{}", after_lbl));
                output.push("".to_string());
                output.push(format!("lbl_{}:\tnop", false_lbl));
                for stmt in false_branch {
                    output.push(format!("\t{}", stmt.convert_3ac(vars, counts, leave_lbl)));
                }
                output.push("".to_string());
                output.push(format!("lbl_{}:\tnop", after_lbl));
            },
        }
        output.join("\n")
    }
}

impl Stmt3AC for LineStmt {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        self.kind.find_vars(curr_fn, vars);
    }

    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter, leave_lbl: usize) -> String {
        self.kind.convert_3ac(vars, counts, leave_lbl)
    }
}

fn handle_call(mut new: String, counts: &mut Counter) -> (String, String) {
    let mut getret = "".to_string();

    if new.contains("call") {
        getret = format!("{}\n\tgetret [tmp{}]\n\t", new, counts.tmp);
        new = format!("[tmp{}]", counts.tmp);
        counts.tmp += 1;
    }

    (getret, new)
}

impl Stmt3AC for LineStmtKind {
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

    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter, leave_lbl: usize) -> String {
        use LineStmtKind::*;
        match self {

            Assign{dest, src} => {
                let (pre_src, new_src) = 
                    src.convert_3ac(vars, counts, Vec::new());
                let (getret, new_src) = handle_call(new_src, counts);
                let pre_src = quad_vec_to_string(pre_src);
                if pre_src.trim() != "".to_string() {
                    format!("{}{}[{}] := {}", pre_src, getret, dest, new_src)
                } else {
                    format!("{}[{}] := {}", getret, dest, new_src)
                }
            },

            PostDec{loc} => format!("[{}] := [{}] ADD64 1", loc, loc),

            PostInc{loc} => format!("[{}] := [{}] SUB64 1", loc, loc),

            Give{output} => {
                let (pre_out, mut new_output) = 
                    output.convert_3ac(vars, counts, Vec::new());
                new_output = if new_output.contains("[") {
                    let mut new_output: std::str::Chars<'_> = new_output.chars();
                    new_output.next();
                    new_output.next_back();
                    new_output.as_str().to_string()
                } else {
                    new_output
                };
                format!("{}WRITE {}", quad_vec_to_string(pre_out), new_output)
            },

            Take{recipient} => format!("READ [{}]", recipient),

            Return{result} => {match result {
                Some(x) => {
                    let (pre_out, new_ret) = 
                        x.convert_3ac(vars, counts, Vec::new());
                    let (getret, new_ret) = handle_call(new_ret, counts);
                    let pre_out = quad_vec_to_string(pre_out);
                    if pre_out.trim() != "".to_string() {
                        format!("{}{}setret {}\n\tgoto lbl_{}", pre_out, getret, new_ret, leave_lbl)
                    } else {
                        format!("{}setret {}\n\tgoto lbl_{}", getret, new_ret, leave_lbl)
                    }
                    
                },
                None => "".to_string()}},

            Exit => "exit".to_string(),

            Call(ref exp) => {
                let (pre_call, new_call) = 
                    exp.convert_3ac(vars, counts, Vec::new());
                // println!("owowowowowo{}", new_call);
                format!("{}\n\t{}", quad_vec_to_string(pre_call), new_call)
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
            True => (curr, "true".to_string()),
            False => (curr, "false".to_string()),
            Magic => (curr, "24Kmagic".to_string()),
            UnaryExp(exp) => exp.convert_3ac(vars, counts, curr),
            BinaryExp(exp) => exp.convert_3ac(vars, counts, curr),
            CallExp(exp) => exp.convert_3ac(vars, counts, curr),
            IntLit(i32) => (curr, i32.to_string()),
            StrLit(str) => (curr, vars.id_from_string(str)),
            Loc(loc) => (curr, format!("[{}]", loc))
        }
    }
}

impl Exp3AC for UnaryExp {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IRSymbolTable) {
        let exp_type = match *self.kind.clone() {
            UnaryExpKind::Neg => Variable3ACType::Int,
            UnaryExpKind::Not => Variable3ACType::Bool
        };
        
        vars.inc_fn_tmps(curr_fn, exp_type);
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
        use BinaryExpKind::*;
        use TypeKind::*;
        use PrimType::*;
        let exp_type = match *self.kind.clone() {
            And | Or => Variable3ACType::Bool,
            Equals | NotEquals => match self.lhs.expr_type.clone() {
                Some(x) => match *x.clone().kind {
                    Prim(prim) => match prim {
                        Bool => Variable3ACType::Bool,
                        _ => Variable3ACType::Int
                    },
                    _ => Variable3ACType::Int
                },
                None => Variable3ACType::Int
            },
            _ => Variable3ACType::Int
        };
        vars.inc_fn_tmps(curr_fn, exp_type);

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
        use PrimType::*;
        let call_exp_type = match self.fn_type.clone() {
            Some(x) => match *x.kind.clone() {
                TypeKind::Prim(prim) => match prim {
                    Bool => Variable3ACType::Bool,
                    _ => Variable3ACType::Int
                },
                _ => Variable3ACType::Int
            }
            None => Variable3ACType::Int
        };
        vars.inc_fn_tmps(curr_fn, call_exp_type);
        for arg in self.args.iter() {
            arg.find_vars(curr_fn, vars);
        }
    }

    fn convert_3ac(&self, vars: &mut IRSymbolTable, counts: &mut Counter, mut curr: Vec<String>) -> (Vec<String>, String) {
        for arg in self.args.iter() {
            let _ : String;
            (curr, _) = arg.convert_3ac(vars, counts, curr);
        }
        (curr, format!("call {}", self.name.to_string()))
    }
}