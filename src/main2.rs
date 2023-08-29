use logos::{Logos};


#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r#"//.*[\n]?"#)]

enum Token {

    //Keywords
    #[token("and", priority = 10)]
    AND,

    #[token("24Kmagic", priority = 10)]
    MAGIC,

    #[token("bool", priority = 10)]
    BOOL,

    #[token("else", priority = 10)]
    ELSE,

    #[regex("false|too hot", priority = 10)]
    FALSE,

    #[token("if", priority = 10)]
    IF,

    #[token("int", priority = 10)]
    INT,

    #[token("give", priority = 10)]
    GIVE,

    #[token("take", priority = 10)]
    TAKE,

    #[token("or", priority = 10)]
    OR,

    #[token("perfect", priority = 10)]
    PERFECT,

    #[token("return", priority = 10)]
    RETURN,

    #[token("class", priority = 10)]
    CLASS,

    #[token("true", priority = 10)]
    TRUE,

    #[token("today I don't feel like doing any work", priority = 10)]
    EXIT,

    #[token("void", priority = 10)]
    VOID,

    #[token("while", priority = 10)]
    WHILE,


    //Identifiers and Literals
    #[regex(r"[a-zA-Z_][0-9a-zA-Z_]*", priority = 9)]
    ID,

    #[regex(r"[0-9]+", priority = 9)]
    INTLITERAL,

    #[regex(r#""(\\[nt"\\]|[^\n"\\])*""#, priority = 9)]
    STRINGLITERAL,


    

    //Symbol Operators
    #[regex("=", priority = 10)]
    ASSIGN,

    #[regex(":", priority = 10)]
    COLON,

    #[regex(",", priority = 10)]
    COMMA,

    #[regex(r#"\+"#, priority = 10)]
    CROSS,

    #[regex("-", priority = 10)]
    DASH,

    #[regex("==", priority = 10)]
    EQUALS,

    #[regex(">", priority = 10)]
    GREATER,

    #[regex(">=", priority = 10)]
    GREATEREQ,

    #[regex(r#"\{"#, priority = 10)]
    LCURLY,

    #[regex("<", priority = 10)]
    LESS,

    #[regex("<=", priority = 10)]
    LESSEQ,

    #[regex(r#"\("#, priority = 10)]
    LPAREN,

    #[regex("!", priority = 10)]
    NOT,

    #[regex("!=", priority = 10)]
    NOTEQUALS,

    #[regex("--", priority = 10)]
    POSTDEC,

    #[regex(r#"\+\+"#, priority = 10)]
    POSTINC,

    #[regex("}", priority = 10)]
    RCURLY,

    #[regex(r#"\)"#, priority = 10)]
    RPAREN,

    #[regex(";", priority = 10)]
    SEMICOL,

    #[regex(r#"/"#, priority = 10)]
    SLASH,

    #[regex(r#"\*"#, priority = 10)]
    STAR,

    
    //illegal and comment
    #[regex(r#"[^\s]"#, priority = 1)]
    ILLEGAL

}

fn main() {
    let mut lex = Token::lexer("457495 == + {} () ;!=/,/,24Kmagic 5 ~~~ 42k 7 5m too hot//TEST");


    for result in lex {
        // match result.unwrap() {
        //     Token::Number(value) => println!("value: {}", value),
        //     // Token::STRINGLIT(value) => println!("value: {}", value),
        //     // _ => println!("Something else..."),
        // }
        match result {
            Ok(token) => println!("{:#?}", token),
            Err(()) => panic!("some error occured."),
        }
    }

}