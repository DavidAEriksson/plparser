use std::{env, process};

use plparser::InputConfig;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = InputConfig::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = plparser::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
