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
  contents.lines()
    .filter(|line| line.contains(query))
    .collect()
}

fn search_case_insensitive<'a>(query: &str, contents : &'a str) -> Vec<&'a str> {
  contents.lines()
    .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
    .collect()
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args : std::env::Args) -> Result<Config, &'static str> {
      args.next();
      let query = match args.next() {
        Some(x) => x,
        None => return Err("Didn't get a query string")
      };
      let filename = match args.next() {
        Some(x) => x,
        None => return Err("Didn't get a filename")
      };
      Ok(Config {
        query: query,
        filename: filename,
        case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
      })
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