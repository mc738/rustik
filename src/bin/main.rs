use rand::Rng;
use rustik::{listen, test_framing};
use rustik::logging::Log;
use rustik::common::covert_bytes_to_hex;
use rustik::messaging::Message;



fn main() {
    
    
    test_framing("/home/max/Data/rustik_test/test_file");
    
    
    
    //let random_bytes = rand::thread_rng().gen::<[u8; 4]>();
    
    //let s = covert_bytes_to_hex(random_bytes.to_vec());
    //println!("Test: {}", s);
    
    //listen("127.0.0.1:7878");
    
    //println!("Hello, world!");
}
