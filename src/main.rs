use std::{env, process};

use drewno_mars::*;

fn main() {
    let (config, mode) = Config::build(env::args().peekable()).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {e}");
        process::exit(1);
    });

    if let Err(e) = run(config, mode) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
