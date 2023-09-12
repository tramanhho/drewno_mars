use logos::Logos;
use std::fmt;
//use std::num::ParseIntError;

// #[derive(Debug, Default, Logos, Clone, PartialEq)]
// pub enum LexingError {
//     InvalidInteger(String),
//
//     #[default]
//     #[regex(r#"[^\s]"#, priority = 1)]
//     Illegal,
// }

// /// Error type returned by calling `lex.slice().parse()` to i32.
// impl From<ParseIntError> for LexingError {
//     fn from(err: ParseIntError) -> Self {
//         use std::num::IntErrorKind::*;
//         match err.kind() {
//             PosOverflow | NegOverflow => LexingError::InvalidInteger("overflow error".to_owned()),
//             _ => LexingError::InvalidInteger("other error".to_owned()),
//         }
//     }
// }

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub token_type: TokenType,
    pub value: String
}

impl Token {
    pub fn new(lex: &mut logos::Lexer<'_, TokenType>, token_type: TokenType ) -> Token {
        Token {
            start: lex.span().start + 1,
            end: lex.span().end + 1,
            value: match &token_type {
                TokenType::ID | 
                TokenType::INTLITERAL | 
                TokenType::STRINGLITERAL | 
                TokenType::Illegal
                => lex.slice().to_string(),
                _ => "".to_string()
            },
            token_type: token_type,
        }
    }
}

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
//#[logos(error = LexingError)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r#"//.*[\n]?"#)]
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
    #[regex(r"[a-zA-Z_][0-9a-zA-Z_]*", priority = 2)]
    ID,

    // #[regex(r"[0-9]+", priority = 2, callback = |lex| lex.slice().parse())]
    // INTLITERAL(i32),

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
    Illegal,

    //string literal with bad escape sequence ignored
    #[regex(r#""((\\[nt"\\]|[^\n"\\])*(\\[^nt"\\])(\\[nt"\\]|[^\n"\\])*)+""#, priority = 3)]
    STRINGLITERALBadEscape,

    //unterminated string literal ignored \n \t \" \\
    //#[regex(r#""(\\[nt"\\]|[^\n"\\])*"#, priority = 3)]
    #[regex(r#""[^\s]"#, priority = 3)]
    STRINGLITERALUnterminated,
    
    //untermintated string literal with bad escape sequence ignored
    #[regex(r#""((\\[nt"\\]|[^\n"\\])*(\\[^nt"\\])(\\[nt"\\]|[^\n"\\])*)+"#, priority = 3)]
    STRINGLITERALUnterminatedBadEscape,

    // //interger literal overflow (int max is 2147483647)
    #[regex(r#"([1-9][0-9]{10}|[3-9][0-9]{9}|2[2-9][0-9]{8}|21[5-9][0-9]{7}|214[8-9][0-9]{6}|2147[5-9][0-9]{5}|21474[9][0-9]{4}|214748[4-9][0-9]{3}|2147483[7-9][0-9]{2}|21474836[5-9][0-9]|214748364[8-9])([0-9])*"#, priority = 3)]
    INTLITERALOverflow,
}
