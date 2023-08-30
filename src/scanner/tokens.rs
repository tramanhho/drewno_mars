
mod position;
use crate::scanner::position::Position;


pub struct Token<T: std::cmp::PartialOrd> {
    pos: Position,
    type: TokenType,
    value: Option<T>
}

enum TokenType {
    AND,
    ASSIGN,
    BOOL,
    CLASS,
    COMMA,
    CROSS,
    DASH,
    ELSE,
    EXIT,
    END,
    EQUALS,
    FALSE,
    GIVE,
    GREATER,
    GREATEREQ,
    ID,
    IF,
    INT,
    INTLITERAL,
    LCURLY,
    LESS,
    LESSEQ,
    LPAREN,
    MAGIC,
    NOT,
    NOTEQUALS,
    OR,
    OPEN,
    PERFECT,
    POSTDEC,
    POSTINC,
    RETURN,
    RCURLY,
    RPAREN,
    SEMICOL,
    SLASH,
    STRINGLITERAL,
    STAR,
    TRUE,
    VOID,
    WHILE,
}

struct IdToken {

}