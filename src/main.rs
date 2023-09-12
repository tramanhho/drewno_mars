use std::{env, process};

use drewno_mars::*;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {e}");
        process::exit(1);
    });

    run(config);

    // if let Err(e) = run(config, mode) {
    //     eprintln!("Application error: {e}");
    //     process::exit(1);
    // }
}
