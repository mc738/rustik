use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use crate::logging::{Log, Logger, LogItem};

pub mod logging;
pub mod threads;
pub mod listener;


pub fn listen(ip_address: &str) {

    let (log, logger) = Log::create();
    
    let pool = threads::ThreadPool::new(4, logger.clone());

    let listener = TcpListener::bind(ip_address);

    let thread_logger = logger.clone();
    
    match (pool, listener) {
        (Ok(pool), Ok(listener)) => {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                
                let l = thread_logger.clone();
                
                pool.execute(|| {
                    handle_connection(stream, l);
                })
            }
        },
        (Err(e1), Err(e2)) => {
            // Both thread pool and listener could failed.
            println!("e1: {}, e2: {}", e1, e2);
        }
        (Err(e), _) => {
            //Thread pool failed.
            println!("e: {}", e);
        }
        (_, Err(e)) => {
            // Listener failed.
            println!("e: {}", e);
        }
        
    }
}

fn handle_connection(mut stream: TcpStream, logger: Logger) {
    let mut buffer = [0; 1024];


    stream.read(&mut buffer).unwrap();

    logger.send(LogItem::debug(String::from("Listener"), format!("Request: : {}", String::from_utf8_lossy(&buffer[..]))));
}




