use crate::parser::ast::*;
use super::IntermediateRepresentation;

// fn fmt_vec_commas<T: std::fmt::ThreeAC>(vec: &Vec<T>) -> String {
//     return vec.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(", ");
// }

// fn fmt_vec<T: std::fmt::ThreeAC>(vec: &Vec<T>) -> String {
//     return vec.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join("");
// }

pub enum FunctionType {
    Global,
    Local{ id: String }
}

pub trait ThreeAC {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IntermediateRepresentation);
    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String;
}

impl ThreeAC for Box<Program> {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IntermediateRepresentation) {
        for gbl in self.globals.iter() {
            gbl.find_vars(curr_fn, vars);
        }
    }

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        self.globals.iter()
            .map(|arg| arg.convert_3ac(vars))
            .collect::<Vec<String>>().join("\n")
    }
}

impl ThreeAC for Decl {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IntermediateRepresentation) {
        use Decl::*;

        match *self {
            VarDecl(ref x) => x.find_vars(curr_fn, vars),
            FnDecl(ref x) => x.find_vars(curr_fn, vars),
            _ => ()
        }
    }

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        use Decl::*;

        match *self {
            VarDecl(ref x) => "".to_string(),
            FnDecl(ref x) => "".to_string(),
            _ => "".to_string()
        }
    }
}

impl ThreeAC for VarDecl {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IntermediateRepresentation) {
        vars.add_var(curr_fn, self.id.to_string(), None);
    }

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        match &self.init_val {
            Some(v) => "".to_string(),
            None => "".to_string(),
        }
    }
}

impl ThreeAC for FnDecl {
    fn find_vars(&self, _curr_fn: &FunctionType, vars: &mut IntermediateRepresentation) {
        vars.add_var(&FunctionType::Global, self.id.to_string(), None);

        for stmt in self.body.iter() {
            stmt.find_vars(&FunctionType::Local { id: self.id.to_string() }, vars);
        }
    }

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        "".to_string()
    }
}

impl ThreeAC for FormalDecl {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IntermediateRepresentation) {
        use self::FormalDecl::*;
        match self {
            VarDecl(ref x) => x.find_vars(curr_fn, vars),
            FormalDecl{id, formal_type: _,} => {
                vars.add_var(curr_fn, id.to_string(), None)
            }
        }
    }

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        use self::FormalDecl::*;
        match self {
            VarDecl(ref x) => "".to_string(),
            FormalDecl{id, formal_type,} => "".to_string()
        }
    }
}

impl ThreeAC for Stmt {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IntermediateRepresentation) {
        use Stmt::*;
        match self {
            Block(ref x) => x.find_vars(curr_fn, vars),
            Line(ref x) => x.find_vars(curr_fn, vars),
            VarDecl(ref x) => x.find_vars(curr_fn, vars),
        };
    }
    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        use Stmt::*;
        match self {
            Block(ref x) => "".to_string(),
            Line(ref x) => "".to_string(),
            VarDecl(ref x) => "".to_string(),
        }
    }
}

impl ThreeAC for BlockStmt {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IntermediateRepresentation) {
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

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        use BlockStmt::*;
        match self {
            While{cond, body} | If{cond, body} => {
                "".to_string()
            },
            IfElse{cond, true_branch, false_branch} => "".to_string(),
        }
    }
}

impl ThreeAC for LineStmt {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IntermediateRepresentation) {
        self.kind.find_vars(curr_fn, vars);
    }

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        "".to_string()
    }
}

impl ThreeAC for LineStmtKind {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IntermediateRepresentation) {
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

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        use LineStmtKind::*;
        match self {
            Assign{dest, src} => "".to_string(),
            PostDec{loc} => "".to_string(),
            PostInc{loc} => "".to_string(),
            Give{output} => "".to_string(),
            Take{recipient} => "".to_string(),
            Return{result} => {match result {
                Some(x) => "".to_string(),
                None => "".to_string(),}},
            Exit => "".to_string(),
            Call(ref exp) => "".to_string(),
        }
    }
}

impl ThreeAC for Exp {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IntermediateRepresentation) {
        self.kind.find_vars(curr_fn, vars);
    }

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        "".to_string()
    }
}

impl ThreeAC for ExpKind {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IntermediateRepresentation) {
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

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        use ExpKind::*;
        match self {
            True => "".to_string(),
            False => "".to_string(),
            Magic => "".to_string(),
            UnaryExp(ref exp) => "".to_string(),
            BinaryExp(ref exp) => "".to_string(),
            CallExp(ref exp) => "".to_string(),
            IntLit(ref lit) => "".to_string(),
            StrLit(ref lit) => "".to_string(),
            Loc(ref loc) => "".to_string(),
        }
    }
}

impl ThreeAC for UnaryExp {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IntermediateRepresentation) {
        vars.inc_fn_tmps(curr_fn);
        self.exp.find_vars(curr_fn, vars);
    }

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        "".to_string()
    }
}

impl ThreeAC for UnaryExpKind {
    fn find_vars(&self, _curr_fn: &FunctionType, _vars: &mut IntermediateRepresentation) {
        ()
    }

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        use UnaryExpKind::*;
        match self {
            Neg => "".to_string(),
            Not => "".to_string(),
        }
    }
}

impl ThreeAC for BinaryExp {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IntermediateRepresentation) {
        vars.inc_fn_tmps(curr_fn);

        self.lhs.find_vars(curr_fn, vars);
        self.rhs.find_vars(curr_fn, vars);
    }

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        "".to_string()
    }
}

impl ThreeAC for BinaryExpKind {
    fn find_vars(&self, _curr_fn: &FunctionType, _vars: &mut IntermediateRepresentation) {
        ()
    }

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        use BinaryExpKind::*;
        match self {
            And => "".to_string(),
            Or => "".to_string(),
            Equals => "".to_string(),
            NotEquals => "".to_string(),
            Greater => "".to_string(),
            Less => "".to_string(),
            GreaterEq => "".to_string(),
            LessEq => "".to_string(),
            Plus => "".to_string(),
            Minus => "".to_string(),
            Times => "".to_string(),
            Divide => "".to_string(),
        }
    }
}

impl ThreeAC for CallExp {
    fn find_vars(&self, curr_fn: &FunctionType, vars: &mut IntermediateRepresentation) {
        vars.inc_fn_tmps(curr_fn);
        for arg in self.args.iter() {
            arg.find_vars(curr_fn, vars);
        }
    }

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        "".to_string()
    }
}

impl ThreeAC for Loc {
    fn find_vars(&self, _curr_fn: &FunctionType, _vars: &mut IntermediateRepresentation) {
        ()
    }

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        "".to_string()
    }
}

impl ThreeAC for LocKind {
    fn find_vars(&self, _curr_fn: &FunctionType, _vars: &mut IntermediateRepresentation) {
        ()
    }

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        use self::LocKind::*;
        match self {
            Id(ref x) => "".to_string(),
            Loc { base_class , field_name} => "".to_string(),
        }
    }
}

impl ThreeAC for Id {
    fn find_vars(&self, _curr_fn: &FunctionType, _vars: &mut IntermediateRepresentation) {
        ()
    }

    fn convert_3ac(&self, vars: &mut IntermediateRepresentation) -> String {
        "".to_string()
    }
}
