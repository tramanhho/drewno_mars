use logos::Logos;
use crate::scanner::tokens::Token;
mod tokens;

pub struct Scanner {
    col: i32
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            col: 0
        }
    }

    pub fn tokenize_line(&mut self, stream: &String) -> (String, String) {
        self.col += 1;
        let mut lex: logos::Lexer<'_, Token> = Token::lexer(&stream);
        let span = &lex.span();

        // initialize return texts
        let mut text: String = "".to_owned();
        let mut errors: String = "".to_owned();
    
        // iterate through tokens that logos lexer found 
        loop {
            let token_type = match lex.next() {
                Some(v) => v.unwrap(),
                None => break
            };
    
            //optional value if we need to keep it 
            let value = match token_type {
                Token::ID | 
                Token::INTLITERAL | 
                Token::STRINGLITERAL | 
                Token::ILLEGAL
                => &lex.slice(),
                _ => ""
            };
            let illegals = [Token::INTLITERALOVERFLOW, Token::ILLEGAL, Token::STRINGLITERALBADESCAPE, Token::STRINGLITERALUNTERMINATED, Token::STRINGLITERALUNTERMINATEDBADESCAPE];
            // add to whichever text. err needs an additional error msg so i moved it to a handler
            if illegals.contains(&token_type) {
                let msg = match token_type {
                    Token::INTLITERALOVERFLOW => "Integer literal overflow",
                    Token::ILLEGAL => "Illegal character ",
                    Token::STRINGLITERALBADESCAPE => "String literal with bad escape sequence detected",
                    Token::STRINGLITERALUNTERMINATED => "Unterminated string literal detected",
                    Token::STRINGLITERALUNTERMINATEDBADESCAPE => "Unterminated string literal with bad escape sequence detected",
                    _ => ""
                };
                let token_error = format!("FATAL [{},{}] - [{},{}]: {}{}", self.col, span.start, self.col, span.end, msg, value);
                errors = format!("{}\n{}", errors, token_error);
            } else {
                text = format!("{}\n{:#?}:{} [{},{:#?}]", text, token_type, value, self.col, span.start);
            }
        }
        (text, errors) // <== will need to uncomment this to return correctly
    }
}



