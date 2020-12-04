use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use crate::logging::{Logger, LogItem};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,    
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    // TODO Add error.
    pub fn new(size: usize, logger: Logger) -> Result<ThreadPool, &'static str> {
        assert!(size > 0);
        match size {
            0 => {
                logger.send(LogItem::error(String::from("thread_pool"), String::from("Invalid pool size, initialization cancelled.")));
                logger.send(LogItem::debug(String::from("thread_pool"), String::from("Check thread pool size is greater than 0 and `std::usize::MAX`")));

                Err("Thread pool size needs to be greater than 0")
            },
            _ => {
                
                let (sender, receiver) = mpsc::channel();
                
                let receiver = Arc::new(Mutex::new(receiver));
                
                let mut workers = Vec::with_capacity(size);
                
                for i in 0..size {
                    workers.push(Worker::new(i, Arc::clone(&receiver), logger.clone()));
                }

                Ok(ThreadPool { workers, sender })
            },
        }
    }
    
    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>, logger: Logger) -> Worker {
        
        let thread = thread::spawn(move || loop {
            
            let job = receiver.lock().unwrap().recv().unwrap();
            
            
            
            logger.send(LogItem::info(format!("thread_{}", id), String::from("Job received")));
            
            // println! ("Worker {} got a job; executing.", id);
            job();
        });
        
        Worker { id, thread }
    }
}