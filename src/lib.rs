use std::path::Path;
use std::fs::File;
use std::io::{self, Write};

mod scanner;
use scanner::tokenizer;

mod parser;
use parser::grammar::*;
use parser::lexer::Lexer;

pub struct Config {
    input: String,
    output: Box<dyn Write>,
    mode: ProcessMode
}

pub enum ProcessMode {
    Tokenize,
    Parse
}


impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        let mut mode = None;
        let mut input = None;
        let mut output_file = None;

        args.next();

        loop {
            let arg = match args.next() {
                Some(x) => x,
                None => break,
            };

            if arg.starts_with("-") {
                mode = match arg.as_str() {
                    "-t" => {
                        output_file = match args.next() {
                            Some(x) => Some(x),
                            None => None
                        };
                        Some(ProcessMode::Tokenize)
                    },
                    "-p" => Some(ProcessMode::Parse),
                    _ => return Err("The only supported options right now are: \n  -t Tokenize \n  -p Parse\nTry again with a supported option.")
                }
            } else {
                input = match input {
                    None => Some(arg),
                    _ => return Err("Only one input file allowed!")
                };
            }
        }

        if mode.is_none()  { return Err("No option given!");     }
        if input.is_none() { return Err("No input file given!"); }

        let output = match &output_file {
            Some(x) => {
                let path = Path::new(&x);
                let out_file = match File::create(&path) {
                    Ok(x) => x,
                    Err(_) => return Err("Unable to write to output file!"),
                };
                Box::new(out_file) as Box<dyn Write>
            }
            None => Box::new(io::stdout()) as Box<dyn Write>,
        };

        Ok(Config {
            input: input.unwrap(), 
            output,
            mode: mode.unwrap()
        })
    }
}

pub fn run(config: Config) {
    // read config
    let input = match std::fs::read_to_string(config.input) {
        Ok(v) => v,
        Err(_) => panic!("Unable to read given input file.")
    };

    match config.mode {
        ProcessMode::Tokenize => {
            let lines: Vec<&str> = input.split("\r\n").collect();
            tokenizer(lines, config.output)
        },
        ProcessMode::Parse => {
            // let mut lexer = TokenType::lexer(&input[..]).spanned().map(|(token, range)| {
            //     Ok::<(usize, TokenType, usize), LexicalError>((range.start, token.unwrap(), range.end))
            // });

            // loop {
            //     let token = match lexer.next() {
            //         Some(x) => x,
            //         None => break
            //     };
            //     dbg!(token.unwrap());
            // }
            let lexer = Lexer::new(&input[..]);
            // let ast = match locParser::new().parse(lexer) {
            //     Ok(x) => x,
            //     Err(_) => { panic!("syntax error, Parse failed"); },
            //     Err(_) => { panic!("syntax error, Parse failed"); },
            // };
            let ast = locParser::new().parse(lexer).unwrap();
            println!("{:?}", ast);
        },
    };
}