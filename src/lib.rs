use std::path::Path;
use std::fs::File;
use std::io::{self, Write};

mod scanner;
use scanner::{tokenizer, lexer::Lexer};

mod parser;
use parser::{unparse, grammar::*};
use parser::ast::span::{line_bytes, node::SpanNode};

mod format;

mod analysis;
use analysis::name::named_unparse;
use analysis::_type::type_error_check;

mod assembly;
use assembly::three_ac::convert_3ac;

use indoc::indoc;

pub struct Config {
    input: String,
    output: Box<dyn Write>,
    mode: ProcessMode
}

enum ProcessMode {
    Tokenize,
    ParseCheck,
    Unparse,
    NamedUnparse,
    TypeCheck,
    Generate3AC,
}


impl Config {
    pub fn build<'a>(
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
                    "-u" => Some(ProcessMode::Unparse),
                    "-n" => Some(ProcessMode::NamedUnparse),
                    "-c" => Some(ProcessMode::TypeCheck),
                    "-a" => Some(ProcessMode::Generate3AC),
                    _ => return Err(indoc!{"
                        The only supported options right now are:
                            [<inputFile.dm> -t <outputFile> ]: Tokenizes inputFile. Outputs result into <outputFile>.
                            [<inputFile.dm> -p]: Checks if inputFile has syntactically correct Drewno Mars code.
                            [<inputFile.dm> -u <outputFile>]: Converts inputFile into canonical Drewno Mars code. Outputs result into <outputFile>.
                            [<inputFile.dm> -n <outputFile>]: Converts inputFile into canonical Drewno Mars code with types. Outputs result into <outputFile>.
                            [<inputFile.dm> -c]: Checks if the Drewno Mars code in inputFile passes Type Analysis.
                            [<inputFile.dm> -a <outputFile>]: Converts Drewno Mars code into an 3AC intermediate representation. Outputs result into <outputFile>.

                        Try again with a supported option.

                        Note: all <outputFile> arguments are optional. If no <outputFile> is given, output will be printed to console.
                    "})
                };

                output_file = match arg_str {
                    "-t" | "-u" | "-n" | "-a" => {
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
            // for token in lexer {
            //     dbg!(token);
            // }
            
            match ProgramParser::new().parse(lexer) {
                Ok(mut x) => {
                    x.correct_span_rec(&line_bytes(input));
                    // println!("{:?}", x);
                },
                Err(_) => { eprintln!("syntax error\nParse failed"); },
            };
        },
        ProcessMode::Unparse => {
            let lexer = Lexer::new(&input[..]);
            let mut output = config.output;

            match ProgramParser::new().parse(lexer) {
                Ok(x) => output
                .write_all(unparse(x).as_bytes())
                .expect("Error writing to output file."),

                Err(x) => { eprintln!("Parse failed: {:?}", x); },
            };
        },

        ProcessMode::NamedUnparse => {
            let lexer = Lexer::new(&input[..]);
            let mut output = config.output;
            // println!("{:?}", ProgramParser::new().parse(lexer));
            match ProgramParser::new().parse(lexer) {
                Ok(x) => output
                .write_all(named_unparse(x, input).as_bytes())
                .expect("Error writing to output file."),
                
                Err(x) => { eprintln!("Parse failed: {:?}", x); },
            };
        },

        ProcessMode::TypeCheck => {
            let lexer = Lexer::new(&input[..]);
            match ProgramParser::new().parse(lexer) {
                Ok(mut x) => { x.correct_span_rec(&line_bytes(input)); type_error_check(x); },
                Err(x) => { eprintln!("Parse failed: {:?}", x); },
            };
        },

        ProcessMode::Generate3AC => {
            let mut output = config.output;

            let lexer = Lexer::new(&input[..]);
            match ProgramParser::new().parse(lexer) {
                Ok(mut x) => output
                    .write_all(convert_3ac(x).as_bytes())
                    .expect("Error writing to output file."),
                Err(x) => { eprintln!("Parse failed: {:?}", x); },
            };
        },
    };
}