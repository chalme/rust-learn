use std::cell::Cell;
use std::sync::{Arc, Barrier, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use thread_local::ThreadLocal;

/// 如果某个系统支持两个或者多个动作的同时存在，那么这个系统就是一个并发系统。
/// 如果某个系统支持两个或者多个动作同时执行，那么这个系统就是一个并行系统。
/// 并发系统与并行系统这两个定义之间的关键差异在于 “存在” 这个词。
/// https://course.rs/advance/concurrency-with-threads/concurrency-parallelism.html

#[test]
pub fn test_spawn() {
  let handle = thread::spawn(|| {
    for i in 0..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });
  handle.join().unwrap();
  for i in 0..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }
}

#[test]
pub fn test_spawn_move() {
  let v = vec![1, 2, 3];
  let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
  });
  println!("{:?}", handle.thread().name());
  handle.join().unwrap();
}

/// 测试用例不是main线程，而是子线程
#[test]
pub fn test_sub_thread() {
  let new_thread = thread::spawn(move || {
    thread::spawn(move || loop {
      println!("Hi, I'm a sub thread");
    })
  });
  new_thread.join().unwrap();
  println!("Child thread is done");
  thread::sleep(Duration::from_millis(100));
}

#[test]
pub fn test_barrier() {
  let barrier = Arc::new(Barrier::new(10));
  let mut handles = vec![];
  for _ in 0..10 {
    let c = barrier.clone();
    handles.push(thread::spawn(move || {
      println!("before wait");
      c.wait();
      println!("after wait");
    }))
  }
  for handle in handles {
    handle.join().unwrap();
  }
}

#[test]
pub fn test_thread_local() {
  let tls = Arc::new(ThreadLocal::new());
  for _ in 0..5 {
    let tls2 = tls.clone();
    thread::spawn(move || {
      let cell = tls2.get_or(|| Cell::new(0));
      cell.set(cell.get() + 1);
    })
    .join()
    .unwrap();
  }
  let tls = Arc::try_unwrap(tls).unwrap();
  let total = tls.into_iter().fold(0, |a, b| a + b.get());
  assert_eq!(5, total);
}

#[test]
pub fn test_condition_run() {
  let cond = Arc::new((Mutex::new(false), Condvar::new()));
  let cond2 = Arc::clone(&cond);
  thread::spawn(move || {
    let (lock, cvar) = &*cond2;
    let mut started = lock.lock().unwrap();
    println!("changing started");
    *started = true;
    cvar.notify_one();
    println!("sub thread changed");
  });
  thread::sleep(Duration::from_secs(1));
  let (lock, cvar) = &*cond;
  let mut started = lock.lock().unwrap();
  println!("main lock started");
  println!("{}", *started);
  while !*started {
    started = cvar.wait(started).unwrap();
  }
  println!("started changed");
}

#[test]
pub fn test_condvar() {
  let shared_data = Arc::new((Mutex::new(false), Condvar::new()));
  let shared_data_clone = Arc::clone(&shared_data);
  thread::spawn(move || {
    let (lock, cvar) = &*shared_data_clone;
    let mut started = lock.lock().unwrap();
    while !*started {
      println!("sub thread waiting for condition to be true");
      started = cvar.wait(started).unwrap();
    }
    println!("sub thread done waiting for condition to be true");
  });

  let shared_data_clone = Arc::clone(&shared_data);
  thread::spawn(move || {
    let (lock, cvar) = &*shared_data_clone;
    let mut started = lock.lock().unwrap();
    *started = true;
    println!(" thread 2 changing condition");
    cvar.notify_one();
  });
  thread::sleep(Duration::from_secs(1));
}
