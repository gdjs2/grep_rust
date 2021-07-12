use std::{env, fs};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 3 { show_help(); return; }
    let conf = parse_config(args);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(mut args: Vec<String>) -> Config {
    let conf = Config {
        query: args.remove(1),
        filename: args.remove(2),
    };
    return conf;
}

fn show_help() {
    let help_info = r#"
Usage: minigrep PATTERNS FILE
Search for PATTERNS in specific file.
Example: minigrep hello main.c"#;
}