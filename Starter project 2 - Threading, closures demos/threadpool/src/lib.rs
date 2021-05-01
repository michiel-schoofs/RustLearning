pub struct ThreadPool{}

impl ThreadPool{
    fn new() -> Self {
        Self
    }

    fn execute(&self) -> (){}
}


#[cfg(test)]
mod tests {
    use crate::ThreadPool;

    #[test]
    fn it_works() {
        let threadPool:ThreadPool = ThreadPool::new();
        threadPool.execute();
    }
}
