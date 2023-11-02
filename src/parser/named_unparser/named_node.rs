use crate::parser::named_unparser::*;
use crate::parser::ast::*;

pub trait NamedNode {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String;
}

trait NamedNodeVector {
    fn named_unparse_vec(self, unparser: &mut NamedUnparser, join: &'static str) -> String;
}

impl NamedNodeVector for Vec<Box<Stmt>> {
    fn named_unparse_vec(self: Vec<Box<Stmt>>, unparser: &mut NamedUnparser, join: &'static str) -> String {
        return self.iter()
            .map(|arg| arg.named_unparse(unparser))
            .collect::<Vec<String>>().join(join);
    }
}

impl NamedNodeVector for Vec<Box<Exp>> {
    fn named_unparse_vec(self: Vec<Box<Exp>>, unparser: &mut NamedUnparser, join: &'static str) -> String {
        return self.iter()
            .map(|arg| arg.named_unparse(unparser))
            .collect::<Vec<String>>().join(join);
    }
}

fn named_unparse_vec(vec: Vec<Box<dyn NamedNode>>, unparser: &mut NamedUnparser, join: &'static str) -> String {
    return vec.iter()
        .map(|arg| arg.named_unparse(unparser))
        .collect::<Vec<String>>().join(join);
}


impl NamedNode for Box<Program> {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        self.globals.iter()
            .map(|arg| arg.named_unparse(unparser))
            .collect::<Vec<String>>().join("\n")
    }
}

impl NamedNode for Decl {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        use Decl::*;

        match *self {
            VarDecl(ref x) => x.named_unparse(unparser),
            ClassDecl(ref x) => x.named_unparse(unparser),
            FnDecl(ref x) => x.named_unparse(unparser),
        }
    }
}

impl Decl {
    fn get_named_string(&self, unparser: &mut NamedUnparser) -> String {
        use Decl::*;

        match *self {
            VarDecl(ref x) => {
                x.get_named_string()
            },
            FnDecl(ref x) => {
                let (output, _unparser) = x.clone().get_named_string(unparser);
                output
            },
            _ => "".to_string()
        }
    }
}

impl NamedNode for VarDecl {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        unparser.add_entry(self.id.to_string(), SymbolKind::Variable {var_type: *self.var_type.clone()} );
        match &self.init_val {
			Some(v) => format!("{}{{{}}} : {} = {};", &self.id, &self.var_type, &self.var_type, v),
			None => format!("{}{{{}}} : {};", &self.id, &self.var_type, &self.var_type),
		}
    } 
}

impl VarDecl {
    fn get_named_string(&self) -> String {
        let init_val = match self.init_val.clone() {
            Some(x) => format!(" = {}", x),
            None => "".to_string()
        };
        format!("{}{{{}}} : {}{}", self.id, self.var_type, self.var_type, init_val)
    }
}

impl NamedNode for ClassDecl {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        use Decl::*;
        let mut field_outputs: Vec<String> = Vec::new();
        for field in self.member_f.iter() {
            match *field.clone() {
                VarDecl(ref x) => {
                    let x = *x.clone();
                    let kind = SymbolKind::Variable { var_type: *x.var_type.clone() };
                    unparser.add_class_entry(self.id.to_string(), x.id.to_string(), kind);
                    field_outputs.push(x.clone().get_named_string());
                },
                FnDecl(x) => {
                    let (unparser, kind) = process_fn(unparser, x.args.clone(), x.ret.clone());
                    unparser.add_class_entry(self.id.to_string(), x.id.to_string(), kind);
                    let (output, _unparser) = x.clone().get_named_string(unparser);
                    field_outputs.push(output)
                },
                _ => {}
            }
        }

        format!("{}{{{}}} : class {{\n{}}};\n", 
            self.id, 
            self.id, 
            field_outputs.join("\n")
        )
    }
}

fn process_fn(unparser: &mut NamedUnparser, args: Vec<Box<FormalDecl>>, ret: Box<Type>) -> (&mut NamedUnparser, SymbolKind) {
    let mut arg_map : HashMap<String, Type> = HashMap::new();
    unparser.scope += 1;
    for arg in args.iter() {
        use self::FormalDecl::*;
        let (id, argkind)  = match *arg.clone() {
            VarDecl(x) => {
                (x.id.to_string(), SymbolKind::Variable { var_type: *x.var_type.clone() })
            },
            FormalDecl{ref id, ref formal_type} => {
                (id.to_string(), SymbolKind::Variable { var_type: *formal_type.clone() })
            }
        };
        arg_map.add(arg.clone());
        unparser.add_entry(id, argkind);
    }
    unparser.scope -= 1;
    let value = SymbolKind::Function { 
        args: arg_map, 
        ret: *ret
    };
    // println!("IN PROCESS FN:");
    // unparser.print();
    (unparser, value)
}

