pub mod io;
pub mod search;

use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;

pub struct Config {
    pub search_string: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            search_string: args[1].clone(),
            filename: args[2].clone(),
            ignore_case,
        })
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "================================================================\n");
        let _ = write!(f, "search_string: {}\nfilename: {}\n", self.search_string, self.filename);
        let _ = write!(f, "ignore_case: {}\n", self.ignore_case);
        write!(f, "================================================================\n")
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(config.filename)?;

    let lines:Vec<&str>;
    if config.ignore_case {
        lines = search::search_case_insensitive(&config.search_string, &contents);
    } else {
        lines = search::search(&config.search_string, &contents);
    }

    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
        ";

        assert_eq!(vec!["safe, fast, productive."], search::search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search::search_case_insensitive(query, contents)
        );
    }
}