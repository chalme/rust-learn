use std::cmp::Ordering;
use std::{char, i64, mem, usize};

struct Foo {
  a: i32,
  b: i32,
  c: bool,
  d: f64,
}

#[test]
pub fn test_mem() {
  println!("foo size of {}", mem::size_of::<Foo>());
  println!("foo align of {}", mem::align_of::<Foo>());
  println!("i32 size of {}", mem::size_of::<i32>());
  println!("i64 size of {}", mem::size_of::<i64>());
  println!("bool align of {}", mem::align_of::<bool>());

  let a = 1;
  println!("{}", a);

  let b: u32 = unsafe { mem::transmute(1) };
  println!("{}", b);

  let s: &str = "on中文";
  println!("{}", mem::size_of_val(s));
}

#[test]
pub fn test_default() {
  println!("i32 default {}", i32::default());
  println!("i64 default {}", i64::default());
  println!("usize default {}", usize::default());
  println!("char default {}", char::default());
}

#[test]
pub fn test_enum_transmute() {
  let a: i8 = 2;
  let order: Ordering = unsafe { mem::transmute(a) };
  println!("{:?}", "hello");
  println!("{:?}", order);
  println!("{:?}", "world");
}
