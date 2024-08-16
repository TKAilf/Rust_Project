pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        /// 新しいThreadPoolを生成する。
        /// sizeはプールのスレッド数です。
        /// #パニック
        /// sizeが0の場合、'new'関数はパニックします。
        assert!(size > 0);

        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
