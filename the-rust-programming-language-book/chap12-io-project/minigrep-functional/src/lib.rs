use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build<I: Iterator<Item = String>>(mut args: I) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let case_sensitive = env::var("IGNORE_CASE").is_ok();

        return Ok(Config {
            query,
            file_path,
            case_sensitive,
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for \"{}\"...", config.query);
    println!("In file \"{}\"...\n", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    return Ok(());
}

fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let lines = contents.lines();

    if case_sensitive {
        lines.filter(|line| line.contains(query)).collect()
    } else {
        lines
            .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    search(query, contents, true)
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    search(query, contents, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        )
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
