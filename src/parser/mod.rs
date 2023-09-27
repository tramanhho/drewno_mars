#![allow(non_camel_case_types)]
#![allow(dead_code)]

use lalrpop_util::lalrpop_mod;
pub mod ast;
pub mod lexer;
lalrpop_mod!(pub grammar, "/parser/grammar.rs");


#[cfg(test)]
mod tests {
    use super::lexer::Lexer;
    use super::grammar::*;

    #[test]
    fn parse_program() {
        let program_good = [
            "a : bool = ! true;",
            "a : bool = !!too hot;",
            "myClass:class {
                a:int = 12;
                b:int;
                c:perfect bool;
            };",
            "myClass:class {};",
        ];

        let program_bad = [
            "main();",
            "myClass:class {
                a:int = 12
                b:int
                c:perfect bool
            };",
        ];

        for prog in program_good.iter() {
            // println!("{:?}\n", ProgramParser::new().parse(Lexer::new(prog)).unwrap());
            assert!(
                ProgramParser::new().parse(Lexer::new(prog)).is_ok(),
                "\nThe following input did not pass the parser as intended:\n{}\n", prog, 
            ); 
        }

        for prog in program_bad.iter() {
            assert!(
                ProgramParser::new().parse(Lexer::new(prog)).is_err(),
                "\nThe following input did not fail the parser as intended:\n{:?}\n", prog, 
            );
        }
    }

    #[test]
    fn parse_stmt() {
        let stmt_good = [
            "a--a = magic",
            "a--b--c--d--e = magic",
            "a--",
            "a++",
            "give magic",
            "take a--b",
            "take meow_on",
            "return true",
            "return",
            "today I don't feel like doing any work",
            "abc()",
            "owo(uwu)",
        ];

        let stmt_bad = [
            "take a-b",
            "abc--abc a--c",
        ];
        
        for stmt in stmt_good.iter() {
            assert!(
                StmtParser::new().parse(Lexer::new(stmt)).is_ok(),
                "\nThe following input did not pass the parser as intended:\n{}\n", stmt, 
            );
        }

        for stmt in stmt_bad.iter() {
            assert!(
                StmtParser::new().parse(Lexer::new(stmt)).is_err(),
                "\nThe following input did not fail the parser as intended:\n{:?}\n", stmt, 
            );
        }
    }

    #[test]
    fn parse_exp() {
        let exp_good = [
            r#"not (123 + "abc")"#,
            "-nice",
            "!!too hot",
            "abc - abc - abc",
            "abc + abc",
            "abc * abc",
            "abc / abc",
            "abc and abc",
            "abc or abc",
            "abc == abc",
            "abc != abc",
            "abc > abc",
            "abc >= abc",
            "abc < abc",
            "abc <= abc",
            "abc",
        ];

        let exp_bad = [
            "abc abc",
        ];

        for exp in exp_good.iter() {
            assert!(ExpParser::new().parse(Lexer::new(exp)).is_ok());    
        }

        for exp in exp_bad.iter() {
            assert!(ExpParser::new().parse(Lexer::new(exp)).is_err());
        }
    }

    #[test]
    fn parse_call_exp() {
        let call_exp_good = [
            "___ (a, b, c)",
            "_ (true, false, too hot)",
            "a (a/3+a)",
        ];

        let call_exp_bad = [
            "a (",
            "a )",
            "a (",
            "a (",
            "abc",
        ];

        for call_exp in call_exp_good.iter() {
            assert!(CallExpParser::new().parse(Lexer::new(call_exp)).is_ok());    
        }

        for call_exp in call_exp_bad.iter() {
            assert!(CallExpParser::new().parse(Lexer::new(call_exp)).is_err());
        }
    }

    #[test]
    fn parse_actuals_list() {
        let actuals_lists_good = [
            "a--b--c--d",
            "true, true, true, true, true",
            r#"true, "pleading", 54353, magic, (too hot)"#,
            "a+b, too hot",
        ];

        let actuals_lists_bad = [
            ",",
            "true, ",
        ];

        for actual_list in actuals_lists_good.iter() {
            assert!(ActualsListParser::new().parse(Lexer::new(actual_list)).is_ok());    
        }

        for actual_list in actuals_lists_bad.iter() {
            assert!(ActualsListParser::new().parse(Lexer::new(actual_list)).is_err());
        }
    }

    #[test]
    fn parse_term() {
        let terms = [
            "a--b--c--d",
            "5273985",
            r#""meow meow meow \t \n""#,
            "true",
            "false",
            "too hot",
            "magic",
            "a ()",
            "a (a-b)",
            "(!!too hot)",
            "(a-b)",
        ];

        for term in terms.iter() {
            assert!(
                TermParser::new().parse(Lexer::new(term)).is_ok(),
                "\nThe following input did not pass the parser as intended:\n{}\n", term, 
            );
            println!("{:?}", TermParser::new().parse(Lexer::new(term)).unwrap());
        }
    }

    #[test]
    fn parse_loc() {
        let locs_good = [
            "a--b--c--d",
            "owo--owo",
            "_nya_",
        ];

        let locs_bad = [
            "a--b--c--d--",
            "--",
        ];
        
        for loc in locs_good.iter() {
            assert!(LocParser::new().parse(Lexer::new(loc)).is_ok());
        }

        for loc in locs_bad.iter() {
            assert!(LocParser::new().parse(Lexer::new(loc)).is_err());
        }
    }

    #[test]
    fn parse_id() {
        let ids_good = [
            "a",
            "abc",
            "fsndjkfngjkreng",
            "_nice",
            "___",
            "ffds215345",
        ];

        let ids_bad = [
            "0_abc",
            "",
            ":pleading:",
        ];

        for id in ids_good.iter() {
            assert!(IdParser::new().parse(Lexer::new(id)).is_ok());
            println!("{:?}", IdParser::new().parse(Lexer::new(id)).unwrap());
        }

        for id in ids_bad.iter() {
            assert!(IdParser::new().parse(Lexer::new(id)).is_err());
        }
    }
    
}