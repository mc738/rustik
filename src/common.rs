use core::fmt::Write;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct CorrelationId {
    raw: [u8; 6],
}

#[derive(Copy, Clone)]
pub struct NodeId {
    raw: [u8; 4],
}

pub struct RequestSettings {
    pub frame_size: u16,
    pub frame_count: u16,
    pub correlation_id: CorrelationId,
    pub from: NodeId,
}

impl CorrelationId {
    pub fn create(data: [u8; 6]) -> CorrelationId {
        CorrelationId { raw: data }
    }

    pub fn new() -> CorrelationId {
        let random_bytes = rand::thread_rng().gen::<[u8; 6]>();
        CorrelationId::create(random_bytes)
    }

    pub fn to_bytes(&self) -> [u8; 6] {
        self.raw
    }
    
    pub fn to_string(&self) -> String {
        covert_bytes_to_hex(self.raw.to_vec())
    }   
}

impl NodeId {
    pub fn create(data: [u8; 4]) -> NodeId {
        NodeId { raw: data }
    }

    pub fn new() -> NodeId {
        let random_bytes = rand::thread_rng().gen::<[u8; 4]>();
        NodeId::create(random_bytes)
    }

    pub fn to_bytes(&self) -> [u8; 4] { self.raw }

    pub fn to_string(&self) -> String {
        covert_bytes_to_hex(self.raw.to_vec())
    }
}

pub fn create_u16(data: [u8; 2]) -> u16 {
    ((data[0] as u16) << 8) | data[1] as u16
}

impl RequestSettings {
    pub fn get_size(&self) -> u32 {
        (self.frame_size * self.frame_count) as u32
    }
}


pub fn covert_bytes_to_hex(data: Vec<u8>) -> String {
    let mut s = String::with_capacity(2 * data.len());

    for byte in data {
        write!(s, "{:02x}", byte);
    }

    s
}