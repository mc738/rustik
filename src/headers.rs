use crate::common::{NodeId, CorrelationId, create_u16,RequestSettings};
use std::borrow::Borrow;

pub struct Handshake {
    from: NodeId,
    frame_size: u16,
    frame_count: u16,
    correlation_id: CorrelationId,
    flags: [u8; 2],
    // check_sum: [u8; 16],
}

pub struct HandshakeResponse {
    correlation_id: CorrelationId,
    flags: [u8; 2],
}

pub struct Frame {
    correlation_id: CorrelationId,
    frame_number: u16,
}

pub struct FrameResponse {
    correlation_id: CorrelationId,
    frame_number: u16,
}

impl Handshake {
    pub fn create(data: [u8; 32]) -> Handshake {
        
        let from = NodeId::create([data[0], data[1], data[2], data[3]]);
        
        let frame_size = create_u16([data[4], data[5]]);
        
        let frame_count = create_u16([data[6], data[7]]);
        
        let correlation_id = CorrelationId::create([data[8],data[9],data[10],data[11],data[12],data[13]]);
        
        let flags = [data[14], data[15]];
       
        Handshake {
            from,
            frame_size,
            frame_count,
            correlation_id,
            flags,
        }
    }
    
    pub fn create_response(&self) -> HandshakeResponse {
       HandshakeResponse {
           correlation_id: self.correlation_id.clone(),
           flags: self.flags,
       }
    }
    
    pub fn create_settings(&self) -> RequestSettings {
        RequestSettings { frame_size: self.frame_size, frame_count: self.frame_count }
    }
}

impl HandshakeResponse {
    pub fn create(data: [u8; 8]) -> HandshakeResponse {
        let correlation_id = CorrelationId::create([data[0],data[1],data[2],data[3],data[4],data[5]]);
        let flags = [data[6],data[7]];

        HandshakeResponse {
            correlation_id,
            flags
        }
    }
    
    pub fn to_bytes(&self) -> [u8; 8] {
        
        let cor_id = self.correlation_id.to_bytes();

        let mut result: [u8; 8] = [0; 8];
        
        for i in 0..5 {
            result[i] = cor_id[i]
        }
        
        result[6] = self.flags[0];
        result[7] = self.flags[1];
            
        result
    }
}

impl Frame {
    pub fn create(data: [u8; 8]) -> Frame {
        let correlation_id = CorrelationId::create([data[0],data[1],data[2],data[3],data[4],data[5]]);
        let frame_number = create_u16([data[6],data[7]]);

        Frame {
            correlation_id,
            frame_number
        }
    }
}

impl FrameResponse {
    pub fn create(data: [u8; 8]) -> FrameResponse {
        let correlation_id = CorrelationId::create([data[0],data[1],data[2],data[3],data[4],data[5]]);
        let frame_number = create_u16([data[6],data[7]]);
        
        FrameResponse {
            correlation_id,
            frame_number
        }
    }
}