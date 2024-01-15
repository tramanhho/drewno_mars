use std::{env, process};

use drewno_mars::{Config, run};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {e}");
        process::exit(1);
    });

    run(config);
}
