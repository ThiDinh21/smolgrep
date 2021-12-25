use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    let matched_lines = if config.case_sensitive {
        search_case_sensitive(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in matched_lines {
        println!("{}", line);
    }

    Ok(())
}

fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut match_lines: Vec<&str> = vec![];
    for line in content.lines() {
        if line.contains(query) {
            match_lines.push(line);
        }
    }
    match_lines
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut match_lines: Vec<&str> = vec![];
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            match_lines.push(line);
        }
    }
    match_lines
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    mod config_tests {
        use crate::Config;

        #[test]
        fn no_arg() {
            assert!(Config::new(&[]).is_err(), "No arg should return an error.");
        }

        #[test]
        fn one_arg() {
            assert!(
                Config::new(&[String::from("Hello, world!")]).is_err(),
                "One arg should return an error."
            );
        }

        #[test]
        fn wrong_filename() {
            assert!(
                Config::new(&[String::from("Hello, world!"), String::from("Stuffs")]).is_err(),
                "Filename that are different than poem.txt should return an error."
            );
        }

        #[test]
        fn valid_input() {
            assert!(
                Config::new(&[String::from("Hello, world!"), String::from("poem.txt")]).is_err(),
                "Valid inputs shouldn't return an error."
            );
        }
    }

    mod search_tests {
        use crate::*;

        #[test]
        fn found_one_match() {
            let query = "duct";
            let content = "\
Rust:
safe, fast, productive.
Pick three.";

            assert_eq!(
                vec!["safe, fast, productive."],
                search_case_sensitive(query, content)
            );
        }
    }

    mod env_var_tests {
        use crate::*;

        #[test]
        fn case_sensitive() {
            let query = "duct";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

            assert_eq!(
                vec!["safe, fast, productive."],
                search_case_sensitive(query, contents)
            );
        }

        #[test]
        fn case_insensitive() {
            let query = "rUsT";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me";

            assert_eq!(
                vec!["Rust:", "Trust me"],
                search_case_insensitive(query, contents)
            );
        }
    }
}
