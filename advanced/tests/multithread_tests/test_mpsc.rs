use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// 消息传递
/// https://course.rs/advance/concurrency-with-threads/message-passing.html
/// ust 是在标准库里提供了消息通道(channel)，你可以将其想象成一场直播，多个主播联合起来在搞一场直播，
/// 最终内容通过通道传输给屏幕前的我们，
/// 其中主播被称之为发送者，观众被称之为接收者，显而易见的是：一个通道应该支持多个发送者和接收者。
///
/// mpsc::channel 异步通道
/// mpsc::sync_channel(0) 同步通道
/// 所有发送者被drop或者所有接收者被drop后，通道会自动关闭。

#[test]
pub fn test_mpsc() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    tx.send(1).unwrap();
    tx.send(2).unwrap();
    tx.send(2).unwrap();
  });

  assert_eq!(1, rx.recv().unwrap());
  println!("{}", rx.iter().count());
  let _result = rx.try_recv();
}

#[test]
pub fn test_mpsc_in() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    tx.send(1).unwrap();
    thread::sleep(Duration::from_secs(1));
    tx.send(2).unwrap();
    thread::sleep(Duration::from_secs(1));
    tx.send(2).unwrap();
    thread::sleep(Duration::from_secs(1));
  });

  for i in rx {
    println!("{}", i);
  }
}

#[test]
pub fn test_mpsc_multi_sender() {
  let (tx, rx) = mpsc::channel();
  let tx1 = tx.clone();

  thread::spawn(move || {
    tx.send(1).unwrap();
  });

  thread::spawn(move || {
    tx1.send(2).unwrap();
  });

  for i in rx {
    println!("{}", i);
  }
}

#[test]
pub fn test_mpsc_sub_thread() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    tx.send(1).unwrap();
    tx.send(2).unwrap();
  });
  thread::spawn(move || {
    println!("{}", rx.recv().unwrap());
    println!("{}", rx.recv().unwrap());
  });
  thread::sleep(Duration::from_secs(1));
}
