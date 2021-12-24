use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &content) {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut match_lines: Vec<&str> = vec![];
    for line in content.lines() {
        if line.contains(query) {
            match_lines.push(line);
        }
    }
    match_lines
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn found_one_match() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
