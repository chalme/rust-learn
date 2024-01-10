use futures::future::BoxFuture;
use futures::task::{waker_ref, ArcWake};
use futures::FutureExt;
use std::future::Future;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::task::Context;
use std::time::Duration;
use timer_future::TimerFuture;

/// 任务执行器，负责从通道中接收任务然后执行
struct Executor {
  ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner`负责创建新的`Future`然后将它发送到任务通道中
#[derive(Clone)]
struct Spawner {
  task_sender: SyncSender<Arc<Task>>,
}

/// 一个Future，它可以调度自己(将自己放入任务通道中)，然后等待执行器去`poll`
struct Task {
  future: Mutex<Option<BoxFuture<'static, ()>>>,
  task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
  const MAX_QUEUED_TASKS: usize = 10_000;
  let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
  (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
  fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
    let future = future.boxed();
    let task = Arc::new(Task {
      future: Mutex::new(Some(future)),
      task_sender: self.task_sender.clone(),
    });
    self.task_sender.send(task).expect("too many tasks queued");
  }
}

impl ArcWake for Task {
  fn wake_by_ref(arc_self: &Arc<Self>) {
    let cloned = arc_self.clone();
    arc_self.task_sender.send(cloned).expect("too many tasks queued");
  }
}

impl Executor {
  fn run(&self) {
    while let Ok(task) = self.ready_queue.recv() {
      // 获取一个future，若它还没有完成(仍然是Some，不是None)，则对它进行一次poll并尝试完成它
      let mut future_slot = task.future.lock().unwrap();
      if let Some(mut future) = future_slot.take() {
        let waker = waker_ref(&task);
        let context = &mut Context::from_waker(&waker);
        if future.as_mut().poll(context).is_pending() {
          *future_slot = Some(future);
        }
      }
    }
  }
}

fn main() {
  let (executor, spawner) = new_executor_and_spawner();
  spawner.spawn(async {
    println!("howdy!");
    TimerFuture::new(Duration::new(2, 0)).await;
    println!("done!");
  });

  // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
  drop(spawner);

  // 运行执行器直到任务队列为空
  // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
  executor.run();
}
