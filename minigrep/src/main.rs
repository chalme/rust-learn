use minigrep::Config;
use minigrep::run;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let config = Config::build(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    std::process::exit(1);
  });

  run(config).unwrap_or_else(|err| {
    println!("Application error: {}", err);
  })
}
