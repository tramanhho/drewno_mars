#![allow(dead_code)]

mod ast;
use crate::parser::ast::*;

use std::collections::HashMap;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar, "/parser/grammar.rs");

pub fn unparse(prog: Box<Program>) -> String {
    let input = format!("{:?}", prog);
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
    let mut symbol_table: HashMap<String, Symbol> = HashMap::new();

    Ok("".to_string())
}

pub enum ASTNode {
    Exp(Exp),
    Program(Program),
    Stmt(Stmt),
    Type(Type),
}

struct Symbol {
    sym_type: Type,
    init_val: Option<VariableValue>
}

enum VariableValue {
    Int(i32),
    Bool(bool),
    Text(String),
}

// gameplan: 
// recurse through whole structure, finding id nodes specifically. once an id node is found 
// we add it to the symbol table along with its type + init val if it has one 
// might have to query upwards for type/val/etc. 
// if we find a conflict, that means we are redoing stuff. then we see if there is weird stuff happening.
// thumbs up emoji
fn recurse_parse_tree(node: Box<ASTNode>) {
    
}

// fn process_id_node<T>(id_node: Box<Id>, symbol_table: HashMap<String, Symbol<T>>) {

// }