use std::error::Error;
mod tokens;

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            row: 0,
            col: 0
        }
    }

    // pub fn tokenize_line(&mut self, line: String) -> Result<String, &'static str> {
    //     println!("{}: {}", self.row, line);
    //     self.row += 1;
        
    //     let nyaow = Position::new(1, 2, 3, 4);
    //     println!("{}", Position::start_to_string(&nyaow));

    //     Ok("nice\n".to_string())

        
    //     // if (false) {
    //     //     Err(error_handler(1))
    //     // }
    // }

    pub fn tokenize_line(&mut self, stream: String, col: u32) -> (String, String){
        let mut lex: logos::Lexer<'_, Token> = Token::lexer(stream);
    
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
    
            // add to whichever text. err needs an additional error msg so i moved it to a handler
            if token_type == Token::ILLEGAL {
                errors = format!("{}\n{}", errors, error_handler(token_type, value, col, &lex.span()));
            } else {
                text = format!("{}\n{:#?}{} [{},{:#?}]", text, token_type, value, col, &lex.span().start);
            }
        }
        println!("{}", text);
        println!("{}", errors);

        (text, errors) // <== will need to uncomment this to return correctly
    }
}

