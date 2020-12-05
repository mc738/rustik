use crate::common::*;

pub struct HandshakeHeader {
    from: NodeId,
    frame_size: u16,
    frame_count: u16,
    correlation_id: CorrelationId,
    flags: [u8; 2],
    // check_sum: [u8; 16],
}

pub struct HandshakeResponseHeader {
    correlation_id: CorrelationId,
    flags: [u8; 2],
}

pub struct Frame {
    pub header: FrameHeader,
    data: [u8; 1016] // While fixed sized framesize/buffer is used this can be set to buffer size - header size or 1024 - 8.
}

pub struct FrameHeader {
    correlation_id: CorrelationId,
    frame_number: u16,
}

pub struct FrameResponseHeader {
    correlation_id: CorrelationId,
    frame_number: u16,
}

impl HandshakeHeader {
    pub fn create(data: [u8; 32]) -> HandshakeHeader {

        let from = NodeId::create([data[0], data[1], data[2], data[3]]);

        let frame_size = create_u16([data[4], data[5]]);

        let frame_count = create_u16([data[6], data[7]]);

        let correlation_id = CorrelationId::create([data[8],data[9],data[10],data[11],data[12],data[13]]);

        let flags = [data[14], data[15]];

        HandshakeHeader {
            from,
            frame_size,
            frame_count,
            correlation_id,
            flags,
        }
    }

    pub fn create_response(&self) -> HandshakeResponseHeader {
        HandshakeResponseHeader {
            correlation_id: self.correlation_id.clone(),
            flags: self.flags,
        }
    }

    pub fn create_settings(&self) -> RequestSettings {
        RequestSettings { frame_size: self.frame_size, frame_count: self.frame_count }
    }
}

impl HandshakeResponseHeader {
    pub fn create(data: [u8; 8]) -> HandshakeResponseHeader {
        let correlation_id = CorrelationId::create([data[0],data[1],data[2],data[3],data[4],data[5]]);
        let flags = [data[6],data[7]];

        HandshakeResponseHeader {
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
    
    /// Create a frame from a buffer.
    /// 
    /// ## Arguments
    /// 
    /// * `data` - A fixed sized byte array.
    /// 
    /// ## Notes
    /// 
    /// * Currently framesize is fixed at 1024. That is why a fixed size array is used.
    pub fn create(data: [u8; 1024]) -> Frame {
        // Take the first 8 bytes 
        let mut header_buffer: [u8; 8] = [0; 8];
        let mut data_buffer: [u8; 1016] = [0; 1016];
        
        for i in 0..7 {
          header_buffer[i] = data[i];      
        };
        
        let header = FrameHeader::create(header_buffer);
        
        for i in 8..1023 {
            data_buffer[i] = data[i];
        }
        
        Frame { header, data: data_buffer }
    }
}

impl FrameHeader {
    pub fn create(data: [u8; 8]) -> FrameHeader {
        let correlation_id = CorrelationId::create([data[0],data[1],data[2],data[3],data[4],data[5]]);
        let frame_number = create_u16([data[6],data[7]]);

        FrameHeader {
            correlation_id,
            frame_number
        }
    }
}

impl FrameResponseHeader {
    pub fn create(data: [u8; 8]) -> FrameResponseHeader {
        let correlation_id = CorrelationId::create([data[0],data[1],data[2],data[3],data[4],data[5]]);
        let frame_number = create_u16([data[6],data[7]]);

        FrameResponseHeader {
            correlation_id,
            frame_number
        }
    }
}