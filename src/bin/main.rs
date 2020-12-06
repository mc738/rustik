use rand::Rng;
use rustik::{listen, test_framing};
use rustik::logging::Log;
use rustik::common::covert_bytes_to_hex;
use rustik::messaging::Message;
use rustik::node::Node;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::thread::Thread;
use std::thread;

fn main() {
    
    
    //let frames = test_framing("/home/max/Data/rustik_test/test_file");
    
    match (Node::init("127.0.0.1:4200"), Node::init("127.0.0.1:4201")) {
        (Ok(node1), Ok(node2)) => {
            
            thread::spawn( move || {
                node1.listen();
            });
            
            node2.send("127.0.0.1:4200",load_test_data("/home/max/Data/rustik_test/test_file"));
            
            loop {}
        } ,
        _ => {}
    };
    
    //let random_bytes = rand::thread_rng().gen::<[u8; 4]>();
    
    //let s = covert_bytes_to_hex(random_bytes.to_vec());
    //println!("Test: {}", s);
    
    //listen("127.0.0.1:7878");
    
    //println!("Hello, world!");
}

pub fn load_test_data(path: &'static str) -> Vec<u8> {
    let path = Path::new(path);

    let mut data: Vec<u8> = Vec::new();

    match File::open(path) {
        Err(_) => panic!("Could not create file."),
        Ok(mut file) => {
            match file.read_to_end(&mut data) {
                Err(_) => panic!("Could not read file"),
                Ok(_) => {
                    data
                }
            }
        }
    }
}

