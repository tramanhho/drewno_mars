use logos::Logos;
use crate::scanner::tokens::Token;
use crate::scanner::tokens::TokenType;
pub mod tokens;
use std::io::{self, Write};

pub fn tokenizer(input: Vec<&str>, mut tokens: Box<dyn Write>) {
    let mut scanner = Scanner::new();

    // processing 
    for line in input.iter() {
        let (output, error) = scanner.tokenize_line(&line);
        if output != "" {       tokens.write_all(output.as_bytes()).expect("Error writing to file.");      }
        if error  != "" { io::stderr().write_all(error.as_bytes()).expect("Error writing to error file."); }
    }
    
    let eof = if *input.last().unwrap() == "" {
        format!("EOF [{},1]", scanner.row + 1)
    } else {
        format!("EOF [{},{}]", scanner.row, scanner.last_col)
    };
    
    tokens.write_all(eof.as_bytes()).expect("Error writing to file.");
}

pub struct Scanner {
    pub row: u32,
    pub last_col: usize,
}

impl Scanner {
    fn new() -> Scanner {
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
        format!("{:#?}{}\t[{},{:#?}]\n", token.token_type, token.value, self.row, token.start)
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