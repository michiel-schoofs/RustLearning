use std::thread::JoinHandle;

pub struct ThreadPool{
    _handles: Vec<std::thread::JoinHandle<()>>
}

impl ThreadPool{
    fn new(num_threads: u8) -> Self {
        let _handles: Vec<JoinHandle<()>> = (0..num_threads).map(|_|{
           std::thread::spawn(||{});
        }).collect();

        Self{_handles}
    }

    fn execute<T: Fn()>(&self, function:T){
        function();
    }
}


#[cfg(test)]
mod tests {
    use crate::ThreadPool;

    #[test]
    fn it_works() {
        let threadPool:ThreadPool = ThreadPool::new();
        threadPool.execute(||{println!("Hello from thread1");});
        threadPool.execute(||{println!("Hello from thread2");});
    }
}
