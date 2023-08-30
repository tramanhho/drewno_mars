use logos::Logos;
use std::ops::Range;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r#"//.*[\n]?"#)]
enum Token {
    //Keywords
    #[token("and", priority = 3)]
    AND,

    #[token("24Kmagic", priority = 3)]
    MAGIC,

    #[token("bool", priority = 3)]
    BOOL,

    #[token("else", priority = 3)]
    ELSE,

    #[regex("false|too hot", priority = 3)]
    FALSE,

    #[token("if", priority = 3)]
    IF,

    #[token("int", priority = 3)]
    INT,

    #[token("give", priority = 3)]
    GIVE,

    #[token("take", priority = 3)]
    TAKE,

    #[token("or", priority = 3)]
    OR,

    #[token("perfect", priority = 3)]
    PERFECT,

    #[token("return", priority = 3)]
    RETURN,

    #[token("class", priority = 3)]
    CLASS,

    #[token("true", priority = 3)]
    TRUE,

    #[token("today I don't feel like doing any work", priority = 3)]
    EXIT,

    #[token("void", priority = 3)]
    VOID,

    #[token("while", priority = 3)]
    WHILE,

    //Identifiers and Literals
    #[regex(r"[a-zA-Z_][0-9a-zA-Z_]*", priority = 2)]
    ID,

    #[regex(r"[0-9]+", priority = 2)]
    INTLITERAL,

    #[regex(r#""(\\[nt"\\]|[^\n"\\])*""#, priority = 2)]
    STRINGLITERAL,

    //Symbol Operators
    #[regex("=", priority = 3)]
    ASSIGN,

    #[regex(":", priority = 3)]
    COLON,

    #[regex(",", priority = 3)]
    COMMA,

    #[regex(r#"\+"#, priority = 3)]
    CROSS,

    #[regex("-", priority = 3)]
    DASH,

    #[regex("==", priority = 3)]
    EQUALS,

    #[regex(">", priority = 3)]
    GREATER,

    #[regex(">=", priority = 3)]
    GREATEREQ,

    #[regex(r#"\{"#, priority = 3)]
    LCURLY,

    #[regex("<", priority = 3)]
    LESS,

    #[regex("<=", priority = 3)]
    LESSEQ,

    #[regex(r#"\("#, priority = 3)]
    LPAREN,

    #[regex("!", priority = 3)]
    NOT,

    #[regex("!=", priority = 3)]
    NOTEQUALS,

    #[regex("--", priority = 3)]
    POSTDEC,

    #[regex(r#"\+\+"#, priority = 3)]
    POSTINC,

    #[regex("}", priority = 3)]
    RCURLY,

    #[regex(r#"\)"#, priority = 3)]
    RPAREN,

    #[regex(";", priority = 3)]
    SEMICOL,

    #[regex(r#"/"#, priority = 3)]
    SLASH,

    #[regex(r#"\*"#, priority = 3)]
    STAR,

    //illegal
    #[regex(r#"[^\s]"#, priority = 1)]
    ILLEGAL
}

fn error_msg(token_type:Token) -> &'static str {
    match token_type {
        // Token::OVERFLOW => "Integer literal overflow",
        Token::ILLEGAL => "Illegal character ",
        // Token::BADESC => "String literal with bad escape sequence detected",
        // Token::UNTERM => "Unterminated string literal detected",
        // Token::UNTERMBADESC => "Unterminated string literal with bad escape sequence detected",
        _ => ""
    }
}

fn error_handler(token_type : Token, val: &'static str, col: i32, span: &Range<usize> ) -> String {
    let msg = error_msg(token_type);
    format!("FATAL [{},{}] - [{},{}]: {}{}", col, span.start, col, span.end, msg, val)
}