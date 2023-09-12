use logos::{Logos, SpannedIter};
use crate::scanner::tokens::Token;
use crate::scanner::tokens::TokenType;
pub mod tokens;

pub struct Scanner {
    pub row: u32,
    pub last_col: usize,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            row: 0,
            last_col: 0,
        }
    }

    pub fn tokenize_line(&mut self, stream: &str) -> (String, String) {
        self.row += 1;
        let mut lex: logos::Lexer<'_, TokenType> = TokenType::lexer(stream);
        
        // initialize return texts
        let mut text: String = "".to_owned();
        let mut errors: String = "".to_owned();

        // iterate through tokens that logos lexer found 
        loop {
            let token_type = match lex.next() {
                Some(v) => v.unwrap(),
                None => break 
            };

            let token = Token::new(&mut lex, token_type);
            //let illegals = [TokenType::INTLITERALOverflow, TokenType::Illegal, TokenType::STRINGLITERALBadEscape, TokenType::STRINGLITERALUnterminated, TokenType::STRINGLITERALUnterminatedBadEscape];
            let illegals = [TokenType::STRINGLITERALBadEscape, TokenType::STRINGLITERALUnterminated, TokenType::STRINGLITERALUnterminatedBadEscape];
            //add to whichever text
            
            if illegals.contains(&token.token_type) {
                errors = format!("{}{}", errors, self.error_msg(&token));
            } else {
                text = format!("{}{}", text, self.token_msg(&token));
            }
            self.last_col = token.end;
        }
        (text, errors)
    }

    fn token_msg(&self, token: &Token) -> String {
        format!("{:#?}:{} [{},{:#?}]\n", token.token_type, token.value, self.row, token.start)
    }

    fn error_msg(&self, token: &Token) -> String {
        format!("FATAL [{},{}] - [{},{}]: {}{}\n", self.row, token.start, self.row, token.end, error_handler(&token.token_type), token.value)
    }
    
}

fn error_handler(token_type : &TokenType ) -> &str {
    match token_type {
        TokenType::INTLITERALOverflow => "Integer literal overflow",
        TokenType::Illegal => "Illegal character ",
        TokenType::STRINGLITERALBadEscape => "String literal with bad escape sequence detected",
        TokenType::STRINGLITERALUnterminated => "Unterminated string literal detected",
        TokenType::STRINGLITERALUnterminatedBadEscape => "Unterminated string literal with bad escape sequence detected",
        _ => ""
    }
}

#[derive(Debug)]
pub enum LexicalError {
    InvalidToken,
}

pub struct Lexer<'input> {
    // instead of an iterator over characters, we have a token iterator
    token_stream: SpannedIter<'input, TokenType>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        // the Token::lexer() method is provided by the Logos trait
        Self { token_stream: TokenType::lexer(input).spanned() }
    }
}

// impl<'input> Iterator for Lexer<'input> {
//     type Item = Result<(usize, TokenType, usize), LexicalError>;

//     fn next(&mut self) -> Option<Result<(usize, TokenType, usize), LexicalError>> {
//         let x = match self.token_stream.next() {
//             Some(x) => x,
//             None => (Ok(TokenType::Illegal), ,
//         };
//         //Some(Ok((span.start, token.unwrap(), span.end)))
//         Some(Ok((0, TokenType::Illegal, 0)))
//     }
// }

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<TokenType, usize, LexicalError>;
  
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.token_stream.next() {
                Some((token, span)) => {
                    return Some(Ok((span.start, token.unwrap(), span.end)))
                }
                None => return None
            }
        }
    }
}