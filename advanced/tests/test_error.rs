use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
struct AppError {
  code: usize,
  message: String,
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let err_msg = match self.code {
      404 => "Sorry, Can not find the Page!",
      _ => "Sorry, something is wrong! Please Try Again!",
    };

    write!(f, "{}", err_msg)
  }
}

impl std::error::Error for AppError {}

fn produce_error() -> Result<(), AppError> {
  Err(AppError {
    code: 404,
    message: String::from("Page not found"),
  })
}

#[test]
fn test_app_error() {
  match produce_error() {
    Ok(_) => {}
    Err(e) => {
      eprintln!("{}", e)
    }
  }
  eprintln!("{:?}", produce_error());
}

fn render() -> Result<String, Box<dyn Error>> {
  let file = std::fs::read_to_string("hello.txt")?;
  Ok(file)
}

#[test]
fn test_render() -> Result<(), Box<dyn Error>> {
  produce_error()?;
  let html = render()?;
  println!("{}", html);

  Ok(())
}

#[derive(thiserror::Error, Debug)]
enum MyError {
  #[error("Environment variable not found")]
  EnvironmentVariableNotFount(#[from] std::env::VarError),
  #[error(transparent)]
  Io(#[from] std::io::Error),
}

fn test_render_my_error() -> Result<String, MyError> {
  let file = std::env::var("HELLO")?;
  let text = std::fs::read_to_string(file)?;
  Ok(text)
}

#[test]
fn test_main_render_my_error() -> Result<(), MyError> {
  let string = test_render_my_error()?;
  println!("{}", string);
  Ok(())
}

fn test_anyhow_error() -> anyhow::Result<String> {
  let file = std::env::var("HELLO")?;
  let text = std::fs::read_to_string(file)?;
  Ok(text)
}

#[test]
fn test_main_anyhow_error() -> anyhow::Result<()> {
  let string = test_anyhow_error()?;
  println!("{}", string);
  Ok(())
}
