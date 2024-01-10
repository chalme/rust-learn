use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::time::Instant;

/// Mutex用起来简单，但是无法并发读，RwLock可以并发读，但是使用场景较为受限且性能不够
/// 原子类型。原子指的是一系列不可被 CPU 上下文交换的机器指令，这些指令组合在一起就形成了原子操作。
/// 在多核 CPU 下，当某个 CPU 核心开始运行原子操作时，会先暂停其它 CPU 内核对内存的操作，
/// 以保证原子操作不会被其它 CPU 内核所干扰。
/// 由于原子操作是通过指令提供的支持，因此它的性能相比锁和消息传递会好很多。相比较于锁而言，原子类型不需要开发者处理加锁和释放锁的问题，
/// 同时支持修改，读取等操作，还具备较高的并发性能，几乎所有的语言都支持原子类型。
///
/// 原子类型是无锁类型，但是无锁不代表无需等待，因为原子类型内部使用了CAS循环，当大量的冲突发生时，该等待还是得等待！但是总归比锁要好。
/// https://course.rs/advance/concurrency-with-threads/sync2.html
///
///
/// 内存顺序是指 CPU 在访问内存时的顺序，该顺序可能受以下因素的影响：
///
///  代码中的先后顺序
///  编译器优化导致在编译阶段发生改变(内存重排序 reordering)
///  运行阶段因 CPU 的缓存机制导致顺序被打乱

const THREAD_NUM: usize = 10;
const N_TIMES: u64 = 10000;
static R: AtomicU64 = AtomicU64::new(0);

#[test]
pub fn test_atomic() {
  let start = Instant::now();
  let mut threads = Vec::with_capacity(THREAD_NUM);
  for _ in 0..THREAD_NUM {
    threads.push(thread::spawn(|| {
      for _ in 0..N_TIMES {
        R.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
      }
    }));
  }
  for thread in threads {
    thread.join().unwrap();
  }
  assert_eq!(
    N_TIMES * THREAD_NUM as u64,
    R.load(std::sync::atomic::Ordering::Relaxed)
  );
  println!("{}", R.load(std::sync::atomic::Ordering::Relaxed));
  println!("{}", start.elapsed().as_millis());
}

#[test]
fn test_mutex() {
  let start = Instant::now();

  let counter = Arc::new(Mutex::new(0));
  let mut threads = Vec::with_capacity(THREAD_NUM);
  for _ in 0..THREAD_NUM {
    let counter = Arc::clone(&counter);
    threads.push(thread::spawn(move || {
      for _ in 0..N_TIMES {
        let mut num = counter.lock().unwrap();
        *num += 1;
      }
    }));
  }
  for thread in threads {
    thread.join().unwrap();
  }

  assert_eq!(N_TIMES * THREAD_NUM as u64, *counter.lock().unwrap());
  println!("{}", start.elapsed().as_millis());
}
