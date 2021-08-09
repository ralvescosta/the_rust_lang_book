use std::env;
use std::process;

use minigrap_cli::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrap_cli::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
