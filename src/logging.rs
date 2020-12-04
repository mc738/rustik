
use std::thread;
use std::sync::mpsc;
use std::thread::JoinHandle;

pub type Logger = mpsc::Sender<LogItem>;

pub struct Log {
    handler: JoinHandle<()>
}

pub struct LogItem {
    from: String,
    message: String,
    item_type: LogItemType
}

enum LogItemType {
    Information,
    Success,
    Error,
    Warning,
    Debug,
}

impl Log {
    pub fn create() -> (Log, Logger) {
        
        let (sender, receiver) = mpsc::channel();
        
        let logger:Logger = sender;
        
        let handler = thread::spawn(move || loop {
            let item = receiver.recv().unwrap();
            
            println!("{} {}", item.from, item.message);
        });
        
        let log = Log { handler };
        
        (log, logger)
        
    }
}

impl LogItem {
    
    pub fn info(from: String, message: String) -> LogItem {
        create_item(from, message, LogItemType::Information)
    }
    
    pub fn success(from: String, message: String) -> LogItem {
        create_item(from, message, LogItemType::Success)
    }
    
    pub fn error(from: String, message: String) -> LogItem {
        create_item(from, message, LogItemType::Error)
    }
    
    pub fn warning(from: String, message: String) -> LogItem {
        create_item(from, message, LogItemType::Warning)
    }
    
    pub fn debug(from: String, message: String) -> LogItem {
        create_item(from, message, LogItemType::Debug)
    }
}

fn create_item(from: String, message: String, item_type: LogItemType) -> LogItem {
    LogItem { from, message, item_type }
}