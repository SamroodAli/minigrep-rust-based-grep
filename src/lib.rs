use std::{env, fs};
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        const IGNORE_CASE: &str = "IGNORE_CASE";

        if args.len() < 3 {
            return Err("Not enough arguments");
        };

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var(IGNORE_CASE).is_ok_and(|v| v != "false");

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let search = if config.ignore_case {
        search_case_insensitive
    } else {
        search_case_sensitive
    };

    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}


fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }
    }

    result
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{search_case_insensitive, search_case_sensitive};

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents))
    }


    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me";
        assert_eq!(vec!["Rust:", "Trust me"], search_case_insensitive(query, contents))
    }
}
