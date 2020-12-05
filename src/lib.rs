use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use crate::logging::{Log, Logger, LogItem};
use crate::common::RequestSettings;
use crate::messaging::*;

pub mod common;
pub mod logging;
pub mod threads;
pub mod listener;
pub mod node;
pub mod messaging;


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
    logger.send(LogItem::debug(String::from("Listener"), format!("Request: : {}", String::from_utf8_lossy(&buffer[..]))));

    // Attempted to handled the handshake
    let handshake_result = handle_handshake(&stream);
    
    match handshake_result {
        Ok(settings) => {
            let result = handle_request(&stream, settings);
            
            // TOOD handle the result.
        }
        Err (e) => {
            
        }
    }
    
    stream.read(&mut buffer).unwrap();
}

fn handle_handshake(mut stream: &TcpStream) -> Result<RequestSettings, &'static str> {
    let mut handshake_buffer: [u8; 32] = [0; 32];

    let read_result = stream.read(&mut handshake_buffer);

    match read_result {
        Ok(_) => {
            let header = messaging::HandshakeHeader::create(handshake_buffer);

            let response = header.create_response();
            let settings = header.create_settings();
            
            let write_result = stream.write(&response.to_bytes());
            
            match write_result {
                Ok(_) => Ok(settings),
                Err(_) => Err("Could not write handshake response")
            }
        }
        Err(_) => Err("Could not read handshake header")
        
    }
    
}

fn handle_request(mut stream: &TcpStream, settings: RequestSettings) -> Result<MessageResult, &'static str> {
    let buffer = [0; 1024];
    
    let mut msg_result = MessageResult::create(settings.correlation_id, settings.frame_count as usize);
    
    for i in 0..settings.frame_count {
        let frame = read_frame(stream, buffer);
        
        let result = match handle_frame(&frame) {
            Ok(_) => {
                FrameResult::create_success(frame.header.frame_number, frame) 
            }
            Err(e) => {
                FrameResult::create_error(frame.header.frame_number, e)
            }
        };
        
        msg_result.add_result(result);
    }
    
    Ok (msg_result)
}

fn read_frame(mut stream: &TcpStream, mut buffer: [u8; 1024]) -> Frame {
    stream.read(&mut buffer);

    Frame::create(buffer)
}

fn handle_frame(frame: &Frame) -> Result<(), &'static str> {
    Err("Not implemented")
}