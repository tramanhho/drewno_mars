#![allow(dead_code)]

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar, "/parser/grammar.rs");

pub mod ast;
use ast::Program;
use crate::format::add_tabs;

pub fn unparse(prog: Box<Program>) -> String {
    add_tabs(prog.to_string())
}

