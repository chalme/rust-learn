use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

#[test]
pub fn test_i32() {
  let x = Box::new(1);
  let _sum = *x + 1;
}

#[test]
pub fn test_string() {
  let s = Box::new("hello".to_string());
  let _x = s.deref().deref();
}

struct Foo;

#[test]
pub fn test_drop() {
  let x = Foo;
  drop(x);
}

#[test]
pub fn test_rc_clone() {
  let a = Rc::new(String::from("hello"));
  let b = a.clone();
  let c = Rc::clone(&a);
  let a_size = Rc::strong_count(&a);
  println!("{}", a_size);
  println!("{}", Rc::strong_count(&b));
  println!("{}", Rc::strong_count(&c));
}

#[test]
pub fn test_arc() {
  let s = Arc::new("hello".to_string());

  for _ in 0..10 {
    let s = Arc::clone(&s);
    thread::spawn(move || {
      println!("{}", s);
      println!("{}", Arc::strong_count(&s));
      println!("{}", Arc::weak_count(&s));
    });
  }
}

/// Cell 只适用于 Copy 类型，用于提供值，而 RefCell 用于提供引用
/// Cell 不会 panic，而 RefCell 会
#[test]
pub fn test_cell() {
  let x = Cell::new(1);
  let y = &x;
  let z = &x;
  x.set(2);
  y.set(3);
  z.set(4);
  println!("{}", x.get());
  println!("{}", y.get());
  println!("{}", z.get());
}

#[test]
pub fn test_rc_refcell() {
  let x = Rc::new(RefCell::new("hello, world".to_string()));
  let s1 = x.clone();
  let s2 = x.clone();
  s2.borrow_mut().push_str(" from s2");
  println!("{:?}, {:?}", s1, s2);
}
