use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{}]", self.0.join(", "))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_fmt() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
  }
}
