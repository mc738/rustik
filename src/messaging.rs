use crypto::md5::Md5;
use crypto::digest::Digest;
use crate::common::*;
use std::slice::from_mut;
use std::convert::TryFrom;
use std::process::id;

pub struct Message {
    pub correlation_id: CorrelationId,
    check_sum: [u8; 16],
    pub data: Vec<u8>,
}

pub struct HandshakeHeader {
    from: NodeId,
    frame_size: u16,
    frame_count: u16,
    correlation_id: CorrelationId,
    flags: [u8; 2],
    // check_sum: [u8; 16],
}

pub struct HandshakeResponseHeader {
    pub correlation_id: CorrelationId,
    pub flags: [u8; 2],
}

pub struct Frame {
    pub header: FrameHeader,
    data: [u8; 1016], // While fixed sized framesize/buffer is used this can be set to buffer size - header size or 1024 - 8.
}

pub struct FrameHeader {
    pub correlation_id: CorrelationId,
    pub frame_number: u16,
}

pub struct FrameResponseHeader {
    correlation_id: CorrelationId,
    frame_number: u16,
}

pub struct MessageResult {
    pub correlation_id: CorrelationId,
    pub frame_results: Vec<FrameResult>,
}

pub enum FrameResultType {
    Success(Frame),
    Error(&'static str),
}

pub struct FrameResult {
    pub number: u16,
    pub result: FrameResultType,
}

impl Message {
    pub fn create(data: Vec<u8>) -> Message {

        let mut hasher = Md5::new();
        
        hasher.input(&data);
        
        let mut check_sum =  [0; 16];
        
        hasher.result(&mut check_sum);
        
        Message {
            correlation_id: CorrelationId::new(),
            check_sum,
            data,
        }
    }
    
    pub fn create_frames(&mut self) -> Vec<Frame> {
        // 
        
        let size = self.data.len() as i32;
        
        let rem = size % 1016;
        
        let mut frame_count = size / 1016;
        
        // If there is a remainder ad one to the frame count and add set padding.
        
        let mut padding = 0;
        
        if rem > 0 { 
            frame_count = frame_count + 1;
            padding = 1016 - rem
        };
        
        let header_size = frame_count * 8; 
        
        let total_size = (frame_count * 1016) + header_size;
        
        let mut frame_no = 1;

        let mut frames = Vec::with_capacity(frame_count as usize);

        println!("{}", self.data.len());
        
        // Bit ugly...but works
        // Add padding to the data, if not padded, it will fail later on.
        for i in 0..padding {
            self.data.push(0);
        }
        
        println!("{}", self.data.len());

        println!("Frame count: {}, Padding: {}, Total size: {}, Header size: {}", frame_count, padding, total_size, header_size);
        
        for chunk in self.data.chunks(1016) {
            let frame = Frame {
                header: FrameHeader::generate(self.correlation_id.clone(), frame_no),
                data: <[u8; 1016]>::try_from(chunk).unwrap(),
            };
            
            frames.push(frame);
            
            frame_no = frame_no + 1;
        }
        
        
        
        return frames;
    }
}

impl HandshakeHeader {
    pub fn create(data: [u8; 32]) -> HandshakeHeader {
        let from = NodeId::create([data[0], data[1], data[2], data[3]]);

        let frame_size = create_u16([data[4], data[5]]);

        let frame_count = create_u16([data[6], data[7]]);

        let correlation_id = CorrelationId::create([data[8], data[9], data[10], data[11], data[12], data[13]]);

        let flags = [data[14], data[15]];
        
        HandshakeHeader {
            from,
            frame_size,
            frame_count,
            correlation_id,
            flags,
        }
    }

    pub fn generate(from: NodeId, frame_size: u16, frame_count: u16, correlation_id: CorrelationId, flags: [u8; 2]) -> HandshakeHeader {
        HandshakeHeader {
            from,
            frame_size,
            frame_count,
            correlation_id,
            flags
        }
    }
    
    pub fn create_response(&self) -> HandshakeResponseHeader {
        HandshakeResponseHeader {
            correlation_id: self.correlation_id.clone(),
            flags: self.flags,
        }
    }

    pub fn create_settings(&self) -> RequestSettings {
        RequestSettings {
            frame_size: self.frame_size,
            frame_count: self.frame_count,
            correlation_id: self.correlation_id.clone(),
            from: self.from.clone(),
        }
    }
    
