#[test]
fn test_pointer() {
  let mut num = 1;
  let r1 = &num as *const i32;
  unsafe {
    println!("{}", *r1);
  }
  let r2 = &mut num as *mut i32;
  unsafe {
    *r2 = 2;
  }
  println!("{}", num);
}

fn get_memory_location() -> (usize, usize) {
  let string = "hello";
  let pointer = string.as_ptr() as usize;
  let len = string.len();
  (pointer, len)
}

fn get_str_at_location(pointer: usize, len: usize) -> String {
  unsafe {
    let slice = std::slice::from_raw_parts(pointer as *const u8, len);
    String::from_utf8_unchecked(slice.to_vec())
  }
}

#[test]
fn test_str() {
  let (pointer, len) = get_memory_location();
  let string = get_str_at_location(pointer, len);
  println!("The {} bytes at 0x{:X} stored: {}", len, pointer, string);
}

#[test]
fn test_box() {
  // 0x600002b40a50
  // 0x10015389B
  let a: Box<i32> = Box::new(10);
  // 需要先解引用a
  let b: *const i32 = &*a;
  // 使用 into_raw 来创建
  let c: *const i32 = Box::into_raw(a);
  unsafe {
    println!("{:?}", b);
    println!("{:?}", c);
  }
}
