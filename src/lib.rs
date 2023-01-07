use std::fs;
use std::error::Error;
use std::env;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(args.file_name)?;

    let result = if args.case_sensitive {
        search(&args.query, &content)
    } else {
        search_case_insensitive(&args.query, &content)
    };
    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }
    result
}

pub struct Args {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool
}

impl Args {
    pub fn new(args: &[String]) -> Result<Args, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        return Ok(Args { query, file_name, case_sensitive });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        );
    }
}