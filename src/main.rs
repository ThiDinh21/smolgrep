use std::{env, process};

use smolgrep::Config;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = smolgrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
