use super::grammar::*;
use crate::scanner::lexer::Lexer;

pub enum ParserType {
    Program,

    Decl,
    VarDecl,

    Type,
    PrimType,

    ClassDecl,
    ClassBody,

    FnDecl,

    FormalsList,
    FormalDecl,

    StmtList,
    BlockStmt,
    LineStmt,

    Exp,
    ActualsList,
    CallExp,
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
            ParserType::Program  => ProgramParser::new().parse(Lexer::new(t)).is_ok(),

            ParserType::Decl  => DeclParser::new().parse(Lexer::new(t)).is_ok(),
            ParserType::VarDecl  => VarDeclParser::new().parse(Lexer::new(t)).is_ok(),
            ParserType::FnDecl   => FnDeclParser::new().parse(Lexer::new(t)).is_ok(),
            ParserType::ClassDecl  => ClassDeclParser::new().parse(Lexer::new(t)).is_ok(),
            ParserType::ClassBody  => ClassBodyParser::new().parse(Lexer::new(t)).is_ok(),

            ParserType::Type  => TypeParser::new().parse(Lexer::new(t)).is_ok(),
            ParserType::PrimType  => PrimTypeParser::new().parse(Lexer::new(t)).is_ok(),

            ParserType::FormalsList  => FormalsListParser::new().parse(Lexer::new(t)).is_ok(),
            ParserType::FormalDecl  => FormalDeclParser::new().parse(Lexer::new(t)).is_ok(),
            
            ParserType::StmtList  => StmtListParser::new().parse(Lexer::new(t)).is_ok(),
            ParserType::BlockStmt  => BlockStmtParser::new().parse(Lexer::new(t)).is_ok(),
            ParserType::LineStmt  => LineStmtParser::new().parse(Lexer::new(t)).is_ok(),
            ParserType::CallExp  => CallExpParser::new().parse(Lexer::new(t)).is_ok(),
            ParserType::Exp  => ExpParser::new().parse(Lexer::new(t)).is_ok(),
            ParserType::ActualsList  => ActualsListParser::new().parse(Lexer::new(t)).is_ok(),
            ParserType::Term => TermParser::new().parse(Lexer::new(t)).is_ok(),
            ParserType::Loc  => LocParser::new().parse(Lexer::new(t)).is_ok(),
            ParserType::Id   => IdParser::new().parse(Lexer::new(t)).is_ok(),
        };
        
        let err_string : String;
        match input {
            InputType::Good => {
                err_string = format!("\nThe following input did not pass the parser as intended:\n\t{}\n", t)
            },
            InputType::Bad => {
                result = !result;
                err_string = format!("\nThe following input did not fail the parser as intended:\n\t{}\n", t)
            }
        };
        assert!(result, "{err_string}");
    }
}

#[test]
fn parse_program() {
    let program_good = vec![
        "a : bool = true;",
        "a : bool = too hot;",
        "myClass:class {
            a:int = 12;
            b:int;
            c:perfect bool;
        };",
        "myClass:class {};",
    ];

    let program_bad = vec![
        "main();",
        "myClass:class {
            a:int = 12
            b:int
            c:perfect bool
        };",
    ];
    
    test_inputs(program_good, Some(program_bad), &ParserType::Program);
}


#[test]
fn parse_decl() {
    let d_good = vec![
        "a : int;",
        "a : (a: int) void {}",
        "a : class {};",
    ];

    let d_bad = vec![
        "int : a;",
    ];
    
    test_inputs(d_good, Some(d_bad), &ParserType::Decl);
}

#[test]
fn parse_var_decl() {
    let vd_good = vec![
        "a : int;",
        "a : int = 123;",
    ];

    let vd_bad = vec![
        "a : int =",
    ];
    
    test_inputs(vd_good, Some(vd_bad), &ParserType::VarDecl);
}

#[test]
fn parse_fn_decl() {
    let fd_good = vec![
        "main : () void {}",
        "a : (b: int, c: void) bool {}",
        "a : (b: int, c: void) void { return; }",
    ];

    let fd_bad = vec![
        "a : () int",
        "a : (b: int = 3) void {}",
    ];
    
    test_inputs(fd_good, Some(fd_bad), &ParserType::FnDecl);
}

#[test]
fn parse_type() {
    let type_good = vec![
        "int",
        "bool",
        "void",
        "perfect int",
        "perfect owo",
        "perfect _nya",
    ];

    let type_bad = vec![
        "perfect",
        "perfect 123",
    ];
    
    test_inputs(type_good, Some(type_bad), &ParserType::Type);
}

#[test]
fn parse_prim_type() {
    let prim_type = vec![
        "int",
        "bool",
        "void",
    ];
    
    test_inputs(prim_type, None, &ParserType::PrimType);
}

#[test]
fn parse_class_decl() {
    let cd_good = vec![
        "a : class {};",
        "a : class {b : int; c : int;};",
    ];
    
    test_inputs(cd_good, None, &ParserType::ClassDecl);
}


#[test]
fn parse_formal_list() {
    let fd_good = vec![
        "a: int, b: void, c: owo",
        "_owo : int",
        "",
    ];
    
    test_inputs(fd_good, None, &ParserType::FormalsList);
}

#[test]
fn parse_formal_decl() {
    let fd_good = vec![
        "meow : perfect _nya",
        "_owo : int",
    ];
    
    test_inputs(fd_good, None, &ParserType::FormalDecl);
}

#[test]
fn parse_stmt_list() {
    let stmt_list_good = vec![
        "return;",
        "return; return;",
        "return; if (true) {}",
    ];

    let stmt_list_bad = vec![
        "return"
    ];
    //println!("{:?}", StmtListParser::new().parse(Lexer::new("return; return;")));
    test_inputs(stmt_list_good, Some(stmt_list_bad), &ParserType::StmtList);
}

#[test]
fn parse_line_stmt() {
    let stmt_good = vec![
        "a--a = magic",
        "a--b--c--d--e = magic",
        "a--",
        "a++",
        "give magic",
        "take a",
        "take meow_on",
        "return true",
        "return",
        "today I don't feel like doing any work",
        "abc()",
        "owo(uwu)",
    ];

    let stmt_bad = vec![
        "take a-b",
        "abc--abc a--c",
    ];


    // println!("{:?}", LineStmtParser::new().parse(Lexer::new("a--a = magic")));
    test_inputs(stmt_good, Some(stmt_bad), &ParserType::LineStmt);
}

#[test]
fn parse_exp() {
    let exp_good = vec![
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
        "a-b-c-d-e-f",
    ];

    let exp_bad = vec![
        "abc abc",
    ];
    test_inputs(exp_good, Some(exp_bad), &ParserType::Exp);
}

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
    test_inputs(actuals_lists_good, Some(actuals_lists_bad), &ParserType::ActualsList);
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