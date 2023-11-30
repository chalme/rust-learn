use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  println!("Searching for {}", config.query);
  println!("In file {}", config.file_path);
  fs::read_to_string(config.file_path)?
    .lines()
    .filter(|line| line.contains(&config.query))
    .for_each(|line| println!("{}", line));
  Ok(())
}

pub struct Config {
  query: String,
  file_path: String,
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    if args.len() > 3 {
      return Err("too many arguments");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();
    Ok(Config { query, file_path })
  }

  fn new(args: &[String]) -> Config {
    Config {
      query: args[1].clone(),
      file_path: args[2].clone(),
    }
  }
}
