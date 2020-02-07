use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let result = if config.case_sensitive {
      search(&config.query, &contents)
    } else {
      search_case_insensitive(&config.query, &contents)
    };
    for line in result {
      println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query : &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }
  results
}

fn search_case_insensitive<'a>(query: &str, contents : &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  let query = query.to_lowercase();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }
  results
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args : &[String]) -> Result<Config, &str> {
        if args.len() >= 3 {
            Ok(Config {
              query: args[1].clone(),
              filename: args[2].clone(),
              case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
            })
        } else {
            Err("Wrong number of arguments")
        }
    }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
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