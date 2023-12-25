use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::thread::{sleep, spawn};
use std::time::Duration;
use tokio::runtime;
use tokio::sync::Semaphore;

/// https://course.rs/advance/concurrency-with-threads/sync1.html
/// 同步
/// 共享内存 （1. 减少内存拷贝成本，2. 实现简洁， 3. 数据竞争更多）
/// 消息传递 （1. 可靠简单是下你，2.模拟现实世界，通知；3. 需要一个任务处理流水线）
/// 消息传递类似一个单所有权的系统：一个值同时只能有一个所有者，如果另一个线程需要该值的所有权，需要将所有权通过消息传递进行转移。
/// 而共享内存类似于一个多所有权的系统：多个线程可以同时访问同一个值。
/// Rc<T>/RefCell<T>用于单线程内部可变性， Arc<T>/Mutex<T>用于多线程内部可变性。
#[test]
pub fn test_mutex() {
  let m = Mutex::new(5);
  {
    let mut num = m.lock().unwrap();
    *num = 6;
  }
  println!("m = {:?}", m);
}

#[test]
pub fn test_mutex_multi_thread() {
  let count = Arc::new(Mutex::new(0));
  let mut handles = vec![];
  for _ in 0..10 {
    let count = Arc::clone(&count);
    handles.push(thread::spawn(move || {
      let mut num = count.lock().unwrap();
      *num += 1;
    }));
  }
  for handle in handles {
    handle.join().unwrap();
  }
  println!("count = {:?}", count);
}

#[test]
pub fn test_lock() {
  let data = Mutex::new(0);
  let _d1 = data.lock();
  let _d2 = data.lock();
}

#[test]
pub fn test_lock_multi_thread() {
  let mutex_1 = Arc::new(Mutex::new(0));
  let mutex_2 = Arc::new(Mutex::new(0));
  let mut children = vec![];
  for i_thread in 0..2 {
    let mutex_1 = Arc::clone(&mutex_1);
    let mutex_2 = Arc::clone(&mutex_2);
    children.push(thread::spawn(move || {
      if i_thread % 2 == 0 {
        let _num = mutex_1.lock().unwrap();
        println!("thread {} lock mutex_1, prepare to lock mutex_2", i_thread);
        thread::sleep(Duration::from_millis(10));
        let _num = mutex_2.lock().unwrap();
      } else {
        let _num = mutex_2.lock().unwrap();
        println!("thread {} lock mutex_2, prepare to lock mutex_1", i_thread);
        let _guard = mutex_1.lock().unwrap();
      }
    }));
  }
  for child in children {
    child.join().unwrap();
  }
  println!("dead lock did not happen");
}

#[test]
pub fn test_lock_multi_thread_try() {
  let mutex_1 = Arc::new(Mutex::new(0));
  let mutex_2 = Arc::new(Mutex::new(0));
  let mut children = vec![];
  for i_thread in 0..2 {
    let mutex_1 = Arc::clone(&mutex_1);
    let mutex_2 = Arc::clone(&mutex_2);
    children.push(thread::spawn(move || {
      if i_thread % 2 == 0 {
        let _num = mutex_1.lock().unwrap();
        println!("thread {} lock mutex_1, prepare to lock mutex_2", i_thread);
        thread::sleep(Duration::from_millis(10));
        let num = mutex_2.try_lock();
        println!("thread {} try_lock mutex_2, result: {:?}", i_thread, num);
      } else {
        let _num = mutex_2.lock().unwrap();
        println!("thread {} lock mutex_2, prepare to lock mutex_1", i_thread);
        thread::sleep(Duration::from_millis(10));
        let guard = mutex_1.try_lock();
        println!("thread {} try_lock mutex_1, result: {:?}", i_thread, guard);
      }
    }));
  }
  for child in children {
    child.join().unwrap();
  }
  println!("dead lock did not happen");
}

#[test]
pub fn test_rw() {
  let lock = RwLock::new(5);
  {
    let guard = lock.read().unwrap();
    let guard1 = lock.read().unwrap();
    assert_eq!(*guard, 5);
    assert_eq!(*guard1, 5);
  }

  {
    let mut guard = lock.write().unwrap();
    *guard = 6;
    assert_eq!(*guard, 6);
    // let result = lock.read();
    // println!("result: {:?}", result);
  }
}

#[test]
pub fn test_condvar() {
  let flag = Arc::new(Mutex::new(false));
  let cond = Arc::new(Condvar::new());

  let cflag = Arc::clone(&flag);
  let ccond = Arc::clone(&cond);

  let hdl = spawn(move || {
    let mut lock = cflag.lock().unwrap();
    let mut counter = 0;
    while counter < 3 {
      while !*lock {
        lock = ccond.wait(lock).unwrap();
      }
      *lock = false;
      counter += 1;
      println!("inner counter: {}", counter);
    }
  });
  let mut counter = 0;
  loop {
    sleep(Duration::from_millis(1000));
    *flag.lock().unwrap() = true;
    counter += 1;
    if counter > 3 {
      break;
    }
    println!("outer counter: {}", counter);
    cond.notify_one();
  }
  hdl.join().unwrap();
  println!("{:?}", flag);
}

#[test]
pub fn test_semaphore() {
  let rt = runtime::Runtime::new().unwrap();
  rt.block_on(async {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handles = vec![];
    for _ in 0..5 {
      let permit = semaphore.clone().acquire_owned().await.unwrap();
      join_handles.push(tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(1000)).await;
        println!("task done");
        drop(permit);
      }));
    }
    for handle in join_handles {
      handle.await.unwrap();
    }
  });
}
