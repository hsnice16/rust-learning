pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
