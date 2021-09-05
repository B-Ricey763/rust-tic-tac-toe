use std::{process, env};

use tic_tac_toe::Config;

fn main() {
    let _config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = tic_tac_toe::run() {
        eprintln!("Game error: {}", e);
        process::exit(1);
    }
}
