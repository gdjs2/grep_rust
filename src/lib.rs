use std::{env::{self, Args}, error::Error};

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
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename string"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
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
    contents.lines()
            .filter(|l| l.contains(query))
            .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
            .filter(
                |l| l.to_lowercase().contains(&query.to_lowercase())
            )
            .collect()
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