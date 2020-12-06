use crate::listener::Listener;
use crate::logging::{Log, Logger, LogItem};
use crate::messaging::{Message, HandshakeHeader, HandshakeResponseHeader};
use crate::common::NodeId;
use std::net::TcpStream;
use std::io::{Write, Read};

pub struct Node {
    id: NodeId,
    listener: Listener,
    log: Log,
    logger: Logger,
}

impl Node {
    pub fn init(ipAddress: &'static str) -> Result<Node, &'static str> {
        
        let id = NodeId::new();
        
        let name = format!("node_{}", id.to_string());
        
        let (log, logger) = Log::create();

        logger.send(LogItem::info(name.clone(), String::from("Node initialization started")));


        logger.send(LogItem::success(name.clone(), String::from("Logger started")));

        let listener = Listener::new(ipAddress, 4, logger.clone());

        match listener {
            Ok(listener) => {
                logger.send(LogItem::success(name.clone(), format!("Listener successfully created. Bound to '{}'", ipAddress)));
                logger.send(LogItem::success(name.clone(), String::from("Node initialization successful")));
                Ok(Node {
                    id,
                    listener,
                    log,
                    logger,
                })
            }
            Err(e) => {
                logger.send(LogItem::error(name.clone(), format!("Listener failed to start, error: '{}'", e)));
                Err(e)
            }
        }
    }

    pub fn listen(&self) {
        self.logger.send(LogItem::info(format!("node_{}", self.id.to_string()), String::from("Listener started")));
        self.listener.listen()
    }

    pub fn send(&self, ip_address: &str, data: Vec<u8>) -> () {

        let con_result = TcpStream::connect(ip_address);
        
        match con_result {
            Ok(mut stream) => {
                let message = Message::create(data);
                
                let handshake = HandshakeHeader::generate(self.id, 1024, message.data.len() as u16, message.correlation_id, [0; 2]);

                self.logger.send(LogItem::info(format!("node_{}", self.id.to_string()), format!("Message created, correlation id: '{}'", message.correlation_id.to_string())));
                self.logger.send(LogItem::info(format!("node_{}", self.id.to_string()), format!("Sending handshake")));
                
                stream.write(&handshake.to_bytes());
                
                // Read the response
                
                let mut buffer: [u8; 8] = [0; 8];
                
                stream.read(&mut buffer);
                
                let handshake_response = HandshakeResponseHeader::create(buffer);
                
                self.logger.send(LogItem::info(format!("node_{}", self.id.to_string()), format!("Handshake response received, correlation id: '{}'", handshake_response.correlation_id.to_string())));


                //for i in message.data {
                    
                //}
            }
            Err(e) => {}
        }
        
        
        
    }
}