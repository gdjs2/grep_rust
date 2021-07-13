use std::{env, process};

use minigrep::{Config, run, show_help};

fn main() {
    let conf = Config::new(env::args()).unwrap_or_else(|err| {
        #[cfg(debug_assertions)]
        panic!();
        #[cfg(not(debug_assertions))] {
            eprintln!("{}", err);
            show_help();
            process::exit(1);
        }
    });
    if let Err(e) = run(conf) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
