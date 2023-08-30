use logos::Logos;
use crate::scanner::tokens::Token;
mod tokens;

pub struct Scanner {
    pub row: u32
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            row: 0
        }
    }

    pub fn tokenize_line(&mut self, stream: &String) -> (String, String) {
        self.row += 1;
        let mut lex: logos::Lexer<'_, Token> = Token::lexer(&stream);

        // initialize return texts
        let mut text: String = "".to_owned();
        let mut errors: String = "".to_owned();
    
        // iterate through tokens that logos lexer found 
        loop {
            let token_type = match lex.next() {
                Some(v) => v.unwrap(),
                None => break
            };
            let span_start = &lex.span().start + 1;
            let span_end = &lex.span().end + 1;
            //optional value if we need to keep it 
            let value = match token_type {
                Token::ID | 
                Token::INTLITERAL | 
                Token::STRINGLITERAL | 
                Token::Illegal
                => &lex.slice(),
                _ => ""
            };
            let illegals = [Token::INTLITERALOverflow, Token::Illegal, Token::STRINGLITERALBadEscape, Token::STRINGLITERALUnterminated, Token::STRINGLITERALUnterminatedBadEscape];
            // add to whichever text. err needs an additional error msg so i moved it to a handler
            if illegals.contains(&token_type) {
                let msg = match token_type {
                    Token::INTLITERALOverflow => "Integer literal overflow",
                    Token::Illegal => "Illegal character ",
                    Token::STRINGLITERALBadEscape => "String literal with bad escape sequence detected",
                    Token::STRINGLITERALUnterminated => "Unterminated string literal detected",
                    Token::STRINGLITERALUnterminatedBadEscape => "Unterminated string literal with bad escape sequence detected",
                    _ => ""
                };
                let token_error = format!("FATAL [{},{}] - [{},{}]: {}{}\n", self.row, span_start, self.row, span_end, msg, value);
                errors = format!("{}{}", errors, token_error);
            } else {
                text = format!("{}{:#?}:{} [{},{:#?}]\n", text, token_type, value, self.row, span_start);
            }
        }
        (text, errors) // <== will need to uncomment this to return correctly
    }
}



