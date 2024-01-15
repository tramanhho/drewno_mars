use std::io::{self, Write};
use logos::Logos;

pub mod tokens;
use tokens::TokenType;

pub mod lexer;

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
        let lex: logos::SpannedIter<'_, TokenType>  = TokenType::lexer(stream).spanned();
        
        // initialize return texts
        let mut text: String = "".to_owned();
        let mut errors: String = "".to_owned();

        // iterate through tokens that logos lexer found 
        for (result, mut range) in lex {
            range.start += 1;
            range.end += 1;
            
            match result {
                Ok(token) => {
                    text = format!("{}{:15}\t[{},{}]\n", text, token, self.row, range.start);
                }
                Err(e) => {
                    errors = format!("{}FATAL [{},{}]-[{},{}]: {}\n", errors, self.row, range.start, self.row, range.end, e);
                }
            }
            self.last_col = range.end;
        }
        (text, errors)
    }
}