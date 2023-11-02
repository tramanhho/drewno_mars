use crate::parser::ast::*;

use std::fmt::{Display, Formatter, Error};

impl Display for Program {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}", fmt_vec(&self.globals))
    }
}

impl Display for Decl {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use Decl::*;

        match *self {
            VarDecl(ref x) => write!(fmt, "{}", x),
            ClassDecl(ref x) => write!(fmt, "{}", x),
            FnDecl(ref x) => write!(fmt, "{}", x),
        }
    }
}

impl Display for VarDecl {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match &self.init_val {
            Some(v) => write!(fmt, "{} : {} = {};\n", &self.id, &self.var_type, v),
            None => write!(fmt, "{} : {};\n", &self.id, &self.var_type),
        }
    }
}

impl Display for Type {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use Type::*;
        match *self {
            Prim(ref x) => write!(fmt, "{}", x),
            PerfectPrim(ref x) => write!(fmt, "perfect {}", x),
            Class(ref x) => write!(fmt, "{}", x),
            PerfectClass(ref x) => write!(fmt, "perfect {}", x),
        }   
    }
}

impl Display for PrimType {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use PrimType::*;
        match *self {
            Int => write!(fmt, "int"),
            Bool => write!(fmt, "bool"),
            Void => write!(fmt, "void"),
        }   
    }
}

impl Display for ClassDecl {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{} : class {{\n{}}};\n", &self.id, fmt_vec(&self.member_f))
    }
}

impl Display for FnDecl {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, r#"{} : ({}) {} {{\n{}}}\n"#, &self.id, fmt_vec_commas(&self.args), &self.ret, fmt_vec(&self.body))
    }
}

impl Display for FormalDecl {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::FormalDecl::*;
        match self {
            VarDecl(ref x) => write!(fmt, "{}", x),
            FormalDecl{id, formal_type,} => write!(fmt, "{} : {}", id, formal_type)
        }
    }
}

impl Display for Stmt {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use Stmt::*;
        match self {
            Block(ref x) => write!(fmt, "{}", x),
            Line(ref x) => write!(fmt, "{};\n", x),
            VarDecl(ref x) => write!(fmt, "{}", x),
        }
    }
}

impl Display for BlockStmt {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use BlockStmt::*;
        match self {
            While{cond, body} => write!(fmt, "while ({}) {{\n{}}}\n", cond, fmt_vec(body)),
            If{cond, body} => write!(fmt, "if ({}) {{\n{}}}\n", cond, fmt_vec(body)),
            IfElse{cond, true_branch, false_branch} => write!(fmt, "if ({}) {{\n{}}}\nelse {{\n{}}}\n", cond, fmt_vec(true_branch), fmt_vec(false_branch)),
        }
    }
}

impl Display for LineStmt {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use LineStmt::*;
        match self {
            Assign{dest, src} => write!(fmt, "{} = {}", dest, src),
            PostDec{loc} => write!(fmt, "{}--", loc),
            PostInc{loc} => write!(fmt, "{}++", loc),
            Give{output} => write!(fmt, "give {}", output),
            Take{recipient} => write!(fmt, "take {}", recipient),
            Return{result} => {match result {
                Some(x) => write!(fmt, "return {}", x),
                None => write!(fmt, "return"),}},
            Exit => write!(fmt, "today I don't feel like doing any work"),
            Call(ref exp) => write!(fmt, "{}", exp),
        }
    }
}

impl Display for Exp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use Exp::*;
        match self {
            True => write!(fmt, "true"),
            False => write!(fmt, "false"),
            Magic => write!(fmt, "magic"),
            UnaryExp(ref exp) => write!(fmt, "{}", exp),
            BinaryExp(ref exp) => write!(fmt, "{}", exp),
            CallExp(ref exp) => write!(fmt, "{}", exp),
            IntLit(ref lit) => write!(fmt, "{}", lit),
            StrLit(ref lit) => write!(fmt, "{}", lit),
            Loc(ref loc) => write!(fmt, "{}", loc),
        }
    }
}

impl Display for UnaryExp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use UnaryExp::*;
        match self {
            Neg{exp} => write!(fmt, "-{}", exp),
            Not{exp} => write!(fmt, "!{}", exp),
        }
    }
}

impl Display for BinaryExp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use BinaryExp::*;
        match self {
            And{lhs, rhs} => write!(fmt, "{} and {}", lhs, rhs),
            Or{lhs, rhs} => write!(fmt, "{} or {}", lhs, rhs),
            Equals{lhs, rhs} => write!(fmt, "{} == {}", lhs, rhs),
            NotEquals{lhs, rhs} => write!(fmt, "{} != {}", lhs, rhs),
            Greater{lhs, rhs} => write!(fmt, "{} > {}", lhs, rhs),
            Less{lhs, rhs} => write!(fmt, "{} < {}", lhs, rhs),
            GreaterEq{lhs, rhs} => write!(fmt, "{} >= {}", lhs, rhs),
            LessEq{lhs, rhs} => write!(fmt, "{} <= {}", lhs, rhs),
            Plus{lhs, rhs} => write!(fmt, "{} + {}", lhs, rhs),
            Minus{lhs, rhs} => write!(fmt, "{} - {}", lhs, rhs),
            Times{lhs, rhs} => write!(fmt, "{} * {}", lhs, rhs),
            Divide{lhs, rhs} => write!(fmt, "{} / {}", lhs, rhs),
        }
    }
}

impl Display for CallExp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}({})", &self.name, fmt_vec_commas(&self.args))
    }
}

impl Display for Loc {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Loc::*;
        match self {
            Id(ref x) => write!(fmt, "{}", x),
            Loc { base_class , field_name} => write!(fmt, "{}--{}", base_class, field_name),
        }
    }
}

impl Display for Id {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}", &self.name)
    }
}

fn fmt_vec_commas<T: std::fmt::Display>(vec: &Vec<T>) -> String {
    return vec.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(", ");
}

fn fmt_vec<T: std::fmt::Display>(vec: &Vec<T>) -> String {
    return vec.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join("");
}