impl NamedNode for FnDecl {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        let (unparser, value) = process_fn(unparser, self.args.clone(), self.ret.clone());
        unparser.add_entry(self.id.to_string(), value.clone());
        
        
        unparser.scope += 1;
        let (output, unparser) = self.clone().get_named_string(unparser);
		unparser.remove_scope(unparser.scope);
        unparser.scope -= 1;
        output
    }
}

impl FnDecl {
    fn get_named_string(self, unparser: &mut NamedUnparser) -> (String, &mut NamedUnparser) {
        let mut body = self.body.iter()
            .map(|arg| arg.named_unparse(unparser))
            .collect::<Vec<String>>().join("\n");
        body = format!("{}\n", body);

        let print_args = &self.args.iter()
            .map(|arg| arg.named_unparse(unparser))
            .collect::<Vec<String>>().join(", ");

        let fn_type = format!("{{({})->{}}}", print_args, self.ret.clone());

        let output = format!("{}{} : ({}) {} {{\n{body}}}\n", &self.id, fn_type, print_args, &self.ret);
        (output, unparser)
    }
}

impl NamedNode for FormalDecl {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        use self::FormalDecl::*;
        match self {
            VarDecl(x) => {
                x.named_unparse(unparser)
            },
            FormalDecl{ref id, ref formal_type} => {
                unparser.add_entry(id.to_string(), SymbolKind::Variable {var_type: *formal_type.clone()} );
                format!("{}{{{}}} : {}", id, formal_type, formal_type)
            }
        }
    }
}

impl NamedNode for Stmt {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        use Stmt::*;
        match self {
            Block(ref x) => x.named_unparse(unparser),
            Line(ref x) => format!("{};", x.named_unparse(unparser)),
            VarDecl(ref x) => x.named_unparse(unparser),
        }
    }
}

impl NamedNode for BlockStmt {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        use BlockStmt::*;
        match self {
            While{cond, body} => {
                let cond = cond.named_unparse(unparser);
                unparser.scope += 1;
                let output = format!("while ({}) {{\n{}}}\n", 
                    cond,
                    body.clone().named_unparse_vec(unparser, "")
                );
				unparser.remove_scope(unparser.scope);
                unparser.scope -= 1;
                output 
            },
            If{cond, body} => {
                let cond = cond.named_unparse(unparser);
                unparser.scope += 1;
                let output = format!("if ({}) {{\n{}}}\n", 
                    cond,
                    body.clone().named_unparse_vec(unparser, "")
                );
				unparser.remove_scope(unparser.scope);
                unparser.scope -= 1;
                output
            },
            IfElse{cond, true_branch, false_branch} => {
                let cond = cond.named_unparse(unparser);
                unparser.scope += 1;
                let output = format!("if ({}) {{\n{}}}\nelse {{\n{}}}\n", 
                    cond, 
                    true_branch.clone().named_unparse_vec(unparser, ""),
                    false_branch.clone().named_unparse_vec(unparser, "")
                );
				unparser.remove_scope(unparser.scope);
                unparser.scope -= 1;
                output
            },
        }
    }
}

impl NamedNode for LineStmt {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        use LineStmt::*;
        match self {
            Assign{dest, src} => format!("{} = {}", dest.named_unparse(unparser), src.named_unparse(unparser)),
            PostDec{loc} => format!("{}--", loc.named_unparse(unparser)),
            PostInc{loc} => format!("{}++", loc.named_unparse(unparser)),
            Give{output} => format!("give {}", output.named_unparse(unparser)),
            Take{recipient} => format!("take {}", recipient.named_unparse(unparser)),
            Return{result} => {match result {
                Some(x) => format!("return {}", x.named_unparse(unparser)),
                None => format!("return"),}},
            Exit => format!("today I don't feel like doing any work"),
            Call(ref exp) => format!("{}", exp.named_unparse(unparser)),
        }
    }
}

impl NamedNode for Exp {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        use Exp::*;

        match self {
            True => "true".to_string(),
            False => "false".to_string(),
            Magic => "magic".to_string(),
            UnaryExp(ref exp) => format!("{}", exp.named_unparse(unparser)),
            BinaryExp(ref exp) => format!("{}", exp.named_unparse(unparser)),
            CallExp(ref exp) => format!("{}", exp.named_unparse(unparser)),
            IntLit(ref lit) => lit.to_string(),
            StrLit(ref lit) => lit.to_string(),
            Loc(ref loc) => loc.named_unparse(unparser),
        }
    }
}

impl NamedNode for UnaryExp {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        use UnaryExp::*;
        match self {
            Neg{exp} => format!("-{}", exp.named_unparse(unparser)),
            Not{exp} => format!("!{}", exp.named_unparse(unparser)),
        }
    }
}

