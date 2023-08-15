pub struct ThreadPool;

impl ThreadPool {
    /// 创建线程池
    ///
    /// 线程池中线程的数量
    /// # Panics
    ///
    /// `new`函数会在size为0时触发panic。
    ///
    pub fn new(size: u8) -> ThreadPool {
        assert!(size > 0);
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {}
}