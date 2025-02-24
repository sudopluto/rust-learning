use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = String::from(&args[1]);
        let file_path = String::from(&args[2]);

        let case_sensitive = !env::var("IGNORE_CASE").is_ok();

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
    let mut ret = Vec::new();

    for line in contents.lines() {
        if !case_sensitive {
            let line_norm = line.to_lowercase();
            let query_norm = query.to_lowercase();
            if line_norm.contains(&query_norm) {
                ret.push(line);
            }
        } else {
            if line.contains(&query) {
                ret.push(line);
            }
        }
    }

    return ret;
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
