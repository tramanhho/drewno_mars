use logos::Logos;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum LexingError {
    IntliteralOverflow,    
    StringliteralBadEscape,
    StringliteralUnterminated,
    StringliteralUnterminatedBadEscape,
    Illegal(String), // needed to print which char flagged this

    #[default]
    NonAsciiCharacter,
}

/// Error type returned by calling `lex.slice().parse()` to i32.
impl From<ParseIntError> for LexingError {
    fn from(_err: ParseIntError) -> Self {
        LexingError::IntliteralOverflow
    }
}

impl fmt::Display for LexingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexingError::IntliteralOverflow => write!(f, "Integer literal overflow"),
            LexingError::StringliteralBadEscape => write!(f, "String literal with bad escape sequence detected"),
            LexingError::StringliteralUnterminated => write!(f, "Unterminated string literal detected"),
            LexingError::StringliteralUnterminatedBadEscape => write!(f, "Unterminated string literal with bad escape sequence detected"),
            LexingError::Illegal(v) => write!(f, "Illegal character: {}", v),
            LexingError::NonAsciiCharacter => write!(f, "Non ASCII character detected"),
        }
    }
}

#[derive(Debug, Logos, PartialEq, Clone)]
#[logos(error = LexingError)]
#[logos(skip r"[ \s]+")]
#[logos(skip r#"//.*[\n\r]?"#)]
pub enum TokenType {
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
    #[regex(r"[a-zA-Z_][0-9a-zA-Z_]*", priority = 2, callback = |lex| lex.slice().parse().ok())]
    ID(String),

    #[regex(r"[0-9]+", priority = 2, callback = |lex| lex.slice().parse())]
    INTLITERAL(i32),

    #[regex(r#""(\\[nt"\\]|[^\n"\\])*""#, priority = 2, callback = |lex| lex.slice().parse().ok())]
    STRINGLITERAL(String),

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

    //string literal with bad escape sequence ignored
    #[regex(
        r#""((\\[nt"\\]|[^\n"\\])*(\\[^nt"\\])(\\[nt"\\]|[^\n"\\])*)+""#, 
        priority = 3, 
        callback = |_| Err(LexingError::StringliteralBadEscape)
    )]

    //unterminated string literal ignored \n \t \" \\
    #[regex(
        r#""(\\[nt"\\]|[^\n"\\])*"#, 
        priority = 3, 
        callback = |_| Err(LexingError::StringliteralUnterminated)
    )]
    
    //untermintated string literal with bad escape sequence ignored
    #[regex(
        r#""((\\[nt"\\]|[^\n"\\])*(\\[^nt"\\])(\\[nt"\\]|[^\n"\\])*)+"#, 
        priority = 3, 
        callback = |_| Err(LexingError::StringliteralUnterminatedBadEscape)
    )]

    //illegal
    #[regex(
        r#"[^\s]"#, 
        priority = 1, 
        callback = |lex| Err(LexingError::Illegal(lex.slice().parse().ok().unwrap()))
    )]
    Null
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::ID(v) =>            write!(f, "ID:{}", v),
            TokenType::STRINGLITERAL(v) => write!(f, "STRINGLITERAL:{}", v),
            TokenType::INTLITERAL(v) =>       write!(f, "INTLITERAL:{}", v),
            _ =>                                    write!(f, "{:?}", self)
        }
    }
}