use crate::error::{NResult, ThreadError};
use flume::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

/// 对每一个任务的抽象, 只需要通过函数抽象其功能即可
type ThreadTask = Box<dyn FnOnce() + 'static + Send>;
/// 模拟广播的机制, 通过同步源语保证安全性
type ThreadSafeReceiver = Arc<Mutex<Receiver<ThreadTask>>>;
/// 每一个 worker 是一个线程的抽象
struct Worker {
    inner: JoinHandle<()>,
}

impl Worker {
    pub fn new(receiver: ThreadSafeReceiver, id: usize) -> NResult<Self> {
        let worker_name = format!("[Thread: {}]", id);
        log::info!("The worker: {} is creating!", worker_name.clone());

        let builder = thread::Builder::new().name(worker_name);
        let handler = builder
            .spawn(move || loop {
                if let Ok(r) = receiver.lock() {
                    if let Ok(task) = r.recv() {
                        // safety: 每一个 worker 都是带着名字创建的, 因此可以正常使用 unwrap!
                        log::info!("{} Task has been set!", thread::current().name().unwrap());
                        task();
                    } else {
                        log::error!("All of the senders had been dropped!");
                    }
                } else {
                    log::error!("Lock has been poisoned!");
                }
            })
            .map_err(|e| ThreadError::OSFailToCreateThread(e))?;

        Ok(Self { inner: handler })
    }
}
/// 线程池本质上就是管理很多线程的调度器, 避免大量新开线程带来的开销
struct ThreadPool {
    workers: Vec<Worker>,
    task_sender: Sender<ThreadTask>,
}

impl ThreadPool {
    /// 起一个线程池, 其中包含了 n 个线程
    pub fn new(size: usize) -> NResult<Self> {
        log::info!("Thread pool is creating with {} workers!", size);
        let mut workers = Vec::new();
        let (task_sender, receiver) = flume::unbounded();

        let recv_wrapper = Arc::new(Mutex::new(receiver));
        for x in 0..size {
            let worker = Worker::new(recv_wrapper.clone(), x)?;
            workers.push(worker);
        }

        Ok(Self {
            workers,
            task_sender,
        })
    }

    pub fn execute<F>(&mut self, f: F) -> NResult<()>
    where
        F: FnOnce() + 'static + Send,
    {
        self.task_sender
            .send(Box::new(f))
            .map_err(|e| ThreadError::TaskSendError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::thread_pool::thread_pool::ThreadPool;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test() -> anyhow::Result<()> {
        ns_log::init(None)?;
        let mut pool = ThreadPool::new(4)?;
        let mut i = 10;
        loop {
            i += 1;
            let f = move || {
                log::info!("Hello! {}", i);
            };
            pool.execute(f)?;
            sleep(Duration::from_secs(2));
        }

        Ok(())
    }
}
