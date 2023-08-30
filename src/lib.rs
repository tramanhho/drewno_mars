use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

mod scanner;
use scanner::Scanner;

pub struct Config {
    input_file: String,
    output_file: String,
}

//TODO: actually process so its not just index based. add https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html??
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let input_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an input file"),
        };

        // TODO: add option types
        let _option = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an option"),
        };

        let output_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an output file"),
        };

        Ok(Config { 
            input_file, 
            output_file
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read config
    let input = File::open(config.input_file).expect("Unable to open file");
    let input = BufReader::new(input);

    // write config 
    let tokens = File::create(config.output_file).expect("Unable to create file");
    let mut tokens = BufWriter::new(tokens);

    // new scanner 
    let mut scanner = Scanner::new();
    // processing 
    for line in input.lines() {
        let line = line.expect("Unable to read line");
        let (output, error) = scanner.tokenize_line(&line);
        if output != "" {       tokens.write_all(output.as_bytes()).expect("Error writing to file.");      }
        if error  != "" { io::stderr().write_all(error.as_bytes()).expect("Error writing to error file."); }
    }
    let eof = format!("EOF [{},1]", scanner.row + 1);
    tokens.write_all(eof.as_bytes()).expect("Error writing to file.");
    Ok(())
}