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

    pub enum ParserType {
        Type,
        PrimType,

        FormalList,
        FormalDecl,
        
        // Exp,
        // ActualsList,
        CallExp,
        Exp,
        Term,
        Loc,
        Id,
    }

    pub enum InputType {
        Good,
        Bad,
    }

    fn test_inputs(good_tests: Vec<&'static str>, bad_tests: Option<Vec<&'static str>>, parse: &ParserType) {
        test_inputs_helper(good_tests, parse, &InputType::Good);
        match bad_tests {
            Some(t) => test_inputs_helper(t, parse, &InputType::Bad),
            None => ()
        }
    }

    fn test_inputs_helper(tests: Vec<&'static str>, parse: &ParserType, input: &InputType) {
        for t in tests.iter() {
            let mut result = match parse {
                ParserType::Type  => TypeParser::new() .parse(Lexer::new(t)).is_ok(),
                ParserType::PrimType  => PrimTypeParser::new() .parse(Lexer::new(t)).is_ok(),

                ParserType::FormalList  => FormalsListParser::new() .parse(Lexer::new(t)).is_ok(),
                ParserType::FormalDecl  => FormalDeclParser::new() .parse(Lexer::new(t)).is_ok(),
                
                // ParserType::ActualsList  => ActualsListParser::new() .parse(Lexer::new(t)).is_ok(),
                ParserType::CallExp  => CallExpParser::new() .parse(Lexer::new(t)).is_ok(),
                ParserType::Exp  => ExpParser::new() .parse(Lexer::new(t)).is_ok(),
                ParserType::Term => TermParser::new().parse(Lexer::new(t)).is_ok(),
                ParserType::Loc  => LocParser::new() .parse(Lexer::new(t)).is_ok(),
                ParserType::Id   => IdParser::new()  .parse(Lexer::new(t)).is_ok(),
            };
            
            let err_string : String;
            match input {
                InputType::Good => {
                    err_string = format!("\nThe following input did not pass the parser as intended:\n{}\n", t)
                },
                InputType::Bad => {
                    result = !result;
                    err_string = format!("\nThe following input did not fail the parser as intended:\n{}\n", t)
                }
            };

            assert!(result, "{err_string}");
        }
    }

    // #[test]
    // fn parse_program() {
    //     let program_good = [
    //         "a : bool = ! true;",
    //         "a : bool = !!too hot;",
    //         "myClass:class {
    //             a:int = 12;
    //             b:int;
    //             c:perfect bool;
    //         };",
    //         "myClass:class {};",
    //     ];

    //     let program_bad = [
    //         "main();",
    //         "myClass:class {
    //             a:int = 12
    //             b:int
    //             c:perfect bool
    //         };",
    //     ];
    //     for prog in program_good.iter() {
    //         // println!("{:?}\n", ProgramParser::new().parse(Lexer::new(prog)).unwrap());
    //         assert!(
    //             ProgramParser::new().parse(Lexer::new(prog)).is_ok(),
    //             "\nThe following input did not pass the parser as intended:\n{}\n", prog, 
    //         ); 
    //     }

    //     for prog in program_bad.iter() {
    //         assert!(
    //             ProgramParser::new().parse(Lexer::new(prog)).is_err(),
    //             "\nThe following input did not fail the parser as intended:\n{:?}\n", prog, 
    //         );
    //     }
    // }

    #[test]
    fn parse_type() {
        let type_good = [
            "int",
            "bool",
            "void",
            "perfect int",
            "perfect owo",
            "perfect _nya",
        ];

        let type_bad = [
            "perfect",
            "perfect 123",
        ];
        
        for t in type_good.iter() {
            assert!(
                TypeParser::new().parse(Lexer::new(t)).is_ok(),
                "\nThe following input did not pass the parser as intended:\n{}\n", t, 
            );
            println!("{:?}", TypeParser::new().parse(Lexer::new(t)).unwrap());
        }

        for t in type_bad.iter() {
            assert!(TypeParser::new().parse(Lexer::new(t)).is_err());
        }
    }

    #[test]
    fn parse_prim_type() {
        let prim_type = [
            "int",
            "bool",
            "void",
        ];
        
        for pt in prim_type.iter() {
            assert!(
                PrimTypeParser::new().parse(Lexer::new(pt)).is_ok(),
                "\nThe following input did not pass the parser as intended:\n{}\n", pt, 
            );
            println!("{:?}", PrimTypeParser::new().parse(Lexer::new(pt)).unwrap());
        }
    }

    
    #[test]
    fn parse_formal_list() {
        let fd_good = [
            "a: int, b: void, c: owo",
            "_owo : int",
            "",
        ];
        
        for fd in fd_good.iter() {
            assert!(
                FormalsListParser::new().parse(Lexer::new(fd)).is_ok(),
                "\nThe following input did not pass the parser as intended:\n{}\n", fd, 
            );
            println!("{:?}", FormalsListParser::new().parse(Lexer::new(fd)).unwrap());
        }
    }

    #[test]
    fn parse_formal_decl() {
        let fd_good = [
            "meow : perfect _nya",
            "_owo : int",
        ];
        
        for fd in fd_good.iter() {
            assert!(
                FormalDeclParser::new().parse(Lexer::new(fd)).is_ok(),
                "\nThe following input did not pass the parser as intended:\n{}\n", fd, 
            );
            println!("{:?}", FormalDeclParser::new().parse(Lexer::new(fd)).unwrap());
        }
    }

    // #[test]
    // fn parse_stmt() {
    //     let stmt_good = [
    //         "a--a = magic",
    //         "a--b--c--d--e = magic",
    //         "a--",
    //         "a++",
    //         "give magic",
    //         "take a--b",
    //         "take meow_on",
    //         "return true",
    //         "return",
    //         "today I don't feel like doing any work",
    //         "abc()",
    //         "owo(uwu)",
    //     ];

    //     let stmt_bad = [
    //         "take a-b",
    //         "abc--abc a--c",
    //     ];
        
    //     for stmt in stmt_good.iter() {
    //         assert!(
    //             StmtParser::new().parse(Lexer::new(stmt)).is_ok(),
    //             "\nThe following input did not pass the parser as intended:\n{}\n", stmt, 
    //         );
    //     }

        // for stmt in stmt_bad.iter() {
        //     assert!(
        //         StmtParser::new().parse(Lexer::new(stmt)).is_err(),
        //         "\nThe following input did not fail the parser as intended:\n{:?}\n", stmt, 
        //     );
        // }
    // }

    // #[test]
    // fn parse_exp() {
    //     let exp_good = [
    //         r#"not (123 + "abc")"#,
    //         "-nice",
    //         "!!too hot",
    //         "abc - abc - abc",
    //         "abc + abc",
    //         "abc * abc",
    //         "abc / abc",
    //         "abc and abc",
    //         "abc or abc",
    //         "abc == abc",
    //         "abc != abc",
    //         "abc > abc",
    //         "abc >= abc",
    //         "abc < abc",
    //         "abc <= abc",
    //         "abc",
    //     ];

    //     let exp_bad = [
    //         "abc abc",
    //     ];

    //     for exp in exp_good.iter() {
    //         assert!(ExpParser::new().parse(Lexer::new(exp)).is_ok());    
    //     }

    //     for exp in exp_bad.iter() {
    //         assert!(ExpParser::new().parse(Lexer::new(exp)).is_err());
    //     }
    // }

    #[test]
    fn parse_call_exp() {
        let call_exp_good = vec![
            "___ (a, b, c)",
            "_ (true, false, too hot)",
            "a (a/3+a)",
        ];

        let call_exp_bad = vec![
            "a (",
            "a )",
            "a (",
            "a (",
            "abc",
        ];

        test_inputs(call_exp_good, Some(call_exp_bad), &ParserType::CallExp);
    }

    #[test]
    fn parse_actuals_list() {
        let actuals_lists_good = vec![
            "a--b--c--d",
            "true, true, true, true, true",
            r#"true, "pleading", 54353, magic, (too hot)"#,
            "a+b, too hot",
        ];

        let actuals_lists_bad = vec![
            ",",
            "true, ",
        ];

        // test_inputs(actuals_lists_good, Some(actuals_lists_bad), &ParserType::ActualsList);
    }

    #[test]
    fn parse_term() {
        let terms_good = vec![
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
            "a-b-c-d-e-f",
        ];

        let terms_bad = vec![
            "2147483648",
            r#""meow meow meow"#,
            r#""heewoo!??! \g""#,
            r#""\g"#,
        ];
        
        test_inputs(terms_good, Some(terms_bad), &ParserType::Term);
    }

    #[test]
    fn parse_loc() {
        let locs_good = vec![
            "a--b--c--d",
            "owo--owo",
            "_nya_",
        ];

        let locs_bad = vec![
            "a--b--c--d--",
            "--",
        ];
        
        test_inputs(locs_good, Some(locs_bad),&ParserType::Loc);
    }

    #[test]
    fn parse_id() {
        let ids_good = vec![
            "a",
            "abc",
            "fsndjkfngjkreng",
            "_nice",
            "___",
            "ffds215345",
        ];

        let ids_bad = vec![
            "0_abc",
            "",
            ":pleading:",
        ];
        test_inputs(ids_good, Some(ids_bad), &ParserType::Id);
    }
    
}