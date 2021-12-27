use std::{
    env::{self, Args},
    error::Error,
    fs,
};

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
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
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
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Skip program name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't receive a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't receive a file name"),
        };
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
