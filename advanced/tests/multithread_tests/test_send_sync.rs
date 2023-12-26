use std::sync::Arc;
use std::thread;

#[derive(Debug)]
struct MyBox(*mut u8);

unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}

#[test]
fn test() {
  let x = 5 as *mut u8;

  let y = MyBox(x);
  let v = Arc::new(y);

  let handle = thread::spawn(move || {
    println!("{:?}", v);
  });
  handle.join().unwrap();
}
