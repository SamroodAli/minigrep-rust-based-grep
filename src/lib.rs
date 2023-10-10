use std::{env, fs};
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build<T>(mut args: T) -> Result<Config, &'static str>
        where
            T: Iterator<Item=String>,
    {
        const IGNORE_CASE: &str = "IGNORE_CASE";

        // ignore the first command argument ( name of the program )
        let _ = args.next();

        let query: String = args.next().expect("Didn't get a query string");
        let file_path: String = args.next().expect("Didn't get a file path");

        let ignore_case = env::var(IGNORE_CASE).is_ok_and(|v| v != "false");

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let search = if config.ignore_case {
        search_case_insensitive
    } else {
        search_case_sensitive
    };

    let search_results = search(&config.query, &contents);

    for line in search_results {
        println!("{line}");
    }
    Ok(())
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
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

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me";
        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_case_insensitive(query, contents)
        )
    }
}
