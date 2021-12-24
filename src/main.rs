use std::{env, process};

use smolgrep::Config;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Query: {}", config.query);
    println!("File: {}", config.filename);

    if let Err(e) = smolgrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
