use std::net::TcpListener;
use crate::threads::ThreadPool;
use crate::logging::{Logger, LogItem, Log};

pub struct Listener {
    thread_pool: ThreadPool,
    listener: TcpListener,
}


impl Listener {
    pub fn new(ipAddress: &str, thread_pool_size: usize, logger: Logger) -> Result<Listener, &'static str> {
        let thread_pool = ThreadPool::new(thread_pool_size, logger);
        
        match thread_pool {
            Ok(thread_pool) => {
                
                let listener = TcpListener::bind(ipAddress).unwrap();
                
                Ok (Listener { thread_pool, listener })
            }
            Err(e) => Err(e)
        }
    }
}
