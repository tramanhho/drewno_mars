use std::error::Error;
mod tokens;

mod position;
use crate::scanner::position::Position;

pub struct Scanner {
    row: i32,
    col: i32
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            row: 0,
            col: 0
        }
    }

    pub fn tokenize_line(&mut self, line: String) -> Result<String, &'static str> {
        println!("{}: {}", self.row, line);
        self.row += 1;
        
        let nyaow = Position::new(1, 2, 3, 4);
        println!("{}", Position::start_to_string(&nyaow));

        Ok("nice\n".to_string())

        
        // if (false) {
        //     Err(error_handler(1))
        // }
    }

    fn error_handler(code: i32) -> &'static str {
        match code {
            1 => "",
            _ => ""
        }
    }
}

