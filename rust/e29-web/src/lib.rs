use std::thread;

struct Worker {
    thread: thread::JoinHandle<()>,
    id: usize,
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker {
            thread,
            id: id + 1,
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// 创建线程池
    ///
    /// 线程池中线程的数量
    /// # Panics
    ///
    /// `new`函数会在size为0时触发panic。
    ///
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // 创建线程并将它们存储到动态数组
            workers.push(Worker::new(id + 1));
        }

        ThreadPool {
            workers
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {}
}