    pub fn to_bytes(&self) -> [u8; 16] {
        let mut buffer: [u8; 16] = [0;16];
        
        let id_bytes = self.from.to_bytes();
        let frame_size_bytes = self.frame_size.to_be_bytes();
        let frame_count_bytes = self.frame_count.to_be_bytes();
        
        let cor_id_byte = self.correlation_id.to_bytes();

        buffer[0] = id_bytes[0];
        buffer[1] = id_bytes[1];
        buffer[2] = id_bytes[2];
        buffer[3] = id_bytes[3];

        buffer[4] = frame_size_bytes[0];
        buffer[5] = frame_size_bytes[1];

        buffer[6] = frame_count_bytes[0];
        buffer[7] = frame_count_bytes[1];

        buffer[8] = cor_id_byte[0];
        buffer[9] = cor_id_byte[1];
        buffer[10] = cor_id_byte[2];
        buffer[11] = cor_id_byte[3];
        buffer[12] = cor_id_byte[4];
        buffer[13] = cor_id_byte[5];

        buffer[14] = self.flags[0];
        buffer[15] = self.flags[1];
        
        buffer
    }
}

impl HandshakeResponseHeader {
    pub fn create(data: [u8; 8]) -> HandshakeResponseHeader {
        let correlation_id = CorrelationId::create([data[0], data[1], data[2], data[3], data[4], data[5]]);
        let flags = [data[6], data[7]];

        HandshakeResponseHeader {
            correlation_id,
            flags,
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
            data_buffer[i - 8] = data[i];
        }

        Frame { header, data: data_buffer }
    }
    
    pub fn to_bytes(&self) -> [u8; 1024] {
        let mut buffer: [u8; 1024] = [0; 1024];
        
        let header_bytes = self.header.to_bytes();
        
        for i in 0..7 {
            buffer[i] = header_bytes[i];
        };
        
        for i in 8..1023 {
            buffer[i] = self.data[i - 8];
        };
        
        buffer
    }
}

impl FrameHeader {
    pub fn create(data: [u8; 8]) -> FrameHeader {
        let correlation_id = CorrelationId::create([data[0], data[1], data[2], data[3], data[4], data[5]]);
        let frame_number = create_u16([data[6], data[7]]);

        FrameHeader {
            correlation_id,
            frame_number,
        }
    }
    
    pub fn generate(correlation_id : CorrelationId, frame_number: u16) -> FrameHeader {
        FrameHeader {
            correlation_id,
            frame_number,
        }
    }
    
    pub fn to_bytes(&self) -> [u8; 8] {
        
        let mut buffer: [u8;8] = [0; 8];
        
        let l = self.frame_number.to_be_bytes();
        
        let cor_id = self.correlation_id.to_bytes();
        
        for i in 0..5 {
            buffer[i] = cor_id[i];
        };
        
        buffer[6] = l[0];
        buffer[7] = l[1];
        
        buffer
    }
}

impl FrameResponseHeader {
    pub fn create(data: [u8; 8]) -> FrameResponseHeader {
        let correlation_id = CorrelationId::create([data[0], data[1], data[2], data[3], data[4], data[5]]);
        let frame_number = create_u16([data[6], data[7]]);

        FrameResponseHeader {
            correlation_id,
            frame_number,
        }
    }
}

impl MessageResult {
    pub fn create(cor_id: CorrelationId, frame_count: usize) -> MessageResult {
        MessageResult {
            correlation_id: cor_id,
            frame_results: Vec::with_capacity(frame_count),
        }
    }

    pub fn add_result(&mut self, result: FrameResult) {
        self.frame_results.push(result);
    }
    
    pub fn get_data(&self) -> Vec<u8> {
        
        let mut data:Vec<u8> = Vec::new();

        // TODO Handle errors, this currently assumes all frames where received successfully.
        for i in &self.frame_results {
            match &i.result {
                FrameResultType::Success(f) => {
                    data.append(&mut f.data.to_vec());
                }
                FrameResultType::Error(e) => {
                    
                }
            }
        }
        
        data
    }
}

impl FrameResult {
    pub fn create_success(number: u16, frame: Frame) -> FrameResult {
        FrameResult {
            number,
            result: FrameResultType::Success(frame),
        }
    }

    pub fn create_error(number: u16, error: &'static str) -> FrameResult {
        FrameResult {
            number,
            result: FrameResultType::Error(error),
        }
    }
}