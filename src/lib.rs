use std::{env, error::Error};

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string(conf.filename)?;
    let result = if conf.case_sensitive {
        search(&conf.query, &content)
    } else {
        search_case_insensitive(&conf.query, &content)
    };

    for line in result { println!("{}", line); }

    return Ok(());
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("You must provide at least 2 parameters.");
        }
        let conf = Config {
            query: args.remove(1),
            filename: args.remove(1),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        };
        return Ok(conf);
    }
}

pub fn show_help() {
    let help_info = r#"
Usage: minigrep PATTERNS FILE
Search for PATTERNS in specific file.
Example: minigrep hello main.c

Set CASE_INSENSITIVE environment variable to 
make the search insensitive"#;

    println!("{}", help_info);
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) { result.push(line); }
    }
    return result;
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let content = "
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}