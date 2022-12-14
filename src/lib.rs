use std::error::Error;
use std::{fs, env};

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let program = match args.next() {
            Some(arg) => arg,
            None => return Err("application error"),
        };
        eprintln!("program {} is running", program);

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did't get a file name"),
        };

        let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_insensitive })
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let query = config.query;
    let contents = fs::read_to_string(config.filename)?;

    let reuslts = if config.case_insensitive == true {
        search(&query, &contents)
    } else {
        search_case_insensitive(&query, &contents)
    };

    for line in reuslts{
        println!("{}", line);
    }
        
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
   contents.lines().filter(|line| line.contains(query)).collect() 
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents.lines()
        .map(|line| line.to_lowercase())
        .filter(|line| line.contains(&query))
        .map(|line| {
           string_to_static_str(line)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust;
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
            search_case_insensitive(query, contents)
        )
    }
}
