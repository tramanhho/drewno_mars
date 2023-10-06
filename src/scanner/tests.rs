use logos::{Logos, Lexer};
use super::tokens::{TokenType, LexingError};

fn check_valid (mut lex: Lexer<'_, TokenType>, expected_vec: Vec<TokenType>) {
    let mut i = 0;
    loop {
        let result = match lex.next() {
            Some(x) => x,
            None => break,
        };

        let text = lex.slice();
        let token = match result {
            Ok(x) => x,
            Err(_) => panic!("{text} returned an error!"),
        };
        let expected = &expected_vec[i];

        assert_eq!(token, *expected,"\n\t{text} was parsed as a {token}, not a {}!\n", *expected);
        i += 1;
    }
}

fn check_invalid (mut lex: Lexer<'_, TokenType>, expected_vec: Vec<LexingError>) {
    let mut i = 0;
    loop {
        let result = match lex.next() {
            Some(x) => x,
            None => break,
        };

        let text = lex.slice();
        let err = match result {
            Ok(_) => panic!("{text} did not get parsed as an error!"),
            Err(e) => e,
        };
        let expected = &expected_vec[i];

        assert_eq!(err, *expected,"\n\t{text} raised the {err} error! This is not the correct error: {}!\n", *expected);
        i += 1;
    }
}

#[test]
fn lex_valid_tokens() {
    let lex = TokenType::lexer(r#"
        and       24Kmagic   bool     else    false 
        too hot   if         int      give    take 
        or        perfect    return   class   true 
        void      while      =        :       , 
        +         -          ==       >       >= 
        {         <          <=       (       ! 
        !=        --         ++       }       ) 
        ;         /          * 
        today I don't feel like doing any work"#
    );

    let correct_results = vec![
        TokenType::AND,       TokenType::MAGIC,   TokenType::BOOL,    TokenType::ELSE,    TokenType::FALSE, 
        TokenType::FALSE,     TokenType::IF,      TokenType::INT,     TokenType::GIVE,    TokenType::TAKE, 
        TokenType::OR,        TokenType::PERFECT, TokenType::RETURN,  TokenType::CLASS,   TokenType::TRUE, 
        TokenType::VOID,      TokenType::WHILE,   TokenType::ASSIGN,  TokenType::COLON,   TokenType::COMMA, 
        TokenType::CROSS,     TokenType::DASH,    TokenType::EQUALS,  TokenType::GREATER, TokenType::GREATEREQ,
        TokenType::LCURLY,    TokenType::LESS,    TokenType::LESSEQ,  TokenType::LPAREN,  TokenType::NOT,
        TokenType::NOTEQUALS, TokenType::POSTDEC, TokenType::POSTINC, TokenType::RCURLY,  TokenType::RPAREN,
        TokenType::SEMICOL,   TokenType::SLASH,   TokenType::STAR,    TokenType::EXIT
    ];

    check_valid(lex, correct_results);
}

#[test]
fn lex_lits() {
    let valid_lex = TokenType::lexer(r#"
        0 500 2147483647 
        _nice __ _ a a123 
        "meow!"
        " this is a valid escape: \n"
        "#
    );

    let invalid_lex = TokenType::lexer(r#"
        2147483648
        @ $
        " this is an invalid escape: \g "
        " what .... 
        " what .... \g
        "#
    );

    let valid_results = vec![
        TokenType::INTLITERAL(0), TokenType::INTLITERAL(500), TokenType::INTLITERAL(2147483647), 
        TokenType::ID("_nice".to_string()), TokenType::ID("__".to_string()), TokenType::ID("_".to_string()), 
        TokenType::ID("a".to_string()), TokenType::ID("a123".to_string()), 
        TokenType::STRINGLITERAL(r#""meow!""#.to_string()), 
        TokenType::STRINGLITERAL(r#"" this is a valid escape: \n""#.to_string()) 
    ];

    let invalid_results = vec![
        LexingError::IntliteralOverflow,
        LexingError::Illegal("@".to_string()), LexingError::Illegal("$".to_string()),
        LexingError::StringliteralBadEscape, 
        LexingError::StringliteralUnterminated, 
        LexingError::StringliteralUnterminatedBadEscape, 
    ];

    check_valid(valid_lex, valid_results);
    check_invalid(invalid_lex, invalid_results);
}