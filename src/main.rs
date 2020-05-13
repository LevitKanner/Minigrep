use minigrep::Config;
use std::{env, process};

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let config = Config::new(&arguments).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(_e) = minigrep::run(config) {
        process::exit(1);
    }

}
