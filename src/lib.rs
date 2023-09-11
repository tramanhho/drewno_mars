use std::path::Path;
use std::fs::File;
use std::io::{self, Write};
use std::iter::Peekable;

mod scanner;
use scanner::Scanner;

pub struct Config {
    input: String,
    output: Box<dyn Write>,
}

pub enum ProcessMode {
    Tokenize,
    Parse
}


impl Config {
    pub fn build<I: Iterator<Item = String>>(
        mut args: Peekable<I>,
    ) -> Result<(Config, ProcessMode), &'static str> {
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

        Ok((
            Config {
                input: input.unwrap(), 
                output
            },
            mode.unwrap()
        ))
    }
}

pub fn run(config: Config, mode: ProcessMode) -> Result<(), &'static str> {
    // read/write config
    match mode {
        ProcessMode::Tokenize => tokenize(config),
        ProcessMode::Parse => parser(config),
    }
    
}

fn tokenize(config: Config) -> Result<(), &'static str> {
    let input = match std::fs::read_to_string(config.input) {
        Ok(v) => v,
        Err(_) => return Err("Unable to read given output file.")
    };
    let lines: Vec<&str> = input.split("\r\n").collect();
    let mut tokens = config.output;
    let mut scanner = Scanner::new();

    // processing 
    for line in lines.iter() {
        let (output, error) = scanner.tokenize_line(&line);
        if output != "" {       tokens.write_all(output.as_bytes()).expect("Error writing to file.");      }
        if error  != "" { io::stderr().write_all(error.as_bytes()).expect("Error writing to error file."); }
    }
    
    let eof = if *lines.last().unwrap() == "" {
        format!("EOF [{},1]", scanner.row + 1)
    } else {
        format!("EOF [{},{}]", scanner.row, scanner.last_col)
    };
    
    tokens.write_all(eof.as_bytes()).expect("Error writing to file.");
    Ok(())
}

use logos::Lexer;
mod parser;
use crate::parser::grammar::*;

fn parser(config: Config) -> Result<(), &'static str> {
    let source_code = match std::fs::read_to_string(config.input) {
        Ok(v) => v,
        Err(_) => return Err("Unable to read given output file.")
    };
    let lexer = Lexer::new(&source_code[..]);
    let parser = locParser::new();
    let ast = parser.parse(lexer)?;

    println!("{:?}", ast);

    Ok(())
}