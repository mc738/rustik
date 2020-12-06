use std::net::{TcpStream, TcpListener};
use crate::threads::ThreadPool;
use crate::logging::{Logger, LogItem, Log};
use crate::common::{RequestSettings, CorrelationId};
use crate::messaging::{MessageResult, Frame, FrameResult};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use crate::messaging;

pub struct Listener {
    thread_pool: ThreadPool,
    listener: TcpListener,
    logger: Logger,
}


impl Listener {
    pub fn new(ipAddress: &str, thread_pool_size: usize, logger: Logger) -> Result<Listener, &'static str> {
        let thread_pool = ThreadPool::new(thread_pool_size, logger);
        
        match thread_pool {
            Ok(thread_pool) => {
                let (log, logger) = Log::create();
                
                let listener = TcpListener::bind(ipAddress);
                
                match listener {
                    Ok (listener) => Ok (Listener { thread_pool, listener, logger }),
                    Err(e) => Err("Could not bind listener"),
                }
            }
            Err(e) => Err(e)
        }
    }

    pub fn listen(&self) {
        
        let thread_logger = self.logger.clone();

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();

            let l = thread_logger.clone();

            self.thread_pool.execute(|| {
                Listener::handle_connection(stream, l);
            });
        }
    }

    fn handle_connection(mut stream: TcpStream, logger: Logger) {
        let mut buffer = [0; 1024];
        logger.send(LogItem::debug(String::from("Listener"), format!("Request: : {}", String::from_utf8_lossy(&buffer[..]))));

        // Attempted to handled the handshake
        let handshake_result = Listener::handle_handshake(&stream);

        match handshake_result {
            Ok(settings) => {
                let result =  Listener::handle_request(&stream, &settings);

                logger.send(LogItem::success(String::from("Listener"), format!("Message received, correlation id: '{}'", settings.correlation_id.to_string())));
                // TODO handle error
                match result {
                    Ok(message) => {
                        let data = message.get_data();
                
                        Listener::handle_data(settings.correlation_id.clone(), data);
                
                        let message =
                            format!("Message received from `{}`. Correlation ID: {}, Frames: {}",
                                    settings.from.to_string(),
                                    settings.correlation_id.to_string(),
                                    settings.frame_count);
                
                        logger.send(LogItem::info(String::from("connection_handler"), message));
                    }
                    Err(e) => {}
                }
            }
            Err(e) => {}
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

    fn handle_request(mut stream: &TcpStream, settings: &RequestSettings) -> Result<MessageResult, &'static str> {
        let buffer = [0; 1024];

        let mut msg_result = MessageResult::create(settings.correlation_id.clone(), settings.frame_count as usize);
        
        //println!("******** Frame count: {}", settings.frame_count);
        
        for i in 1..=settings.frame_count {
            
            
            let frame = Listener::read_frame(stream, buffer);

            
            
            let result = match Listener::handle_frame(&frame) {
                Ok(_) => {
                    // TODO switch to logger.
                    println!("Receiving frame {}", i);
                    FrameResult::create_success(frame.header.frame_number, frame)
                }
                Err(e) => {
                    FrameResult::create_error(frame.header.frame_number, e)
                }
            };

            msg_result.add_result(result);
        }

        Ok(msg_result)
    }

    fn read_frame(mut stream: &TcpStream, mut buffer: [u8; 1024]) -> Frame {
        stream.read(&mut buffer);

        Frame::create(buffer)
    }

    fn handle_frame(frame: &Frame) -> Result<(), &'static str> {
        Ok(())
    }

    fn handle_data(cor_id: CorrelationId, data: Vec<u8>) {
        // For now just save the data to a fixed location.
        let location = "/home/max/Data/rustik_test";

        let cor_id = cor_id.to_string();
        
        let p = format!("{}.txt", cor_id);
        
        let path = Path::new(&p);

        let mut file = match File::create(&path) {
            Err(_) => panic!("Could not create file."),
            Ok(file) => file,
        };

        match file.write_all(&data) {
            Err(_) => panic!("Could not save to file"),
            Ok(_) => (),
        }
    }
}
