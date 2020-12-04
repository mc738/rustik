use rustik::listen;
use rustik::logging::Log;

fn main() {
    
    
    let (log, logger) = Log::create();
    
    listen("127.0.0.1:7878", logger);
    
    println!("Hello, world!");
}
