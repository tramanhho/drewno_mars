#![allow(dead_code)]

mod ast;
use crate::parser::ast::*;

use std::collections::HashMap;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar, "/parser/grammar.rs");

pub fn unparse(prog: Box<Program>) -> String {
    add_tabs(prog.to_string())
}

fn add_tabs(input: String) -> String {
    let text = input.lines();
    let mut output : String = "".to_string();
    let mut tabs = 0;

    for mut line in text {
        line = line.trim();
        let first_char = &line.chars().next();
        let last_char = &line.chars().last();

        match first_char {
            Some(x) => match x {
                '}' => {if tabs > 0 { tabs -= 1; }},
                _    => ()
            },
            None => (),
        }

        output.push_str(&"    ".repeat(tabs));

        match last_char {
            Some(x) => match x {
                '{' => tabs += 1,
                _   => ()
            },
            None => (),
        }

        output.push_str(&line);
        output.push_str("\n");
    }

    output
}


pub fn named_unparse(prog: Box<Program>) -> Result<String, &'static str> {
    let mut unparser: NamedUnparser = NamedUnparser {
        scope: 0,
        table: HashMap::new()
    };
    println!("{}", prog.named_unparse(&mut unparser));
    Ok("".to_string())
}

impl NamedUnparser {
    fn add_entry(&mut self, id: String, kind: SymbolKind) {
        let key : SymbolKey = SymbolKey {
            id: id,
            scope: self.scope,
        };

        self.table.insert(key, kind);
    }
}

struct NamedUnparser {
    scope: u8,
    table: HashMap<SymbolKey, SymbolKind>
}

#[derive(Eq, Hash, PartialEq)]
struct SymbolKey {
    id: String,
    scope: u8 // int incremented corresponding to nesting level, 0 = global
}

impl SymbolKey {
    
}
enum SymbolKind {
    Variable,
    Class,
    Function {args: Vec<Type>, ret: Option<Type>}
    // Function
}

trait Node {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String;
}

impl Node for ast::Program {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        let mut output : String = "".to_owned();
        for arg in self.globals.iter() {
            output = output + &arg.named_unparse(unparser);
        }
        output
    }
}

impl Node for ast::Decl {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        use Decl::*;

        match *self {
            VarDecl(ref x) => x.named_unparse(unparser),
            ClassDecl(ref x) => x.named_unparse(unparser),
            FnDecl(ref x) => x.named_unparse(unparser),
        }
    }
}

impl Node for ast::VarDecl {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        unparser.add_entry(self.id.to_string(), SymbolKind::Variable);
        match &self.init_val {
            Some(v) => format!("{}{{{}}} : {} = {};\n", &self.id, &self.var_type, &self.var_type, v),
            None => format!("{} : {};\n", &self.id, &self.var_type),
        }
    }
}

impl Node for ast::ClassDecl {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        "".to_string()
    }
}

impl Node for ast::FnDecl {
    fn named_unparse(&self, unparser: &mut NamedUnparser) -> String {
        // unparser.add_entry(self.id.to_string(), SymbolKind::Function{args: self.args, ret: self.ret});
        // write!(fmt, r#"{} : ({}) {} {{\n{}}}\n"#, &self.id, fmt_vec_commas(&self.args), &self.ret, fmt_vec(&self.body))
        //to do : go into formaldecl to make the function symbol (only need id + type)
    }
}