use lalrpop_util::lalrpop_mod;
pub mod ast;

lalrpop_mod!(pub grammar, "/parser/grammar.rs");


pub fn parse_token_stream() {

}