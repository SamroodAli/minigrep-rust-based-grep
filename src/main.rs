use std::{env, process};

use minigrep::{Config, run};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("{err}");
        process::exit(1);
    };
}
