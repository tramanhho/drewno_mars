#![allow(dead_code)]

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar, "/parser/grammar.rs");

mod ast;
use crate::parser::ast::*;

mod named_unparser;
use crate::parser::named_unparser::*;
use crate::parser::named_unparser::named_node::NamedNode;

use std::collections::HashMap;

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


pub fn named_unparse(prog: Box<Program>) -> String {
    let mut unparser: NamedUnparser = NamedUnparser {
        scope: 0,
        table: HashMap::new(),
        classes: HashMap::new(),
        error: false
    };
    let named_unparse = add_tabs(prog.named_unparse(&mut unparser));
    println!("{}", unparser);
    if !unparser.error {
        named_unparse
    } else {
        eprintln!("Name Analysis Failed");
        "".to_string()
    }
}
