use std::path::Path;
use std::fs::File;
use std::io::{self, Write};

mod scanner;
use scanner::{tokenizer, lexer::Lexer};

mod parser;
use parser::{unparse, grammar::*};

pub struct Config {
    input: String,
    output: Box<dyn Write>,
    mode: ProcessMode
}

pub enum ProcessMode {
    Tokenize,
    ParseCheck,
    ParsePrint
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
                let arg_str = arg.as_str();
                mode = match arg_str {
                    "-t" => Some(ProcessMode::Tokenize),
                    "-p" => Some(ProcessMode::ParseCheck),
                    "-u" => Some(ProcessMode::ParsePrint),
                    _ => return Err(
                        "The only supported options right now are:\n  
                        [-t <inputFile.dm> <outputFile> ]: Tokenizes inputFile and outputs result into <outputFile>.\n  
                        [-p <inputFile.dm> ]: Checks if inputFile has syntactically correct Drewno Mars code.\n
                        [-u <inputFile.dm> <outputFile> ]: Converts inputFile into canonical Drewno Mars code and outputs result into <outputFile>.\n
                        Try again with a supported option.\n
                        Note: all <outputFile> arguments are optional. If no <outputFile> is given, output will be printed to console.")
                };

                output_file = match arg_str {
                    "-t" | "-u" => {
                        match args.next() {
                            Some(x) => Some(x),
                            None => None
                        }
                    },
                    _ => None
                };

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
            let lines: Vec<&str> = input.split('\n').collect();
            tokenizer(lines, config.output)
        },
        ProcessMode::ParseCheck => {
            let lexer = Lexer::new(&input[..]);
            match ProgramParser::new().parse(lexer) {
                Ok(_) => (),
                Err(_) => { eprintln!("syntax error\nParse failed"); },
            };
        },
        ProcessMode::ParsePrint => {
            let lexer = Lexer::new(&input[..]);
            let mut output = config.output;
            match ProgramParser::new().parse(lexer) {
                Ok(x) => output
                .write_all(unparse(x).as_bytes())
                .expect("Error writing to output file."),

                Err(x) => { eprintln!("Parse failed: {:?}", x); },
            };
        },
    };
}