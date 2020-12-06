use crate::listener::Listener;
use crate::logging::{Log, Logger, LogItem};
use crate::messaging::Message;
use crate::common::NodeId;

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

    pub fn send(&self, data: Vec<u8>) -> () {
        let message = Message::create(data);
    }
}