use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    /// 新しいThreadPoolを生成する。
    ///
    /// sizeがプールのスレッド数です。
    ///
    /// # パニック
    ///
    /// sizeが0なら、`new`関数はパニックします。
    ///
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    // id番号を取り、idと空のクロージャで大量生産されるスレッドを保持するWorkerインスタンスを返す
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // 新しいチャンネルを作成
        let (sender, receiver) = mpsc::channel();

        // size個の要素を保持できる新しい空のベクタを生成
        let mut workers = Vec::with_capacity(size);

        // ThreadPool::newでforループカウンタを使用してidを生成し、そのidで新しいWorkerを生成し、ベクタにWorkerを格納
        for id in 0..size {
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static 
    {

    }
}

// idとJoinHandle<()>を保持するWorker構造体
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        // 空のクロージャを使って新しいスレッドを立ち上げ、生成されるJoinHandle<()>インスタンスを格納
        let thread = thread::spawn(|| {
            receiver;
        });
    }

    Worker {
        id,
        thread,
    }
}