impl NamedNode for BinaryExp {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        use BinaryExp::*;
        match self {
            And{lhs, rhs} => format!("{} and {}", lhs.named_unparse(unparser), rhs.named_unparse(unparser)),
            Or{lhs, rhs} => format!("{} or {}", lhs.named_unparse(unparser), rhs.named_unparse(unparser)),
            Equals{lhs, rhs} => format!("{} == {}", lhs.named_unparse(unparser), rhs.named_unparse(unparser)),
            NotEquals{lhs, rhs} => format!("{} != {}", lhs.named_unparse(unparser), rhs.named_unparse(unparser)),
            Greater{lhs, rhs} => format!("{} > {}", lhs.named_unparse(unparser), rhs.named_unparse(unparser)),
            Less{lhs, rhs} => format!("{} < {}", lhs.named_unparse(unparser), rhs.named_unparse(unparser)),
            GreaterEq{lhs, rhs} => format!("{} >= {}", lhs.named_unparse(unparser), rhs.named_unparse(unparser)),
            LessEq{lhs, rhs} => format!("{} <= {}", lhs.named_unparse(unparser), rhs.named_unparse(unparser)),
            Plus{lhs, rhs} => format!("{} + {}", lhs.named_unparse(unparser), rhs.named_unparse(unparser)),
            Minus{lhs, rhs} => format!("{} - {}", lhs.named_unparse(unparser), rhs.named_unparse(unparser)),
            Times{lhs, rhs} => format!("{} * {}", lhs.named_unparse(unparser), rhs.named_unparse(unparser)),
            Divide{lhs, rhs} => format!("{} / {}", lhs.named_unparse(unparser), rhs.named_unparse(unparser)),
        }
    }
}

impl NamedNode for CallExp {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        format!("{}({})", &self.name.named_unparse(unparser), &self.args.clone().named_unparse_vec(unparser, ", "))
    }
}

impl NamedNode for Loc {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        self.named_unparse_helper(unparser, "".to_string())
    }
}

fn get_id_named_string(name: String, unparser: &mut NamedUnparser) -> (String, &mut NamedUnparser) {
    let key = SymbolKey {
        id: name.clone(),
        scope: unparser.scope
    };
    let key2 = SymbolKey {
        id: name.clone(),
        scope: 0
    };
    let mut output = "".to_string();
    match unparser.table.entry(key.clone()) { // <== undo this clone later
        Occupied(x) => output = format!("{}{{{}}}", name, x.get().to_string()),
        Vacant(_) => match unparser.table.entry(key2) {
			Occupied(x) => output = format!("{}{{{}}}", name, x.get().to_string()),
			Vacant(_) =>  {
                // println!("in get_id_named_string {}", key.clone());
                // unparser.print();
                unparser.report_named_error(NameError::UndefinedDecl, name);
                // unparser.report_error(NameError::UndefinedDecl)
            },
		},
    }
    (output, unparser)
}

impl Loc {
    fn type_of(&self) -> &'static str {
        use self::Loc::*;
        match self {
            Id(_) => "Id",
            Loc{ base_class: _ , field_name: _} => "Loc",
        }
    }
	//TODO: FINISH THIS
    fn named_unparse_helper(&self, unparser: &mut NamedUnparser, current: String) -> String {
        use self::Loc::*;
        // println!("{}", current);
        match self {
            Id(ref x) => {
                let x = *x.clone();
                let (output, unparser) = get_id_named_string(x.to_string(), unparser);
                output
            },
            Loc { ref base_class , ref field_name} => {
                match (&base_class).type_of() {
                    "Id" => {
                        let _id = if current == "" {
                            format!("{}--{}", base_class, field_name)
                        } else {
                            format!("{}--{}--{}", base_class, field_name, current)
                        };
                        let base_class = *base_class.clone();
                        let (output, _unparser) = get_id_named_string(base_class.to_string(), unparser);
                        output
                    },
                    "Loc" => {
                        if current == "" {
                            base_class.named_unparse_helper(unparser, format!("{}", field_name))
                        } else {
                            base_class.named_unparse_helper(unparser, format!("{}--{}", field_name, current))
                        }
                    },
                    &_ => {
                        println!("how did we get here");
                        "".to_string()
                    }
                }
            },
        }
    }
}

impl NamedNode for Id {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        let key = SymbolKey {
            id: self.name.clone(),
            scope: unparser.scope
        };
        let key2 = SymbolKey {
            id: self.name.clone(),
            scope: 0
        };
        match unparser.table.entry(key) {
            Occupied(x) => format!("{}{{{}}}", &self.name, x.get().to_string()),
            Vacant(_) => {
                match unparser.table.entry(key2) {
                    Occupied(x) => format!("{}{{{}}}", &self.name, x.get().to_string()),
                    Vacant(_) => {
                        unparser.report_named_error(NameError::UndefinedDecl, self.name.clone());
                        "".to_string()
                    },
                }
            },
        }
    }
}