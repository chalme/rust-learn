fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod test {
  use std::mem;

  #[test]
  pub fn test_memory_address() {
    let mut values: [i32; 2] = [1, 2];
    let p1 = values.as_mut_ptr();
    println!("{:?}", p1);
    let first_address = p1 as usize;
    println!("first_address: {:?}", first_address);
    println!("{}", mem::size_of::<usize>());
    let second_address = first_address + 4;
    println!("second_address: {:?}", second_address);
    let p2 = second_address as *mut i32;
    unsafe {
      *p2 = 3;
    }
    println!("{:?}", values);
  }

  #[test]
  pub fn test_int_address() {
    let mut a = 1;
    let p = &mut a as *mut i32;
    println!("{:?}", p);
    unsafe {
      println!("{}", *p);
    }
    unsafe {
      println!("{}", *p + 1);
    }
    unsafe {
      *p = *p + 1;
    }
    unsafe {
      println!("{}", a);
    }
    unsafe {
      println!("{}", a);
    }
  }

  #[test]
  pub fn test_string_address() {
    let mut s = String::from("hello");
    let _p = &mut s as *mut String;
  }
}
