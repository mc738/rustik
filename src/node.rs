use crate::listener::Listener;
use crate::logging::{Log, Logger};
use crate::messaging::Message;

pub struct Node {
    listener: Listener,
    log: Log,
    logger: Logger, 
}


impl Node {
    
    pub fn init() -> () {
        
    }
    
    
    pub fn send(&self, data: Vec<u8>) -> () {
        let message = Message::create(data);
    }
    
}